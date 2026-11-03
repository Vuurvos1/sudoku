use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::board::{Board, Candidates, CellId};
use crate::solver::{SolveResult, Solver, SolverContext};
use crate::strategy::Deduction;

/// Difficulty tiers based on the hardest strategy required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,   // max strategy difficulty 1-2 (naked + hidden singles)
    Medium, // requires difficulty 3 (pointing pairs, box/line reduction)
    Hard,   // requires difficulty 4-5 (naked/hidden pairs + triples)
    Expert, // requires difficulty 6-7 (x-wing, swordfish, y-wing)
}

impl Difficulty {
    /// The maximum strategy difficulty allowed for this tier.
    fn ceiling(self) -> u8 {
        match self {
            Difficulty::Easy => 2,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5,
            Difficulty::Expert => 7,
        }
    }

    /// The minimum strategy difficulty required for this tier.
    /// Easy has no floor (any puzzle qualifies).
    fn floor(self) -> u8 {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Medium => 3,
            Difficulty::Hard => 4,
            Difficulty::Expert => 6,
        }
    }
}

pub struct Generator {
    rng: SmallRng,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            rng: SmallRng::from_os_rng(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Generator {
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    /// Fill an empty standard 9x9 grid with randomized digits.
    /// Returns a flat array of 81 digits (1-9).
    pub fn fill_grid(&mut self) -> [u8; 81] {
        self.fill_grid_for(&SolverContext::standard())
    }

    /// Fill an empty grid that satisfies all of `ctx`'s rules.
    pub fn fill_grid_for(&mut self, ctx: &SolverContext) -> [u8; 81] {
        let rng = &mut self.rng;
        let mut solution = None;
        search(ctx, &mut [0u8; 81], &mut |d| d.shuffle(rng), &mut |grid| {
            solution = Some(*grid);
            true
        });
        solution.expect("rules admit no complete grid")
    }

    pub fn generate(&mut self, difficulty: Difficulty) -> Board {
        self.generate_for(difficulty, &SolverContext::standard())
    }

    pub fn generate_for(&mut self, difficulty: Difficulty, ctx: &SolverContext) -> Board {
        let solver = Solver::default();

        loop {
            let grid = self.fill_grid_for(ctx);

            // Build list of all givens (all 81 cells initially)
            let mut givens: Vec<Option<u8>> = grid.iter().copied().map(Some).collect();

            // Shuffle removal order
            let mut order: Vec<usize> = (0..81).collect();
            order.shuffle(&mut self.rng);

            for pos in order {
                let digit = givens[pos].unwrap();
                givens[pos] = None;

                // Collect current givens for checks
                let current: Vec<(CellId, u8)> = givens
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &d)| Some((CellId(i as u8), d?)))
                    .collect();

                // Check uniqueness
                if count_solutions_for(ctx, &current, 2) != 1 {
                    givens[pos] = Some(digit);
                    continue;
                }

                // Check difficulty ceiling
                let mut board = Board::new(&current);
                match solver.solve(&mut board, ctx) {
                    SolveResult::Solved(ref deductions) => {
                        if classify(deductions) > difficulty.ceiling() {
                            givens[pos] = Some(digit);
                        }
                    }
                    _ => {
                        // Solver got stuck or invalid — restore
                        givens[pos] = Some(digit);
                    }
                }
            }

            // Validate difficulty floor
            let final_givens: Vec<(CellId, u8)> = givens
                .iter()
                .enumerate()
                .filter_map(|(i, &d)| Some((CellId(i as u8), d?)))
                .collect();

            let mut board = Board::new(&final_givens);
            if let SolveResult::Solved(ref deductions) = solver.solve(&mut board, ctx) {
                let max_diff = classify(deductions);
                if max_diff >= difficulty.floor() {
                    return Board::new(&final_givens);
                }
            }
            // Floor not met — retry with a new grid
        }
    }
}

/// Depth-first search, always branching on the empty cell with the fewest
/// legal digits. `order` arranges each cell's digits before trying them;
/// `on_solution` sees every complete grid and returns true to stop.
/// Returns true if the search was stopped.
fn search<O, F>(
    ctx: &SolverContext,
    grid: &mut [u8; 81],
    order: &mut O,
    on_solution: &mut F,
) -> bool
where
    O: FnMut(&mut [u8]),
    F: FnMut(&[u8; 81]) -> bool,
{
    let mut best: Option<(usize, Candidates)> = None;
    for i in 0..81 {
        if grid[i] != 0 {
            continue;
        }
        let legal = ctx.legal_digits(grid, CellId(i as u8));
        if legal.count() == 0 {
            return false;
        }
        if best.is_none_or(|(_, b)| legal.count() < b.count()) {
            best = Some((i, legal));
            if legal.count() == 1 {
                break;
            }
        }
    }

    let Some((pos, legal)) = best else {
        return on_solution(grid);
    };

    let mut digits: Vec<u8> = legal.iter().collect();
    order(&mut digits);
    for d in digits {
        grid[pos] = d;
        if search(ctx, grid, order, on_solution) {
            grid[pos] = 0;
            return true;
        }
    }
    grid[pos] = 0;
    false
}

/// Place the givens on an empty grid, or `None` if they break a rule.
fn grid_from_givens(ctx: &SolverContext, givens: &[(CellId, u8)]) -> Option<[u8; 81]> {
    let mut grid = [0u8; 81];
    for &(cell, digit) in givens {
        if !ctx.legal_digits(&grid, cell).contains(digit) {
            return None;
        }
        grid[cell.0 as usize] = digit;
    }
    Some(grid)
}

/// Count solutions of a standard puzzle, stopping once `limit` are found.
pub fn count_solutions(givens: &[(CellId, u8)], limit: usize) -> usize {
    count_solutions_for(&SolverContext::standard(), givens, limit)
}

/// Count solutions under `ctx`'s rules, stopping once `limit` are found.
pub fn count_solutions_for(ctx: &SolverContext, givens: &[(CellId, u8)], limit: usize) -> usize {
    let Some(mut grid) = grid_from_givens(ctx, givens) else {
        return 0;
    };
    let mut count = 0;
    search(ctx, &mut grid, &mut |_| {}, &mut |_| {
        count += 1;
        count >= limit
    });
    count
}

/// First solution under `ctx`'s rules found by brute force, if any.
pub fn solve_brute(ctx: &SolverContext, givens: &[(CellId, u8)]) -> Option<[u8; 81]> {
    let mut grid = grid_from_givens(ctx, givens)?;
    let mut solution = None;
    search(ctx, &mut grid, &mut |_| {}, &mut |g| {
        solution = Some(*g);
        true
    });
    solution
}

/// Extract the max strategy difficulty from deductions, ignoring
/// propagation (difficulty 0).
fn classify(deductions: &[Deduction]) -> u8 {
    deductions
        .iter()
        .map(|d| d.difficulty)
        .filter(|&d| d > 0)
        .max()
        .unwrap_or(0)
}
