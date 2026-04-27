use crate::{board::Board, constraint::Constraint, strategy::{Deduction, Strategy}};

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

fn apply_deduction(board: &mut Board, deduction: &Deduction) {
    for &(cell, digit) in &deduction.eliminations {
        board.remove_candidate(cell, digit);
    }
    for &(cell, digit) in &deduction.placements {
        board.set_value(cell, digit);
    }
}