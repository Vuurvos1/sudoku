use crate::{
    board::{Board, Candidates, CellId},
    constraint::Constraint,
    strategy::{Deduction, Explanation},
};

/// A rule between two cells `a` and `b`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Relation {
    /// |a - b| = 1 (white dot)
    Consecutive,
    /// one is double the other (black dot)
    Ratio2,
    /// a + b = n (X = 10)
    Sum(u8),
    /// a > b (chevron pointing at b)
    Greater,
    /// a < b (thermometer, bulb first)
    Less,
    /// |a - b| >= n (German whispers 5, Dutch whispers 4)
    DiffAtLeast(u8),
}

impl Relation {
    pub fn holds(self, a: u8, b: u8) -> bool {
        match self {
            Relation::Consecutive => a.abs_diff(b) == 1,
            Relation::Ratio2 => a == 2 * b || b == 2 * a,
            Relation::Sum(n) => a + b == n,
            Relation::Greater => a > b,
            Relation::Less => a < b,
            Relation::DiffAtLeast(n) => a.abs_diff(b) >= n,
        }
    }

    /// The same rule seen from `b`'s side.
    fn flipped(self) -> Self {
        match self {
            Relation::Greater => Relation::Less,
            Relation::Less => Relation::Greater,
            symmetric => symmetric,
        }
    }
}

/// Digits in `of` that have at least one partner in `other` under `rel`.
fn supported(rel: Relation, of: Candidates, other: Candidates) -> Candidates {
    let mut keep = Candidates::EMPTY;
    for d in of.iter() {
        if other.iter().any(|e| rel.holds(d, e)) {
            keep.insert(d);
        }
    }
    keep
}

/// Remove every candidate in `cell` that is not in `keep`, recording it.
fn restrict(board: &mut Board, cell: CellId, keep: Candidates, out: &mut Vec<(CellId, u8)>) {
    for d in board.candidates(cell).iter() {
        if !keep.contains(d) {
            board.remove_candidate(cell, d);
            out.push((cell, d));
        }
    }
}

/// Pairwise relations between cells: dots, X, chevrons, whispers, thermos.
pub struct RelationConstraint {
    name: String,
    cells: Vec<CellId>,
    edges: Vec<(CellId, CellId, Relation)>,
    /// All cells on the constraint hold different digits (thermos).
    distinct: bool,
}

impl RelationConstraint {
    pub fn edge(name: &str, a: CellId, b: CellId, rel: Relation) -> Self {
        RelationConstraint {
            name: name.to_string(),
            cells: vec![a, b],
            edges: vec![(a, b, rel)],
            distinct: false,
        }
    }

    /// `rel` between each pair of consecutive cells along `cells`.
    pub fn line(name: &str, cells: Vec<CellId>, rel: Relation, distinct: bool) -> Self {
        let edges = cells.windows(2).map(|w| (w[0], w[1], rel)).collect();
        RelationConstraint {
            name: name.to_string(),
            cells,
            edges,
            distinct,
        }
    }
}

impl Constraint for RelationConstraint {
    fn name(&self) -> &str {
        &self.name
    }

    fn cells(&self) -> &[CellId] {
        &self.cells
    }

    fn distinct_pairs(&self) -> Vec<(CellId, CellId)> {
        if !self.distinct {
            return Vec::new();
        }
        let mut pairs = Vec::new();
        for (i, &a) in self.cells.iter().enumerate() {
            for &b in &self.cells[i + 1..] {
                pairs.push((a, b));
            }
        }
        pairs
    }

    /// Arc consistency: drop any digit with no partner across one of its edges,
    /// repeating until nothing changes.
    fn propagate(&self, board: &mut Board) -> Vec<Deduction> {
        let mut eliminations = Vec::new();
        loop {
            let before = eliminations.len();
            for &(a, b, rel) in &self.edges {
                let (ca, cb) = (board.candidates(a), board.candidates(b));
                restrict(board, a, supported(rel, ca, cb), &mut eliminations);
                let ca = board.candidates(a);
                restrict(
                    board,
                    b,
                    supported(rel.flipped(), cb, ca),
                    &mut eliminations,
                );
            }
            if eliminations.len() == before {
                break;
            }
        }
        if eliminations.is_empty() {
            return Vec::new();
        }
        vec![Deduction {
            strategy: "constraint propagation",
            difficulty: 0,
            placements: vec![],
            explanation: Explanation::variant_elimination(&self.name, &eliminations),
            eliminations,
        }]
    }

