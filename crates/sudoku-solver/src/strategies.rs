use crate::{
    board::{Board, Candidates, CellId},
    constraint::{GroupKind, UniqueGroup},
    solver::SolverContext,
    strategy::{Deduction, Explanation, Strategy},
};

/// Call `f` with each N-sized combination of indices `0..len`.
/// Returns early with `Some(d)` the first time `f` returns `Some`.
fn find_in_combinations<F>(len: usize, n: usize, mut f: F) -> Option<Deduction>
where
    F: FnMut(&[usize]) -> Option<Deduction>,
{
    fn recurse<F>(
        len: usize,
        n: usize,
        start: usize,
        buf: &mut Vec<usize>,
        f: &mut F,
    ) -> Option<Deduction>
    where
        F: FnMut(&[usize]) -> Option<Deduction>,
    {
        if buf.len() == n {
            return f(buf);
        }
        for i in start..len {
            buf.push(i);
            if let Some(d) = recurse(len, n, i + 1, buf, f) {
                return Some(d);
            }
            buf.pop();
        }
        None
    }
    let mut buf = Vec::with_capacity(n);
    recurse(len, n, 0, &mut buf, &mut f)
}

pub struct NakedSingle;

impl Strategy for NakedSingle {
    fn name(&self) -> &'static str { "naked single" }
    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for i in 0..81 {
            let id = CellId(i);
            let candidates = board.candidates(id);

            if candidates.count() == 1 && !board.is_decided(id) {
                let digit = candidates.iter().next().unwrap();
                return Some(Deduction {
                    strategy: "naked single",
                    difficulty: 1,
                    placements: vec![(id, digit)],
                    eliminations: vec![],
                    explanation: Explanation::naked_single(id, digit),
                })
            }
        }

        None
    }
}

pub struct HiddenSingle;

impl Strategy for HiddenSingle {
    fn name(&self) -> &'static str { "hidden single" }
    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for constraint in ctx.constraints {
            let cells = constraint.cells();
            for digit in 1..=9u8 {
                let mut found: Option<CellId> = None;
                let mut count = 0u8;
                for &cell in cells {
                    if board.candidates(cell).contains(digit) {
                        count += 1;
                        if count > 1 { break; }
                        found = Some(cell);
                    }
                }
                if count == 1 {
                    let cell = found.unwrap();
                    if !board.is_decided(cell) {
                        return Some(Deduction {
                            strategy: "hidden single",
                            difficulty: 2,
                            placements: vec![(cell, digit)],
                            eliminations: vec![],
                            explanation: Explanation::hidden_single(cell, digit),
                        });
                    }
                }
            }
        }
        None
    }
}

/// Pointing pair / triple: when all candidates for a digit inside a box are
/// confined to a single row or column, eliminate that digit from the rest of
/// that row or column outside the box.
pub struct PointingPairs;

impl Strategy for PointingPairs {
    fn name(&self) -> &'static str { "pointing pair" }
    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for b in 0..9u8 {
            let box_group = UniqueGroup::box_(b);
            for digit in 1..=9u8 {
                let cells: Vec<CellId> = box_group
                    .cells
                    .iter()
                    .copied()
                    .filter(|c| board.candidates(*c).contains(digit))
                    .collect();
                if cells.len() < 2 || cells.len() > 3 { continue; }

                let r0 = cells[0].row();
                if cells.iter().all(|c| c.row() == r0) {
                    if let Some(d) = pointing_eliminations(
                        board, b, digit, &cells, GroupKind::Row(r0),
                    ) {
                        return Some(d);
                    }
                }

                let c0 = cells[0].col();
                if cells.iter().all(|c| c.col() == c0) {
                    if let Some(d) = pointing_eliminations(
                        board, b, digit, &cells, GroupKind::Col(c0),
                    ) {
                        return Some(d);
                    }
                }
            }
        }
        None
    }
}

fn pointing_eliminations(
    board: &Board,
    box_idx: u8,
    digit: u8,
    locked: &[CellId],
    line: GroupKind,
) -> Option<Deduction> {
    let line_cells = match line {
        GroupKind::Row(r) => UniqueGroup::row(r).cells,
        GroupKind::Col(c) => UniqueGroup::col(c).cells,
        GroupKind::Box(_) => return None,
    };

    let eliminations: Vec<(CellId, u8)> = line_cells
        .into_iter()
        .filter(|c| c.box_idx() != box_idx)
        .filter(|c| board.candidates(*c).contains(digit))
        .map(|c| (c, digit))
        .collect();

    if eliminations.is_empty() { return None; }

    Some(Deduction {
        strategy: "pointing pair",
        difficulty: 3,
        placements: vec![],
        eliminations,
        explanation: Explanation::pointing(box_idx, digit, line, locked),
    })
}

