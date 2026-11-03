use crate::strategies::{
    BoxLineReduction, Fish, HiddenSingle, HiddenSubset, NakedSingle, NakedSubset, PointingPairs,
    YWing,
};
use crate::{
    board::{Board, Candidates, CellId},
    constraint::{build, Constraint, ConstraintSpec},
    house::{standard_houses, House},
    strategy::{Deduction, Explanation, Strategy},
};

/// The rules of one puzzle: its houses, its variant constraints, and the
/// peer table derived from both.
pub struct SolverContext {
    pub houses: Vec<House>,
    pub constraints: Vec<Box<dyn Constraint>>,
    peers: Vec<Vec<CellId>>,
    /// Bit `b` of `peer_mask[a]` is set when cells `a` and `b` are peers.
    peer_mask: Vec<u128>,
    /// Indices into `constraints` for every constraint touching a cell.
    constraints_at: Vec<Vec<usize>>,
}

impl SolverContext {
    /// Standard sudoku: rows, columns and boxes only.
    pub fn standard() -> Self {
        Self::new(&[])
    }

    /// Standard sudoku plus the given variant rules.
    pub fn new(specs: &[ConstraintSpec]) -> Self {
        let mut houses = standard_houses();
        let mut constraints = Vec::new();
        for spec in specs {
            if let Some(c) = build(spec, &mut houses) {
                constraints.push(c);
            }
        }
        Self::from_parts(houses, constraints)
    }

    pub fn from_parts(houses: Vec<House>, constraints: Vec<Box<dyn Constraint>>) -> Self {
        let mut peer_mask = vec![0u128; 81];
        let mut link = |a: CellId, b: CellId| {
            if a != b {
                peer_mask[a.0 as usize] |= 1 << b.0;
                peer_mask[b.0 as usize] |= 1 << a.0;
            }
        };
        for house in &houses {
            for &a in &house.cells {
                for &b in &house.cells {
                    link(a, b);
                }
            }
        }
        for c in &constraints {
            for (a, b) in c.distinct_pairs() {
                link(a, b);
            }
        }

        let peers = peer_mask
            .iter()
            .map(|&mask| {
                (0..81u8)
                    .filter(|&i| mask >> i & 1 == 1)
                    .map(CellId)
                    .collect()
            })
            .collect();

        let mut constraints_at = vec![Vec::new(); 81];
        for (i, c) in constraints.iter().enumerate() {
            for &cell in c.cells() {
                constraints_at[cell.0 as usize].push(i);
            }
        }

        SolverContext {
            houses,
            constraints,
            peers,
            peer_mask,
            constraints_at,
        }
    }

    /// Cells that may not share a digit with `cell`, sorted by index.
    pub fn peers(&self, cell: CellId) -> &[CellId] {
        &self.peers[cell.0 as usize]
    }

    pub fn sees(&self, a: CellId, b: CellId) -> bool {
        self.peer_mask[a.0 as usize] >> b.0 & 1 == 1
    }

    /// Digits that can go at `cell` given the digits in `grid` (0 = empty).
    pub fn legal_digits(&self, grid: &[u8; 81], cell: CellId) -> Candidates {
        let mut legal = Candidates::ALL;
        for p in self.peers(cell) {
            legal.remove(grid[p.0 as usize]);
        }
        for &i in &self.constraints_at[cell.0 as usize] {
            for d in legal.iter() {
                if !self.constraints[i].allows(grid, cell, d) {
                    legal.remove(d);
                }
            }
        }
        legal
    }

    /// Filled cells in `grid` (0 = empty) that break a rule: they repeat a
    /// peer's digit, or a constraint they're on can no longer be satisfied.
    pub fn conflicts(&self, grid: &[u8; 81]) -> Vec<CellId> {
        (0..81u8)
            .map(CellId)
            .filter(|&cell| {
                let digit = grid[cell.0 as usize];
                digit != 0
                    && (self.peers(cell).iter().any(|p| grid[p.0 as usize] == digit)
                        || self.constraints_at[cell.0 as usize]
                            .iter()
                            .any(|&i| !self.constraints[i].allows(grid, cell, digit)))
            })
            .collect()
    }

    /// Strip impossible candidates: every decided cell removes its digit from
    /// its peers, then each constraint runs its own propagation.
    pub fn propagate(&self, board: &mut Board) -> Vec<Deduction> {
        let mut deductions = Vec::new();
        for i in 0..81u8 {
            let source = CellId(i);
            let Some(digit) = board.value(source) else {
                continue;
            };
            let mut eliminations = Vec::new();
            for &peer in self.peers(source) {
                if board.candidates(peer).contains(digit) {
                    board.remove_candidate(peer, digit);
                    eliminations.push((peer, digit));
                }
            }
            if !eliminations.is_empty() {
                deductions.push(Deduction {
                    strategy: "peer elimination",
                    difficulty: 0,
                    placements: vec![],
                    eliminations,
                    explanation: Explanation::peer_elimination(source, digit),
                });
            }
        }
        for c in &self.constraints {
            deductions.extend(c.propagate(board));
        }
        deductions
    }
}

pub enum SolveResult {
    Solved(Vec<Deduction>),
    Stuck(Vec<Deduction>),
    Invalid(Vec<Deduction>),
}

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Solver { strategies }
    }

    /// Find the next deduction without applying it.
    pub fn step(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for strategy in &self.strategies {
            if let Some(deduction) = strategy.find(board, ctx) {
                return Some(deduction);
            }
        }
        None
    }

    /// Propagate (peers, then constraints), then find the next strategy-based deduction.
    /// Propagation mutates the board's candidate sets as a side-effect but
    /// those deductions are not returned — only the strategy deduction is.
    /// Returns `None` if the board is solved (including by propagation) or
    /// has a contradiction or no strategy applies.
    pub fn hint(&self, board: &mut Board, ctx: &SolverContext) -> Option<Deduction> {
        ctx.propagate(board);
        if board.is_solved() || board.has_contradiction() {
            return None;
        }
        self.step(board, ctx)
    }

    /// Solve the puzzle, returning all deductions made (propagation + strategies)
    pub fn solve(&self, board: &mut Board, ctx: &SolverContext) -> SolveResult {
        let mut steps = Vec::new();
        loop {
            // Peer elimination and constraint propagation strip impossible candidates
            steps.extend(ctx.propagate(board));

            if board.has_contradiction() {
                return SolveResult::Invalid(steps);
            }

            if board.is_solved() {
                return SolveResult::Solved(steps);
            }

            // Ask strategies for the next deduction
            match self.step(board, ctx) {
                Some(deduction) => {
                    apply_deduction(board, &deduction);
                    steps.push(deduction);
                }
                None => return SolveResult::Stuck(steps),
            }
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver::new(vec![
            Box::new(NakedSingle),
            Box::new(HiddenSingle),
            Box::new(PointingPairs),
            Box::new(BoxLineReduction),
            Box::new(NakedSubset::<2>),
            Box::new(HiddenSubset::<2>),
            Box::new(NakedSubset::<3>),
            Box::new(HiddenSubset::<3>),
            Box::new(Fish::<2>),
            Box::new(Fish::<3>),
            Box::new(YWing),
        ])
    }
}

fn apply_deduction(board: &mut Board, deduction: &Deduction) {
    for &(cell, digit) in &deduction.eliminations {
        board.remove_candidate(cell, digit);
    }
    for &(cell, digit) in &deduction.placements {
        board.set_value(cell, digit);
    }
}
