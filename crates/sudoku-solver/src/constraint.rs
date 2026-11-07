use crate::{
    board::{Board, CellId},
    house::House,
    strategy::Deduction,
    variants::{Arrow, Cage, Relation, RelationConstraint},
};

/// A rule on top of the houses. Houses already cover "each digit once per
/// row/column/box"; constraints add everything else a variant needs.
pub trait Constraint: Send + Sync {
    fn name(&self) -> &str;

    /// Every cell this constraint touches.
    fn cells(&self) -> &[CellId];

    /// Cell pairs that may not hold the same digit. These become peers, so
    /// the solver's peer elimination and the brute-force search pick them up.
    fn distinct_pairs(&self) -> Vec<(CellId, CellId)> {
        Vec::new()
    }

    /// Variant-specific candidate elimination. Peer elimination is handled
    /// by the solver, so constraints only need this for other logic.
    fn propagate(&self, _board: &mut Board) -> Vec<Deduction> {
        Vec::new()
    }

    /// Whether placing `digit` at `cell` is consistent with the digits already
    /// in `grid` (0 = empty). Used by the brute-force search. Repeats are
    /// already ruled out through `distinct_pairs`.
    fn allows(&self, _grid: &[u8; 81], _cell: CellId, _digit: u8) -> bool {
        true
    }
}

/// Cells a fixed set of offsets apart may not repeat a digit
/// (anti-knight, anti-king).
pub struct NoRepeatAtOffsets {
    name: &'static str,
    cells: Vec<CellId>,
    offsets: &'static [(i8, i8)],
}