/// Box/line reduction: when all candidates for a digit inside a row or column
/// are confined to a single box, eliminate that digit from the rest of that
/// box (the cells outside the row or column).
pub struct BoxLineReduction;

impl Strategy for BoxLineReduction {
    fn name(&self) -> &'static str { "box/line reduction" }
    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for r in 0..9u8 {
            if let Some(d) = box_line_for_line(board, GroupKind::Row(r)) {
                return Some(d);
            }
        }
        for c in 0..9u8 {
            if let Some(d) = box_line_for_line(board, GroupKind::Col(c)) {
                return Some(d);
            }
        }
        None
    }
}

fn box_line_for_line(board: &Board, line: GroupKind) -> Option<Deduction> {
    let line_cells = match line {
        GroupKind::Row(r) => UniqueGroup::row(r).cells,
        GroupKind::Col(c) => UniqueGroup::col(c).cells,
        GroupKind::Box(_) => return None,
    };

    for digit in 1..=9u8 {
        let cells: Vec<CellId> = line_cells
            .iter()
            .copied()
            .filter(|c| board.candidates(*c).contains(digit))
            .collect();
        if cells.len() < 2 || cells.len() > 3 { continue; }

        let b0 = cells[0].box_idx();
        if !cells.iter().all(|c| c.box_idx() == b0) { continue; }

        let box_cells = UniqueGroup::box_(b0).cells;
        let line_set: std::collections::HashSet<CellId> = line_cells.iter().copied().collect();
        let eliminations: Vec<(CellId, u8)> = box_cells
            .into_iter()
            .filter(|c| !line_set.contains(c))
            .filter(|c| board.candidates(*c).contains(digit))
            .map(|c| (c, digit))
            .collect();

        if eliminations.is_empty() { continue; }

        return Some(Deduction {
            strategy: "box/line reduction",
            difficulty: 3,
            placements: vec![],
            eliminations,
            explanation: Explanation::box_line_reduction(b0, digit, line, &cells),
        });
    }
    None
}

pub struct NakedSubset<const N: usize>;

impl<const N: usize> Strategy for NakedSubset<N> {
    fn name(&self) -> &'static str {
        match N {
            2 => "naked pair",
            3 => "naked triple",
            _ => "naked subset",
        }
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for constraint in ctx.constraints {
            let all_cells = constraint.cells();

            // Only cells that could form an N-subset (at most N candidates)
            let subset_cells: Vec<CellId> = all_cells
                .iter()
                .copied()
                .filter(|&c| !board.is_decided(c))
                .filter(|&c| board.candidates(c).count() as usize <= N)
                .collect();

            if subset_cells.len() < N {
                continue;
            }

            let result = find_in_combinations(subset_cells.len(), N, |indices| {
                let subset: Vec<CellId> = indices.iter().map(|&i| subset_cells[i]).collect();
                let union = subset
                    .iter()
                    .fold(Candidates::EMPTY, |acc, &c| acc.union(board.candidates(c)));

                if union.count() as usize != N {
                    return None;
                }

                let eliminations: Vec<(CellId, u8)> = all_cells
                    .iter()
                    .filter(|c| !board.is_decided(**c) && !subset.contains(c))
                    .flat_map(|&c| {
                        union
                            .iter()
                            .filter(move |&d| board.candidates(c).contains(d))
                            .map(move |d| (c, d))
                    })
                    .collect();

                if eliminations.is_empty() {
                    return None;
                }

                let digits: Vec<u8> = union.iter().collect();
                Some(Deduction {
                    strategy: match N {
                        2 => "naked pair",
                        3 => "naked triple",
                        _ => "naked subset",
                    },
                    difficulty: if N == 2 { 4 } else { 5 },
                    placements: vec![],
                    eliminations,
                    explanation: Explanation::naked_subset(&subset, &digits),
                })
            });

            if result.is_some() {
                return result;
            }
        }
        None
    }
}

// hidden pairs
// hidden triples

// x-wing
// y-wing
// swordfish