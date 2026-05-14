use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Candidates(u16); // bits 1..=9 mean digits 1..=9

impl Candidates {
    pub const EMPTY: Self = Candidates(0);
    pub const ALL: Self = Candidates(0b11_1111_1110);

    pub fn single(d: u8) -> Self { Candidates(1 << d)}
    pub fn contains(self, d: u8) -> bool {self.0 & (1 << d) != 0}
    pub fn count(self) -> u32 { self.0.count_ones() } // change to u8?
    pub fn insert(&mut self, d: u8) { self.0 |= 1 << d;}
    pub fn remove(&mut self, d: u8) { self.0 &= !(1 << d);}
    pub fn iter(self) -> impl Iterator<Item = u8> { (1..=9).filter(move |d| self.contains(*d)) }
    pub fn union(self, other: Self) -> Self { Candidates(self.0 | other.0)}
    pub fn intersect(self, other: Self) -> Self { Candidates(self.0 & other.0)}
    pub fn is_subset_of(self, other: Self) -> bool { self.0 & other.0 == self.0 }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CellId(pub u8); // 0..81

impl CellId {
    pub fn row(self) -> u8 { self.0 / 9 }
    pub fn col(self) -> u8 { self.0 % 9 }
    pub fn box_idx(self) -> u8 { (self.row() / 3) * 3 + self.col() / 3 }
    pub fn from_rc(r: u8, c: u8) -> Self { CellId(r * 9 + c) }
}

impl fmt::Display for CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R{}C{}", self.row() + 1, self.col() + 1)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    Empty,   // No digit yet
    Placed,  // Solver/user placed a digit, Erasable
    Given,   // Part of the original puzzle, Immutable
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub candidates: Candidates,
    pub state: CellState,
}

impl Cell {
    pub const fn blank() -> Self {
        Cell { candidates: Candidates::ALL, state: CellState::Empty }
    }
}

pub struct Board {
    cells: [Cell; 81],
}

impl Board {
    pub fn new(givens: &[(CellId, u8)]) -> Self {
        let mut board = Board { cells: [Cell::blank(); 81] };

        for &(id, digit) in givens {
            board.set_given(id, digit);
        }

        board
    }

    pub fn from_array(array: &[u8; 81]) -> Self {
        let mut board = Board { cells: [Cell::blank(); 81] };

        for (i, &digit) in array.iter().enumerate() {
            if digit != 0 {
                board.set_given(CellId(i as u8), digit);
            }
        }

        board
    }

    /// Parse an 81-char puzzle string (`1`–`9` for givens, `.` or `0` for empty).
    pub fn from_line(s: &str) -> Self {
        assert!(s.chars().count() == 81, "puzzle string must be exactly 81 characters");
        let mut array = [0u8; 81];
        for (i, ch) in s.chars().enumerate() {
            array[i] = match ch {
                '1'..='9' => ch as u8 - b'0',
                '.' | '0' => 0,
                _ => panic!("unexpected character '{}' at position {}", ch, i),
            };
        }
        Self::from_array(&array)
    }

    /// Place a digit during solving, Erasable
    pub fn set_value(&mut self, id: CellId, digit: u8) {
        let cell = &mut self.cells[id.0 as usize];
        cell.candidates = Candidates::single(digit);
        cell.state = CellState::Placed;
    }

    /// Lock in a digit from the original puzzle, Immutable
    pub fn set_given(&mut self, id: CellId, digit: u8) {
        let cell = &mut self.cells[id.0 as usize];
        cell.candidates = Candidates::single(digit);
        cell.state = CellState::Given;
    }

    pub fn remove_candidate(&mut self, id: CellId, digit: u8) {
        self.cells[id.0 as usize].candidates.remove(digit);
    }

    pub fn candidates(&self, id: CellId) -> Candidates { self.cells[id.0 as usize].candidates }

    /// True if the cell already has a definite digit (given or placed)
    pub fn is_decided(&self, id: CellId) -> bool {
        self.cells[id.0 as usize].state != CellState::Empty
    }

    pub fn is_given(&self, id: CellId) -> bool {
        self.cells[id.0 as usize].state == CellState::Given
    }

    pub fn value(&self, id: CellId) -> Option<u8> {
        let c = self.candidates(id);
        if c.count() == 1 { c.iter().next() } else { None }
    }

    /// True when every cell has been decided (Given or Placed)
    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|c| c.state != CellState::Empty)
    }

    /// Cheap broken-state detector: a cell with no remaining candidates.
    pub fn has_contradiction(&self) -> bool {
        self.cells.iter().any(|c| c.candidates.count() == 0)
    }
}