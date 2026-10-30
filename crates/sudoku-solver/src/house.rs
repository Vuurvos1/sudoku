use crate::board::CellId;

/// Nine cells that must contain each digit 1–9 exactly once.
pub struct House {
    pub cells: Vec<CellId>,
    pub kind: HouseKind,
    pub name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum HouseKind {
    Row(u8),
    Col(u8),
    Box(u8),
    /// 0 = top-left to bottom-right, 1 = top-right to bottom-left
    Diagonal(u8),
}

impl std::fmt::Display for HouseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HouseKind::Row(r) => write!(f, "row {}", r + 1),
            HouseKind::Col(c) => write!(f, "column {}", c + 1),
            HouseKind::Box(b) => write!(f, "box {}", b + 1),
            HouseKind::Diagonal(0) => write!(f, "the main diagonal"),
            HouseKind::Diagonal(_) => write!(f, "the anti-diagonal"),
        }
    }
}

impl House {
    fn new(kind: HouseKind, cells: Vec<CellId>) -> Self {
        House {
            cells,
            name: kind.to_string(),
            kind,
        }
    }

    pub fn row(r: u8) -> Self {
        House::new(
            HouseKind::Row(r),
            (0..9).map(|c| CellId::from_rc(r, c)).collect(),
        )
    }

    pub fn col(c: u8) -> Self {
        House::new(
            HouseKind::Col(c),
            (0..9).map(|r| CellId::from_rc(r, c)).collect(),
        )
    }

    pub fn box_(b: u8) -> Self {
        let row_start = b / 3 * 3;
        let col_start = b % 3 * 3;
        House::new(
            HouseKind::Box(b),
            (0..9)
                .map(|i| CellId::from_rc(row_start + i / 3, col_start + i % 3))
                .collect(),
        )
    }

    pub fn diagonal(d: u8) -> Self {
        House::new(
            HouseKind::Diagonal(d),
            (0..9)
                .map(|i| CellId::from_rc(i, if d == 0 { i } else { 8 - i }))
                .collect(),
        )
    }
}

/// The 27 houses of a standard 9×9 sudoku: 9 rows + 9 columns + 9 boxes.
pub fn standard_houses() -> Vec<House> {
    let mut houses = Vec::with_capacity(27);
    for i in 0..9 {
        houses.push(House::row(i));
        houses.push(House::col(i));
        houses.push(House::box_(i));
    }
    houses
}
