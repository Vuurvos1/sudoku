use crate::{board::{Board, CellId}, constraint::GroupKind, solver::SolverContext};

pub enum HighlightRole {
    Focus,
    Supporting,
    Eliminated,
}

pub struct CellHighlight {
    pub cell: CellId,
    pub role: HighlightRole,
}

pub struct CandidateHighlight {
    pub cell: CellId,
    pub digit: u8,
    pub role: HighlightRole,
}

pub struct Highlights {
    pub cells: Vec<CellHighlight>,
    pub candidates: Vec<CandidateHighlight>,
}

pub struct Explanation {
    pub summary: String,
    pub detail: String,
    pub highlights: Vec<Highlights>
}

impl Explanation {
    pub fn group_elimination(kind: GroupKind, source: CellId, digit: u8) -> Self {
        let group_name = match kind {
            GroupKind::Row(r) => format!("Row {}", r + 1),
            GroupKind::Col(c) => format!("Column {}", c + 1),
            GroupKind::Box(b) => format!("Box {}", b + 1),
        };
        Explanation {
            summary: format!("{}: {} placed at {}", group_name, digit, source),
            detail: format!(
                "{} already contains {} at {}. The unique-digit rule eliminates \
                 {} from every other cell in the group.",
                group_name, digit, source, digit
            ),
            highlights: vec![Highlights {
                cells: vec![CellHighlight { cell: source, role: HighlightRole::Focus }],
                candidates: vec![CandidateHighlight {
                    cell: source, digit, role: HighlightRole::Focus,
                }],
            }],
        }
    }

    pub fn naked_single(cell: CellId, digit: u8) -> Self {
        Explanation {
            summary: format!("Naked Single: {} = {}", cell, digit),
            detail: format!(
                "{} has only one remaining candidate. All other digits have been \
                 eliminated by its row, column, and box, so it must be {}.",
                cell, digit
            ),
            highlights: vec![Highlights {
                cells: vec![
                    CellHighlight { cell, role: HighlightRole::Focus },
                ],
                candidates: vec![
                    CandidateHighlight { cell, digit, role: HighlightRole::Focus },
                ],
            }],
        }
    }

    pub fn hidden_single(cell: CellId, digit: u8) -> Self {
        Explanation {
            summary: format!("Hidden Single: {} = {}", cell, digit),
            detail: format!(
                "{} is the only cell in one of its groups (row, column, or box) \
                 that can hold {}, so it must be {}.",
                cell, digit, digit
            ),
            highlights: vec![Highlights {
                cells: vec![
                    CellHighlight { cell, role: HighlightRole::Focus },
                ],
                candidates: vec![
                    CandidateHighlight { cell, digit, role: HighlightRole::Focus },
                ],
            }],
        }
    }

    pub fn pointing(box_idx: u8, digit: u8, line: GroupKind, locked: &[CellId]) -> Self {
        let line_name = match line {
            GroupKind::Row(r) => format!("row {}", r + 1),
            GroupKind::Col(c) => format!("column {}", c + 1),
            GroupKind::Box(b) => format!("box {}", b + 1),
        };
        let kind_word = if locked.len() == 2 { "Pair" } else { "Triple" };
        Explanation {
            summary: format!(
                "Pointing {}: {} in box {} locked to {}",
                kind_word, digit, box_idx + 1, line_name
            ),
            detail: format!(
                "In box {}, {} can only go in cells of {}. So {} can be \
                 eliminated from the rest of {}.",
                box_idx + 1, digit, line_name, digit, line_name
            ),
            highlights: vec![Highlights {
                cells: locked.iter().map(|&cell| CellHighlight {
                    cell, role: HighlightRole::Focus,
                }).collect(),
                candidates: locked.iter().map(|&cell| CandidateHighlight {
                    cell, digit, role: HighlightRole::Focus,
                }).collect(),
            }],
        }
    }

    pub fn box_line_reduction(box_idx: u8, digit: u8, line: GroupKind, locked: &[CellId]) -> Self {
        let line_name = match line {
            GroupKind::Row(r) => format!("row {}", r + 1),
            GroupKind::Col(c) => format!("column {}", c + 1),
            GroupKind::Box(b) => format!("box {}", b + 1),
        };
        Explanation {
            summary: format!(
                "Box/Line Reduction: {} in {} locked to box {}",
                digit, line_name, box_idx + 1
            ),
            detail: format!(
                "In {}, {} can only go in cells inside box {}. So {} can be \
                 eliminated from the rest of box {}.",
                line_name, digit, box_idx + 1, digit, box_idx + 1
            ),
            highlights: vec![Highlights {
                cells: locked.iter().map(|&cell| CellHighlight {
                    cell, role: HighlightRole::Focus,
                }).collect(),
                candidates: locked.iter().map(|&cell| CandidateHighlight {
                    cell, digit, role: HighlightRole::Focus,
                }).collect(),
            }],
        }
    }
}

pub trait Strategy {
    fn name(&self) -> &'static str;
    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction>;
}

pub struct Deduction {
    pub strategy: &'static str,
    pub difficulty: u8,
    pub placements: Vec<(CellId, u8)>,
    pub eliminations: Vec<(CellId, u8)>,
    pub explanation: Explanation,
}