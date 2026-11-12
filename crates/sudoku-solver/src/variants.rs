use crate::{
    board::{Board, Candidates, CellId},
    constraint::Constraint,
};

/// A rule between two cells `a` and `b`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Relation {
    /// |a - b| = 1 (hollow dot)
    Consecutive,
    /// one is double the other (filled dot)
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

/// Every unordered pair of `cells`.
fn all_pairs(cells: &[CellId]) -> Vec<(CellId, CellId)> {
    let mut pairs = Vec::new();
    for (i, &a) in cells.iter().enumerate() {
        for &b in &cells[i + 1..] {
            pairs.push((a, b));
        }
    }
    pairs
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
        all_pairs(&self.cells)
    }

    /// Arc consistency: drop any digit with no partner across one of its edges,
    /// repeating until nothing changes.
    fn eliminate(&self, board: &mut Board, eliminations: &mut Vec<(CellId, u8)>) {
        loop {
            let before = eliminations.len();
            for &(a, b, rel) in &self.edges {
                let (ca, cb) = (board.candidates(a), board.candidates(b));
                restrict(board, a, supported(rel, ca, cb), eliminations);
                let ca = board.candidates(a);
                restrict(board, b, supported(rel.flipped(), cb, ca), eliminations);
            }
            if eliminations.len() == before {
                break;
            }
        }
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
        all_pairs(&self.cells)
    }

    /// Keep only digits that appear in some way of filling the whole cage
    /// with different digits that add up to the sum.
    fn eliminate(&self, board: &mut Board, eliminations: &mut Vec<(CellId, u8)>) {
        let Some(sum) = self.sum else {
            return;
        };
        let cands: Vec<Candidates> = self.cells.iter().map(|&c| board.candidates(c)).collect();
        let mut possible = vec![Candidates::EMPTY; self.cells.len()];
        let mut chosen = Vec::with_capacity(self.cells.len());
        fill_cage(&cands, sum, &mut chosen, &mut possible);

        for (&cell, &keep) in self.cells.iter().zip(&possible) {
            restrict(board, cell, keep, eliminations);
        }
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

/// Arrow: the digits along the shaft add up to the digit in the circle.
/// Shaft digits may repeat; the houses rule out repeats where they apply.
pub struct Arrow {
    name: String,
    /// circle first, then the shaft
    cells: Vec<CellId>,
}

impl Arrow {
    pub fn new(cells: Vec<CellId>) -> Self {
        let name = format!("arrow at {}", cells[0]);
        Arrow { name, cells }
    }
}

/// Sums 0..=127 reachable by picking one candidate from each cell, as a bitmask.
fn reachable_sums(cands: &[Candidates]) -> u128 {
    cands
        .iter()
        .fold(1, |sums, c| c.iter().fold(0, |acc, d| acc | sums << d))
}

impl Constraint for Arrow {
    fn name(&self) -> &str {
        &self.name
    }

    fn cells(&self) -> &[CellId] {
        &self.cells
    }

    /// Keep circle digits some shaft total can reach, and shaft digits that
    /// fit in such a total together with some choice for the rest of the shaft.
    fn eliminate(&self, board: &mut Board, eliminations: &mut Vec<(CellId, u8)>) {
        let circle = board.candidates(self.cells[0]);
        let shaft: Vec<Candidates> = self.cells[1..]
            .iter()
            .map(|&c| board.candidates(c))
            .collect();

        let mut circle_sums = 0u128;
        for d in circle.iter() {
            circle_sums |= 1 << d;
        }

        let mut keep_circle = Candidates::EMPTY;
        let totals = reachable_sums(&shaft);
        for d in circle.iter() {
            if totals & (1 << d) != 0 {
                keep_circle.insert(d);
            }
        }
        restrict(board, self.cells[0], keep_circle, eliminations);

        for (i, &cell) in self.cells[1..].iter().enumerate() {
            let others: Vec<Candidates> = shaft
                .iter()
                .enumerate()
                .filter_map(|(j, &c)| (j != i).then_some(c))
                .collect();
            let rest = reachable_sums(&others);
            let mut keep = Candidates::EMPTY;
            for d in shaft[i].iter() {
                if (rest << d) & circle_sums != 0 {
                    keep.insert(d);
                }
            }
            restrict(board, cell, keep, eliminations);
        }
    }

    fn allows(&self, grid: &[u8; 81], cell: CellId, digit: u8) -> bool {
        let value = |c: CellId| if c == cell { digit } else { grid[c.0 as usize] };
        let circle = value(self.cells[0]);
        let mut total = 0u8;
        let mut empty = 0u8;
        for &c in &self.cells[1..] {
            match value(c) {
                0 => empty += 1,
                d => total += d,
            }
        }
        // each empty shaft cell adds 1 to 9
        let (min, max) = (total + empty, total + 9 * empty);
        if circle == 0 {
            min <= 9
        } else {
            min <= circle && circle <= max
        }
    }
}

/// Renban line: different digits that form a run of consecutive digits,
/// in any order.
pub struct Renban {
    name: String,
    cells: Vec<CellId>,
}

impl Renban {
    pub fn new(cells: Vec<CellId>) -> Self {
        let name = format!("renban at {}", cells[0]);
        Renban { name, cells }
    }

    /// The runs `lo..lo + n` of digits the line could hold, as candidate sets.
    fn windows(&self) -> impl Iterator<Item = Candidates> {
        let n = self.cells.len() as u8;
        (1..=10u8.saturating_sub(n)).map(move |lo| {
            let mut window = Candidates::EMPTY;
            for d in lo..lo + n {
                window.insert(d);
            }
            window
        })
    }
}

impl Constraint for Renban {
    fn name(&self) -> &str {
        &self.name
    }

    fn cells(&self) -> &[CellId] {
        &self.cells
    }

    fn distinct_pairs(&self) -> Vec<(CellId, CellId)> {
        all_pairs(&self.cells)
    }

    /// Keep a digit only if it lies in a run the whole line can still fill:
    /// every cell has a candidate in the run, and together they cover it.
    fn eliminate(&self, board: &mut Board, eliminations: &mut Vec<(CellId, u8)>) {
        let cands: Vec<Candidates> = self.cells.iter().map(|&c| board.candidates(c)).collect();
        let mut keep = vec![Candidates::EMPTY; self.cells.len()];
        for window in self.windows() {
            let inside: Vec<Candidates> = cands.iter().map(|c| c.intersect(window)).collect();
            let covered = inside
                .iter()
                .fold(Candidates::EMPTY, |acc, &c| acc.union(c));
            if inside.iter().all(|c| c.count() > 0) && covered == window {
                for (k, c) in keep.iter_mut().zip(&inside) {
                    *k = k.union(*c);
                }
            }
        }
        for (&cell, &keep) in self.cells.iter().zip(&keep) {
            restrict(board, cell, keep, eliminations);
        }
    }

    fn allows(&self, grid: &[u8; 81], cell: CellId, digit: u8) -> bool {
        let (mut min, mut max) = (digit, digit);
        for &c in &self.cells {
            match grid[c.0 as usize] {
                _ if c == cell => {}
                0 => {}
                d => {
                    min = min.min(d);
                    max = max.max(d);
                }
            }
        }
        usize::from(max - min) < self.cells.len()
    }
}
