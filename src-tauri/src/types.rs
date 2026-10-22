use serde::{Deserialize, Serialize};
use sudoku_solver::{
    constraint::ConstraintSpec,
    strategy::{Deduction, HighlightRole},
};

/// A puzzle as the frontend sees it: an 81-char line of digits
/// (`.` or `0` for empty) plus its variant rules.
#[derive(Serialize, Deserialize)]
pub struct Puzzle {
    pub givens: String,
    #[serde(default)]
    pub constraints: Vec<ConstraintSpec>,
}

#[derive(Serialize)]
pub struct CellAction {
    pub cell: u8,
    pub digit: u8,
}

#[derive(Serialize)]
pub struct HighlightCell {
    pub cell: u8,
    pub role: String,
}

#[derive(Serialize)]
pub struct HighlightCandidate {
    pub cell: u8,
    pub digit: u8,
    pub role: String,
}

#[derive(Serialize)]
pub struct HighlightGroup {
    pub cells: Vec<HighlightCell>,
    pub candidates: Vec<HighlightCandidate>,
}

#[derive(Serialize)]
pub struct HintResponse {
    pub strategy: String,
    pub difficulty: u8,
    pub summary: String,
    pub detail: String,
    pub placements: Vec<CellAction>,
    pub eliminations: Vec<CellAction>,
    pub highlights: Vec<HighlightGroup>,
}

fn role_to_string(role: &HighlightRole) -> String {
    match role {
        HighlightRole::Focus => "focus".to_string(),
        HighlightRole::Supporting => "supporting".to_string(),
        HighlightRole::Eliminated => "eliminated".to_string(),
    }
}

pub fn hint_response_from(d: &Deduction) -> HintResponse {
    HintResponse {
        strategy: d.strategy.to_string(),
        difficulty: d.difficulty,
        summary: d.explanation.summary.clone(),
        detail: d.explanation.detail.clone(),
        placements: d
            .placements
            .iter()
            .map(|&(cell, digit)| CellAction {
                cell: cell.0,
                digit,
            })
            .collect(),
        eliminations: d
            .eliminations
            .iter()
            .map(|&(cell, digit)| CellAction {
                cell: cell.0,
                digit,
            })
            .collect(),
        highlights: d
            .explanation
            .highlights
            .iter()
            .map(|h| HighlightGroup {
                cells: h
                    .cells
                    .iter()
                    .map(|c| HighlightCell {
                        cell: c.cell.0,
                        role: role_to_string(&c.role),
                    })
                    .collect(),
                candidates: h
                    .candidates
                    .iter()
                    .map(|c| HighlightCandidate {
                        cell: c.cell.0,
                        digit: c.digit,
                        role: role_to_string(&c.role),
                    })
                    .collect(),
            })
            .collect(),
    }
}
