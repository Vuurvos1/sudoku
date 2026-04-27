use crate::{board::{Board, CellId}, solver::SolverContext, strategy::{Deduction, Explanation, Strategy}};

pub struct NakedSingle;

impl Strategy for NakedSingle {
    fn name(&self) -> &'static str { "naked single" }
    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for i in 0..81 {
            let id = CellId(i);
            let candidates = board.candidates(id);

            if candidates.count() == 1 && !board.is_decided(id) {
                let digit = candidates.iter().next().unwrap();
                return Some(Deduction {
                    strategy: "naked single",
                    difficulty: 1,
                    placements: vec![(id, digit)],
                    eliminations: vec![],
                    explanation: Explanation::naked_single(id, digit),
                })
            }
        }

        None
    }
}

// naked pairs
// naked triples

// hidden singles
// hidden pairs
// hidden triples

// x-wing
// y-wing
// swordfish