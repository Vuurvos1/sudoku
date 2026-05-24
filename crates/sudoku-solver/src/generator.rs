use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::board::{Board, CellId};
use crate::constraint::standard_constraints;
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

    /// Fill an empty 9x9 grid using backtracking with randomized candidates.
    /// Returns a flat array of 81 digits (1-9).
    pub fn fill_grid(&mut self) -> [u8; 81] {
        let mut grid = [0u8; 81];
        fill_recursive(&mut grid, 0, &mut self.rng);
        grid
    }

    pub fn generate(&mut self, difficulty: Difficulty) -> Board {
        let constraints = standard_constraints();
        let ctx = SolverContext {
            constraints: &constraints,
        };
        let solver = Solver::default();

        loop {
            let grid = self.fill_grid();

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
                if count_solutions(&current, 2) != 1 {
                    givens[pos] = Some(digit);
                    continue;
                }

                // Check difficulty ceiling
                let mut board = Board::new(&current);
                match solver.solve(&mut board, &ctx) {
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
            if let SolveResult::Solved(ref deductions) = solver.solve(&mut board, &ctx) {
                let max_diff = classify(deductions);
                if max_diff >= difficulty.floor() {
                    return Board::new(&final_givens);
                }
            }
            // Floor not met — retry with a new grid
        }
    }
}

fn fill_recursive(grid: &mut [u8; 81], pos: usize, rng: &mut SmallRng) -> bool {
    if pos == 81 {
        return true;
    }

    let mut digits: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    digits.shuffle(rng);

    for d in digits {
        if is_valid_placement(grid, pos, d) {
            grid[pos] = d;
            if fill_recursive(grid, pos + 1, rng) {
                return true;
            }
            grid[pos] = 0;
        }
    }

    false
}

fn is_valid_placement(grid: &[u8; 81], pos: usize, digit: u8) -> bool {
    let row = pos / 9;
    let col = pos % 9;

    // Check row
    for c in 0..9 {
        if grid[row * 9 + c] == digit {
            return false;
        }
    }

    // Check column
    for r in 0..9 {
        if grid[r * 9 + col] == digit {
            return false;
        }
    }

    // Check 3x3 box
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if grid[r * 9 + c] == digit {
                return false;
            }
        }
    }

    true
}

/// Count solutions for a puzzle up to `limit` using backtracking.
/// Returns as soon as `limit` solutions are found.
pub fn count_solutions(givens: &[(CellId, u8)], limit: usize) -> usize {
    let mut grid = [0u8; 81];
    for &(cell, digit) in givens {
        grid[cell.0 as usize] = digit;
    }
    let mut count = 0;
    count_recursive(&mut grid, 0, &mut count, limit);
    count
}

fn count_recursive(grid: &mut [u8; 81], pos: usize, count: &mut usize, limit: usize) {
    if *count >= limit {
        return;
    }
    if pos == 81 {
        *count += 1;
        return;
    }
    if grid[pos] != 0 {
        count_recursive(grid, pos + 1, count, limit);
        return;
    }

    for d in 1..=9 {
        if is_valid_placement(grid, pos, d) {
            grid[pos] = d;
            count_recursive(grid, pos + 1, count, limit);
            grid[pos] = 0;
            if *count >= limit {
                return;
            }
        }
    }
}

/// Extract the max strategy difficulty from deductions, ignoring
/// constraint propagation (difficulty 0).
fn classify(deductions: &[Deduction]) -> u8 {
    deductions
        .iter()
        .map(|d| d.difficulty)
        .filter(|&d| d > 0)
        .max()
        .unwrap_or(0)
}
