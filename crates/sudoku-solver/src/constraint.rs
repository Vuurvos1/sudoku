use crate::{board::{Board, CellId}, strategy::{Deduction, Explanation}};

pub trait Constraint {
    fn cells(&self) -> &[CellId];

    fn propagate(&self, board: &mut Board) -> Vec<Deduction>;
}

pub struct UniqueGroup {
    pub cells: Vec<CellId>,
    pub kind: GroupKind,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GroupKind {
    Row(u8),
    Col(u8),
    Box(u8),
}

impl UniqueGroup {
    pub fn row(r: u8) -> Self {
        UniqueGroup {
            cells: (0..9).map(|c| CellId::from_rc(r, c)).collect(),
            kind: GroupKind::Row(r),
        }
    }

    pub fn col(c: u8) -> Self {
        UniqueGroup {
            cells: (0..9).map(|r| CellId::from_rc(r, c)).collect(),
            kind: GroupKind::Col(c),
        }
    }

    pub fn box_(b: u8) -> Self {
        let row_start = b / 3 * 3;
        let col_start = b % 3 * 3;
        let cells = (0..9).map(|i| CellId::from_rc(row_start + i / 3, col_start + i % 3)).collect();
        UniqueGroup { cells, kind: GroupKind::Box(b) }
    }
}

impl Constraint for UniqueGroup {
    fn cells(&self) -> &[CellId] { &self.cells }

    fn propagate(&self, board: &mut Board) -> Vec<Deduction> {
        let mut deductions = Vec::new();

        for &source in &self.cells {
            let Some(digit) = board.value(source) else { continue };

            let mut eliminations = Vec::new();
            for &peer in &self.cells {
                if peer == source { continue; }
                if board.candidates(peer).contains(digit) {
                    board.remove_candidate(peer, digit);
                    eliminations.push((peer, digit));
                }
            }

            if !eliminations.is_empty() {
                deductions.push(Deduction {
                    strategy: "unique group propagation",
                    difficulty: 0,
                    placements: vec![],
                    eliminations,
                    explanation: Explanation::group_elimination(self.kind, source, digit),
                });
            }
        }

        deductions
    }
}

/// The 27 unique-distinct constraints of a standard 9×9 sudoku
/// 9 rows + 9 columns + 9 boxes
pub fn standard_constraints() -> Vec<Box<dyn Constraint>> {
    let mut cs: Vec<Box<dyn Constraint>> = Vec::with_capacity(27);
    for i in 0..9 {
        cs.push(Box::new(UniqueGroup::row(i)));
        cs.push(Box::new(UniqueGroup::col(i)));
        cs.push(Box::new(UniqueGroup::box_(i)));
    }
    cs
}