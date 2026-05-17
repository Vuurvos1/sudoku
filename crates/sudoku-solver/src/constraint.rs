use crate::{
    board::{Board, CellId},
    strategy::{Deduction, Explanation},
};

pub trait Constraint {
    fn name(&self) -> &str;
    fn cells(&self) -> &[CellId];

    /// Default propagation: for each decided cell, eliminate its digit from peers.
    fn propagate(&self, board: &mut Board) -> Vec<Deduction> {
        let mut deductions = Vec::new();
        for &source in self.cells() {
            let Some(digit) = board.value(source) else {
                continue;
            };
            let mut eliminations = Vec::new();
            for &peer in self.cells() {
                if peer == source {
                    continue;
                }
                if board.candidates(peer).contains(digit) {
                    board.remove_candidate(peer, digit);
                    eliminations.push((peer, digit));
                }
            }
            if !eliminations.is_empty() {
                deductions.push(Deduction {
                    strategy: "constraint propagation",
                    difficulty: 0,
                    placements: vec![],
                    eliminations,
                    explanation: Explanation::group_elimination(self.name(), source, digit),
                });
            }
        }
        deductions
    }
}

pub struct UniqueGroup {
    pub cells: Vec<CellId>,
    pub kind: GroupKind,
    name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GroupKind {
    Row(u8),
    Col(u8),
    Box(u8),
}

impl std::fmt::Display for GroupKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GroupKind::Row(r) => write!(f, "row {}", r + 1),
            GroupKind::Col(c) => write!(f, "column {}", c + 1),
            GroupKind::Box(b) => write!(f, "box {}", b + 1),
        }
    }
}

impl UniqueGroup {
    pub fn row(r: u8) -> Self {
        let kind = GroupKind::Row(r);
        UniqueGroup {
            cells: (0..9).map(|c| CellId::from_rc(r, c)).collect(),
            name: kind.to_string(),
            kind,
        }
    }

    pub fn col(c: u8) -> Self {
        let kind = GroupKind::Col(c);
        UniqueGroup {
            cells: (0..9).map(|r| CellId::from_rc(r, c)).collect(),
            name: kind.to_string(),
            kind,
        }
    }

    pub fn box_(b: u8) -> Self {
        let row_start = b / 3 * 3;
        let col_start = b % 3 * 3;
        let kind = GroupKind::Box(b);
        let cells = (0..9)
            .map(|i| CellId::from_rc(row_start + i / 3, col_start + i % 3))
            .collect();
        UniqueGroup {
            cells,
            name: kind.to_string(),
            kind,
        }
    }
}

impl Constraint for UniqueGroup {
    fn name(&self) -> &str {
        &self.name
    }
    fn cells(&self) -> &[CellId] {
        &self.cells
    }
    // uses default propagate()
}

/// Returns all cells that share a constraint group with the given cell (excluding itself).
pub fn peers(cell: CellId, constraints: &[Box<dyn Constraint>]) -> Vec<CellId> {
    let mut result = Vec::new();
    for c in constraints {
        let cells = c.cells();
        if cells.contains(&cell) {
            for &peer in cells {
                if peer != cell && !result.contains(&peer) {
                    result.push(peer);
                }
            }
        }
    }
    result.sort_by_key(|c| c.0);
    result
}

/// Returns the peer set for every cell (0..81) as a flat map.
pub fn all_peers(constraints: &[Box<dyn Constraint>]) -> Vec<Vec<CellId>> {
    (0..81u8).map(|i| peers(CellId(i), constraints)).collect()
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