const KNIGHT_OFFSETS: &[(i8, i8)] = &[
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

const KING_OFFSETS: &[(i8, i8)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl NoRepeatAtOffsets {
    pub fn anti_knight() -> Self {
        Self::new("anti-knight", KNIGHT_OFFSETS)
    }

    pub fn anti_king() -> Self {
        Self::new("anti-king", KING_OFFSETS)
    }

    fn new(name: &'static str, offsets: &'static [(i8, i8)]) -> Self {
        NoRepeatAtOffsets {
            name,
            cells: (0..81).map(CellId).collect(),
            offsets,
        }
    }
}

impl Constraint for NoRepeatAtOffsets {
    fn name(&self) -> &str {
        self.name
    }

    fn cells(&self) -> &[CellId] {
        &self.cells
    }

    fn distinct_pairs(&self) -> Vec<(CellId, CellId)> {
        let mut pairs = Vec::new();
        for &a in &self.cells {
            for &(dr, dc) in self.offsets {
                let r = a.row() as i8 + dr;
                let c = a.col() as i8 + dc;
                if (0..9).contains(&r) && (0..9).contains(&c) {
                    let b = CellId::from_rc(r as u8, c as u8);
                    // each unordered pair once
                    if a.0 < b.0 {
                        pairs.push((a, b));
                    }
                }
            }
        }
        pairs
    }
}

/// Serializable description of a variant rule. This is what the frontend
/// sends, saves and renders from; `build` turns it into solver objects.
/// Cells are indices 0..81 (row * 9 + col).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub enum ConstraintSpec {
    /// X-sudoku: both main diagonals are houses.
    Diagonals,
    AntiKnight,
    AntiKing,
    /// Killer cage; `sum` is optional for unlabelled cages.
    Cage {
        cells: Vec<u8>,
        sum: Option<u8>,
    },
    /// A line through adjacent cells, in order. Thermos start at the bulb.
    Line {
        kind: LineKind,
        cells: Vec<u8>,
    },
    /// A marker between two orthogonally adjacent cells.
    Marker {
        kind: MarkerKind,
        cells: [u8; 2],
    },
    /// The shaft's digits add up to the circle's. Cells start at the circle
    /// and follow the shaft to the arrowhead.
    Arrow {
        cells: Vec<u8>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum LineKind {
    GermanWhisper,
    DutchWhisper,
    Thermo,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum MarkerKind {
    /// consecutive digits
    White,
    /// one digit is double the other
    Black,
    /// digits sum to 10
    X,
    /// first cell is greater; the chevron points at the second
    Chevron,
}

fn adjacent(a: CellId, b: CellId) -> bool {
    a != b && a.row().abs_diff(b.row()) <= 1 && a.col().abs_diff(b.col()) <= 1
}

fn orthogonal(a: CellId, b: CellId) -> bool {
    adjacent(a, b) && (a.row() == b.row() || a.col() == b.col())
}

impl ConstraintSpec {
    /// Rejects specs the solver can't build, such as out-of-range cells.
    pub fn validate(&self) -> Result<(), String> {
        let cells: &[u8] = match self {
            ConstraintSpec::Cage { cells, .. }
            | ConstraintSpec::Line { cells, .. }
            | ConstraintSpec::Arrow { cells } => cells,
            ConstraintSpec::Marker { cells, .. } => cells,
            _ => &[],
        };
        if let Some(c) = cells.iter().find(|&&c| c >= 81) {
            return Err(format!("cell {} is out of range", c));
        }
        let ids: Vec<CellId> = cells.iter().map(|&c| CellId(c)).collect();
        let connected = ids.windows(2).all(|w| adjacent(w[0], w[1]));
        match self {
            ConstraintSpec::Cage { sum, .. } => {
                if ids.is_empty() || ids.len() > 9 {
                    return Err("a cage needs 1 to 9 cells".into());
                }
                if sum.is_some_and(|s| s == 0 || s > 45) {
                    return Err("cage sum must be 1 to 45".into());
                }
            }
            ConstraintSpec::Line { .. } if ids.len() < 2 || !connected => {
                return Err("a line needs 2+ cells, each touching the next".into());
            }
            // the shaft can hold at most nine 1s, and the circle at most 9
            ConstraintSpec::Arrow { .. } if !(2..=10).contains(&ids.len()) || !connected => {
                return Err(
                    "an arrow needs a circle and 1 to 9 shaft cells, each touching the next".into(),
                );
            }
            ConstraintSpec::Marker { .. } if !orthogonal(ids[0], ids[1]) => {
                return Err("a marker sits between two orthogonal neighbours".into());
            }
            _ => {}
        }
        Ok(())
    }
}

/// Houses and constraints a spec adds on top of the standard rules.
/// Specs are expected to have passed `validate`.
pub(crate) fn build(spec: &ConstraintSpec, houses: &mut Vec<House>) -> Option<Box<dyn Constraint>> {
    let ids = |cells: &[u8]| -> Vec<CellId> { cells.iter().map(|&c| CellId(c)).collect() };
    match spec {
        ConstraintSpec::Diagonals => {
            houses.push(House::diagonal(0));
            houses.push(House::diagonal(1));
            None
        }
        ConstraintSpec::AntiKnight => Some(Box::new(NoRepeatAtOffsets::anti_knight())),
        ConstraintSpec::AntiKing => Some(Box::new(NoRepeatAtOffsets::anti_king())),
        ConstraintSpec::Cage { cells, sum } => Some(Box::new(Cage::new(ids(cells), *sum))),
        ConstraintSpec::Line { kind, cells } => {
            let cells = ids(cells);
            let line = match kind {
                LineKind::GermanWhisper => RelationConstraint::line(
                    &format!("German whisper at {}", cells[0]),
                    cells,
                    Relation::DiffAtLeast(5),
                    false,
                ),
                LineKind::DutchWhisper => RelationConstraint::line(
                    &format!("Dutch whisper at {}", cells[0]),
                    cells,
                    Relation::DiffAtLeast(4),
                    false,
                ),
                LineKind::Thermo => RelationConstraint::line(
                    &format!("thermo at {}", cells[0]),
                    cells,
                    Relation::Less,
                    true,
                ),
            };
            Some(Box::new(line))
        }
        ConstraintSpec::Arrow { cells } => Some(Box::new(Arrow::new(ids(cells)))),
        ConstraintSpec::Marker { kind, cells } => {
            let (a, b) = (CellId(cells[0]), CellId(cells[1]));
            let (name, rel) = match kind {
                MarkerKind::White => ("hollow dot", Relation::Consecutive),
                MarkerKind::Black => ("filled dot", Relation::Ratio2),
                MarkerKind::X => ("X", Relation::Sum(10)),
                MarkerKind::Chevron => ("chevron", Relation::Greater),
            };
            let name = format!("{} between {} and {}", name, a, b);
            Some(Box::new(RelationConstraint::edge(&name, a, b, rel)))
        }
    }
}
