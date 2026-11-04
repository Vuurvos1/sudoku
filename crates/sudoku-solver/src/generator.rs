use std::cmp::Reverse;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread;

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

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

    /// How many puzzles that reach this tier to pick the best one from. Easy
    /// and medium puzzles are much alike, harder ones vary more.
    fn pool_size(self) -> usize {
        match self {
            Difficulty::Easy | Difficulty::Medium => 4,
            Difficulty::Hard | Difficulty::Expert => 8,
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
        first_solution(ctx, &Partial::empty(), &mut |d| d.shuffle(rng))
            .expect("rules admit no complete grid")
    }

    pub fn generate(&mut self, difficulty: Difficulty) -> Board {
        self.generate_for(difficulty, &SolverContext::standard())
    }

    /// A puzzle of the given difficulty: the best of a few that reach it.
    ///
    /// Each attempt is numbered and seeded from this generator, and the pool
    /// is the lowest-numbered successes, so a seeded generator gives the same
    /// puzzle however the threads are scheduled.
    pub fn generate_for(&mut self, difficulty: Difficulty, ctx: &SolverContext) -> Board {
        let seed: u64 = self.rng.random();
        let pool = first_successes(difficulty.pool_size(), |n, cancelled| {
            Generator::seeded(seed.wrapping_add(n)).attempt(difficulty, ctx, cancelled)
        });

        // the first of the highest scores
        let best = pool
            .into_iter()
            .min_by_key(|puzzle| Reverse(puzzle.score))
            .expect("the pool is never empty");
        Board::new(&best.givens)
    }

    /// One try: fill a grid, then take digits away in random order while the
    /// puzzle stays unique and within the difficulty's ceiling. `None` if it
    /// misses the floor, or once `cancelled` says so.
    fn attempt(
        &mut self,
        difficulty: Difficulty,
        ctx: &SolverContext,
        cancelled: impl Fn() -> bool,
    ) -> Option<ScoredPuzzle> {
        let solver = Solver::default();
        let mut givens: Vec<Option<u8>> = self.fill_grid_for(ctx).map(Some).to_vec();
        let mut order: Vec<usize> = (0..81).collect();
        order.shuffle(&mut self.rng);
        // the solver's steps for the puzzle as it stands after the last removal
        let mut steps = None;

        for pos in order {
            if cancelled() {
                return None;
            }
            let digit = givens[pos].take().unwrap();
            let current = given_cells(&givens);

            // The puzzle had one solution before this removal, so a second
            // one has to put another digit here
            let solved = if has_solution_without(ctx, &current, CellId(pos as u8), digit) {
                None
            } else {
                match solver.solve_quietly(&mut Board::new(&current), ctx) {
                    SolveResult::Solved(s) if hardest(&s) <= difficulty.ceiling() => Some(s),
                    _ => None,
                }
            };
            match solved {
                Some(s) => steps = Some(s),
                None => givens[pos] = Some(digit),
            }
        }

        let steps = steps?;
        let givens = given_cells(&givens);
        (hardest(&steps) >= difficulty.floor()).then(|| ScoredPuzzle {
            score: score(difficulty, &steps, givens.len()),
            givens,
        })
    }
}

/// Run numbered attempts on every core until `wanted` of them succeed, and
/// return those successes in attempt order.
///
/// They're always the lowest-numbered successes, whichever thread finishes
/// first: once `wanted` are in, attempts numbered past the last of them are
/// cancelled, but every lower-numbered one still runs to the end. `attempt`
/// gets its number and a `cancelled` check to give up early on.
fn first_successes<T: Send>(
    wanted: usize,
    attempt: impl Fn(u64, &dyn Fn() -> bool) -> Option<T> + Sync,
) -> Vec<T> {
    let next = AtomicU64::new(0);
    // the number of the last attempt that can still make it
    let cutoff = AtomicU64::new(u64::MAX);
    let found = Mutex::new(Vec::<(u64, T)>::new());
    let threads = thread::available_parallelism().map_or(1, |n| n.get());

    thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let n = next.fetch_add(1, Ordering::Relaxed);
                let cancelled = || n > cutoff.load(Ordering::Relaxed);
                if cancelled() {
                    break;
                }
                let Some(success) = attempt(n, &cancelled) else {
                    continue;
                };

                // keep the lowest-numbered `wanted`, in order
                let mut found = found.lock().unwrap();
                let at = found.partition_point(|&(m, _)| m < n);
                found.insert(at, (n, success));
                found.truncate(wanted);
                if found.len() == wanted {
                    cutoff.store(found[wanted - 1].0, Ordering::Relaxed);
                }
            });
        }
    });

    let found = found.into_inner().unwrap();
    found.into_iter().map(|(_, success)| success).collect()
}

/// A puzzle that reached its difficulty, and how good it is.
struct ScoredPuzzle {
    givens: Vec<(CellId, u8)>,
    score: i32,
}

/// The hardest strategy among the solver's steps.
fn hardest(steps: &[Deduction]) -> u8 {
    steps.iter().map(|d| d.difficulty).max().unwrap_or(0)
}

