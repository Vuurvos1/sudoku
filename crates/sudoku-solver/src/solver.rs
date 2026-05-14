use crate::{board::Board, constraint::Constraint, strategy::{Deduction, Strategy}};
use crate::strategies::{
    NakedSingle, HiddenSingle, PointingPairs, BoxLineReduction,
    NakedSubset, HiddenSubset, Fish, YWing,
};

pub struct SolverContext<'a> {
    pub constraints: &'a [Box<dyn Constraint>],
}

pub enum SolveResult {
    Solved(Vec<Deduction>),
    Stuck(Vec<Deduction>),
    Invalid(Vec<Deduction>),
}

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Solver { strategies }
    }

    /// Find the next deduction without applying it.
    pub fn step(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for strategy in &self.strategies {
            if let Some(deduction) = strategy.find(board, ctx) {
                return Some(deduction);
            }
        }
        None
    }

    /// Propagate constraints, then find the next strategy-based deduction.
    /// Propagation mutates the board's candidate sets as a side-effect but
    /// those deductions are not returned — only the strategy deduction is.
    /// Returns `None` if the board is solved (including by propagation) or
    /// has a contradiction or no strategy applies.
    pub fn hint(&self, board: &mut Board, ctx: &SolverContext) -> Option<Deduction> {
        for c in ctx.constraints {
            c.propagate(board);
        }
        if board.is_solved() || board.has_contradiction() {
            return None;
        }
        self.step(board, ctx)
    }

    /// Solve the puzzle, returning all deductions made (propagation + strategies)
    pub fn solve(&self, board: &mut Board, ctx: &SolverContext) -> SolveResult {
        let mut steps = Vec::new();
        loop {
            // Run all constraints to strip impossible candidates
            for c in ctx.constraints {
                steps.extend(c.propagate(board));
            }

            if board.has_contradiction() {
                return SolveResult::Invalid(steps);
            }

            if board.is_solved() {
                return SolveResult::Solved(steps);
            }

            // Ask strategies for the next deduction
            match self.step(board, ctx) {
                Some(deduction) => {
                    apply_deduction(board, &deduction);
                    steps.push(deduction);
                }
                None => return SolveResult::Stuck(steps),
            }
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver::new(vec![
            Box::new(NakedSingle),
            Box::new(HiddenSingle),
            Box::new(PointingPairs),
            Box::new(BoxLineReduction),
            Box::new(NakedSubset::<2>),
            Box::new(HiddenSubset::<2>),
            Box::new(NakedSubset::<3>),
            Box::new(HiddenSubset::<3>),
            Box::new(Fish::<2>),
            Box::new(Fish::<3>),
            Box::new(YWing),
        ])
    }
}

fn apply_deduction(board: &mut Board, deduction: &Deduction) {
    for &(cell, digit) in &deduction.eliminations {
        board.remove_candidate(cell, digit);
    }
    for &(cell, digit) in &deduction.placements {
        board.set_value(cell, digit);
    }
}