    fn allows(&self, grid: &[u8; 81], cell: CellId, digit: u8) -> bool {
        self.edges.iter().all(|&(a, b, rel)| {
            if a == cell {
                let other = grid[b.0 as usize];
                other == 0 || rel.holds(digit, other)
            } else if b == cell {
                let other = grid[a.0 as usize];
                other == 0 || rel.holds(other, digit)
            } else {
                true
            }
        })
    }
}

/// Killer cage: no repeats, and the digits add up to `sum` if given.
pub struct Cage {
    name: String,
    cells: Vec<CellId>,
    sum: Option<u8>,
}

impl Cage {
    pub fn new(cells: Vec<CellId>, sum: Option<u8>) -> Self {
        let name = match sum {
            Some(s) => format!("{} cage at {}", s, cells[0]),
            None => format!("cage at {}", cells[0]),
        };
        Cage { name, cells, sum }
    }
}

/// Smallest and largest total `k` different digits can reach.
fn sum_bounds(k: u8) -> (u8, u8) {
    let min = k * (k + 1) / 2;
    let max = (10 - k..=9).sum();
    (min, max)
}

impl Constraint for Cage {
    fn name(&self) -> &str {
        &self.name
    }

    fn cells(&self) -> &[CellId] {
        &self.cells
    }

    fn distinct_pairs(&self) -> Vec<(CellId, CellId)> {
        let mut pairs = Vec::new();
        for (i, &a) in self.cells.iter().enumerate() {
            for &b in &self.cells[i + 1..] {
                pairs.push((a, b));
            }
        }
        pairs
    }

    /// Keep only digits that appear in some way of filling the whole cage
    /// with different digits that add up to the sum.
    fn propagate(&self, board: &mut Board) -> Vec<Deduction> {
        let Some(sum) = self.sum else {
            return Vec::new();
        };
        let cands: Vec<Candidates> = self.cells.iter().map(|&c| board.candidates(c)).collect();
        let mut possible = vec![Candidates::EMPTY; self.cells.len()];
        let mut chosen = Vec::with_capacity(self.cells.len());
        fill_cage(&cands, sum, &mut chosen, &mut possible);

        let mut eliminations = Vec::new();
        for (&cell, &keep) in self.cells.iter().zip(&possible) {
            restrict(board, cell, keep, &mut eliminations);
        }
        if eliminations.is_empty() {
            return Vec::new();
        }
        vec![Deduction {
            strategy: "constraint propagation",
            difficulty: 0,
            placements: vec![],
            explanation: Explanation::variant_elimination(&self.name, &eliminations),
            eliminations,
        }]
    }

    fn allows(&self, grid: &[u8; 81], cell: CellId, digit: u8) -> bool {
        let Some(sum) = self.sum else {
            return true;
        };
        let mut total = digit;
        let mut empty = 0u8;
        for &c in &self.cells {
            match grid[c.0 as usize] {
                _ if c == cell => {}
                0 => empty += 1,
                d => total += d,
            }
        }
        let (min, max) = sum_bounds(empty);
        total + min <= sum && total + max >= sum
    }
}

/// Depth-first over the cage's cells, recording in `possible` every digit
/// used by a complete assignment that hits `remaining` exactly.
fn fill_cage(
    cands: &[Candidates],
    remaining: u8,
    chosen: &mut Vec<u8>,
    possible: &mut [Candidates],
) -> bool {
    let i = chosen.len();
    if i == cands.len() {
        if remaining != 0 {
            return false;
        }
        for (p, &d) in possible.iter_mut().zip(chosen.iter()) {
            p.insert(d);
        }
        return true;
    }
    let mut any = false;
    for d in cands[i].iter() {
        if d > remaining || chosen.contains(&d) {
            continue;
        }
        chosen.push(d);
        any |= fill_cage(cands, remaining - d, chosen, possible);
        chosen.pop();
    }
    any
}