/// How good a puzzle is for its difficulty, higher is better: the technique
/// that sets the difficulty needed often, a gentle opening before it's first
/// needed, and few givens.
fn score(difficulty: Difficulty, steps: &[Deduction], givens: usize) -> i32 {
    // hidden singles are what make an easy puzzle
    let hard = |d: &Deduction| d.difficulty >= difficulty.floor().max(2);
    let hard_steps = steps.iter().filter(|d| hard(d)).count() as i32;
    // how far into the solve the first one comes, 0 to 10
    let first = steps.iter().position(hard).unwrap_or(steps.len());
    let opening = (10 * first / steps.len().max(1)) as i32;
    10 * hard_steps + opening - givens as i32
}

/// The filled cells of a puzzle as (cell, digit) pairs.
fn given_cells(givens: &[Option<u8>]) -> Vec<(CellId, u8)> {
    givens
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| Some((CellId(i as u8), d?)))
        .collect()
}

/// A grid being filled in by the search: the digits placed so far, and the
/// candidates left once those are removed from their peers. `grid` repeats
/// the board's placed digits because `Constraint::allows` reads a plain grid.
#[derive(Clone)]
struct Partial {
    board: Board,
    grid: [u8; 81],
}

impl Partial {
    fn empty() -> Self {
        Partial {
            board: Board::new(&[]),
            grid: [0; 81],
        }
    }

    /// Place the givens on an empty grid, or `None` if they break a rule.
    fn from_givens(ctx: &SolverContext, givens: &[(CellId, u8)]) -> Option<Self> {
        let mut partial = Self::empty();
        for &(cell, digit) in givens {
            if !partial.legal(ctx, cell).contains(digit) || !partial.place(ctx, cell, digit) {
                return None;
            }
        }
        Some(partial)
    }

    /// Digits that can still go in an empty cell.
    fn legal(&self, ctx: &SolverContext, cell: CellId) -> Candidates {
        ctx.constrain(&self.grid, cell, self.board.candidates(cell))
    }

    /// Place `digit` at `cell` and remove it from the cell's peers. Returns
    /// false when that leaves a peer without candidates.
    fn place(&mut self, ctx: &SolverContext, cell: CellId, digit: u8) -> bool {
        self.grid[cell.0 as usize] = digit;
        ctx.place(&mut self.board, cell, digit)
    }
}

/// Depth-first search, always branching on the empty cell with the fewest
/// legal digits. `order` arranges each cell's digits before trying them;
/// `on_solution` sees every complete grid and returns true to stop.
/// Returns true if the search was stopped.
fn search<O, F>(ctx: &SolverContext, partial: &Partial, order: &mut O, on_solution: &mut F) -> bool
where
    O: FnMut(&mut [u8]),
    F: FnMut(&[u8; 81]) -> bool,
{
    let mut best: Option<(CellId, Candidates)> = None;
    for i in 0..81 {
        if partial.grid[i] != 0 {
            continue;
        }
        let cell = CellId(i as u8);
        let legal = partial.legal(ctx, cell);
        if legal.count() == 0 {
            return false;
        }
        if best.is_none_or(|(_, b)| legal.count() < b.count()) {
            best = Some((cell, legal));
            if legal.count() == 1 {
                break;
            }
        }
    }

    let Some((cell, legal)) = best else {
        return on_solution(&partial.grid);
    };

    let mut digits = [0u8; 9];
    let n = legal.count() as usize;
    for (slot, d) in digits.iter_mut().zip(legal.iter()) {
        *slot = d;
    }
    order(&mut digits[..n]);
    for &d in &digits[..n] {
        let mut next = partial.clone();
        if next.place(ctx, cell, d) && search(ctx, &next, order, on_solution) {
            return true;
        }
    }
    false
}

/// First complete grid the search reaches from `partial`, if any.
fn first_solution<O: FnMut(&mut [u8])>(
    ctx: &SolverContext,
    partial: &Partial,
    order: &mut O,
) -> Option<[u8; 81]> {
    let mut solution = None;
    search(ctx, partial, order, &mut |grid| {
        solution = Some(*grid);
        true
    });
    solution
}

/// Whether the puzzle has a solution without `digit` at `cell`.
fn has_solution_without(
    ctx: &SolverContext,
    givens: &[(CellId, u8)],
    cell: CellId,
    digit: u8,
) -> bool {
    let Some(mut partial) = Partial::from_givens(ctx, givens) else {
        return false;
    };
    partial.board.remove_candidate(cell, digit);
    search(ctx, &partial, &mut |_| {}, &mut |_| true)
}

/// Count solutions of a standard puzzle, stopping once `limit` are found.
pub fn count_solutions(givens: &[(CellId, u8)], limit: usize) -> usize {
    count_solutions_for(&SolverContext::standard(), givens, limit)
}

/// Count solutions under `ctx`'s rules, stopping once `limit` are found.
pub fn count_solutions_for(ctx: &SolverContext, givens: &[(CellId, u8)], limit: usize) -> usize {
    let Some(partial) = Partial::from_givens(ctx, givens) else {
        return 0;
    };
    let mut count = 0;
    search(ctx, &partial, &mut |_| {}, &mut |_| {
        count += 1;
        count >= limit
    });
    count
}

/// First solution under `ctx`'s rules found by brute force, if any.
pub fn solve_brute(ctx: &SolverContext, givens: &[(CellId, u8)]) -> Option<[u8; 81]> {
    first_solution(ctx, &Partial::from_givens(ctx, givens)?, &mut |_| {})
}
