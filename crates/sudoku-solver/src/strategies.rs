use crate::{
    board::{Board, Candidates, CellId},
    house::{House, HouseKind},
    solver::SolverContext,
    strategy::{Deduction, Explanation, Strategy},
};

/// Call `f` with each N-sized combination of indices `0..len`.
/// Returns early with `Some(d)` the first time `f` returns `Some`.
fn find_in_combinations<T, F>(len: usize, n: usize, mut f: F) -> Option<T>
where
    F: FnMut(&[usize]) -> Option<T>,
{
    fn recurse<T, F>(
        len: usize,
        n: usize,
        start: usize,
        buf: &mut Vec<usize>,
        f: &mut F,
    ) -> Option<T>
    where
        F: FnMut(&[usize]) -> Option<T>,
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
    fn name(&self) -> &'static str {
        "naked single"
    }
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
                });
            }
        }

        None
    }
}

pub struct HiddenSingle;

impl Strategy for HiddenSingle {
    fn name(&self) -> &'static str {
        "hidden single"
    }
    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for house in &ctx.houses {
            let cells = &house.cells;
            for digit in 1..=9u8 {
                let mut found: Option<CellId> = None;
                let mut count = 0u8;
                for &cell in cells {
                    if board.candidates(cell).contains(digit) {
                        count += 1;
                        if count > 1 {
                            break;
                        }
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
                            explanation: Explanation::hidden_single(cell, digit, &house.name),
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
    fn name(&self) -> &'static str {
        "pointing pair"
    }
    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for b in 0..9u8 {
            let box_house = House::box_(b);
            for digit in 1..=9u8 {
                let cells: Vec<CellId> = box_house
                    .cells
                    .iter()
                    .copied()
                    .filter(|c| board.candidates(*c).contains(digit))
                    .collect();
                if cells.len() < 2 || cells.len() > 3 {
                    continue;
                }

                let r0 = cells[0].row();
                if cells.iter().all(|c| c.row() == r0) {
                    if let Some(d) =
                        pointing_eliminations(board, b, digit, &cells, HouseKind::Row(r0))
                    {
                        return Some(d);
                    }
                }

                let c0 = cells[0].col();
                if cells.iter().all(|c| c.col() == c0) {
                    if let Some(d) =
                        pointing_eliminations(board, b, digit, &cells, HouseKind::Col(c0))
                    {
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
    line: HouseKind,
) -> Option<Deduction> {
    let line_cells = match line {
        HouseKind::Row(r) => House::row(r).cells,
        HouseKind::Col(c) => House::col(c).cells,
        HouseKind::Box(_) | HouseKind::Diagonal(_) => return None,
    };

    let eliminations: Vec<(CellId, u8)> = line_cells
        .into_iter()
        .filter(|c| c.box_idx() != box_idx)
        .filter(|c| board.candidates(*c).contains(digit))
        .map(|c| (c, digit))
        .collect();

    if eliminations.is_empty() {
        return None;
    }

    Some(Deduction {
        strategy: "pointing pair",
        difficulty: 3,
        placements: vec![],
        eliminations,
        explanation: Explanation::pointing(
            &HouseKind::Box(box_idx).to_string(),
            digit,
            &line.to_string(),
            locked,
        ),
    })
}

/// Box/line reduction: when all candidates for a digit inside a row or column
/// are confined to a single box, eliminate that digit from the rest of that
/// box (the cells outside the row or column).
pub struct BoxLineReduction;

impl Strategy for BoxLineReduction {
    fn name(&self) -> &'static str {
        "box/line reduction"
    }
    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for r in 0..9u8 {
            if let Some(d) = box_line_for_line(board, HouseKind::Row(r)) {
                return Some(d);
            }
        }
        for c in 0..9u8 {
            if let Some(d) = box_line_for_line(board, HouseKind::Col(c)) {
                return Some(d);
            }
        }
        None
    }
}

fn box_line_for_line(board: &Board, line: HouseKind) -> Option<Deduction> {
    let line_cells = match line {
        HouseKind::Row(r) => House::row(r).cells,
        HouseKind::Col(c) => House::col(c).cells,
        HouseKind::Box(_) | HouseKind::Diagonal(_) => return None,
    };

    for digit in 1..=9u8 {
        let cells: Vec<CellId> = line_cells
            .iter()
            .copied()
            .filter(|c| board.candidates(*c).contains(digit))
            .collect();
        if cells.len() < 2 || cells.len() > 3 {
            continue;
        }

        let b0 = cells[0].box_idx();
        if !cells.iter().all(|c| c.box_idx() == b0) {
            continue;
        }

        let box_cells = House::box_(b0).cells;
        let line_set: std::collections::HashSet<CellId> = line_cells.iter().copied().collect();
        let eliminations: Vec<(CellId, u8)> = box_cells
            .into_iter()
            .filter(|c| !line_set.contains(c))
            .filter(|c| board.candidates(*c).contains(digit))
            .map(|c| (c, digit))
            .collect();

        if eliminations.is_empty() {
            continue;
        }

        return Some(Deduction {
            strategy: "box/line reduction",
            difficulty: 3,
            placements: vec![],
            eliminations,
            explanation: Explanation::box_line_reduction(
                &HouseKind::Box(b0).to_string(),
                digit,
                &line.to_string(),
                &cells,
            ),
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
        for house in &ctx.houses {
            let all_cells = &house.cells;

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
                    strategy: self.name(),
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

pub struct HiddenSubset<const N: usize>;

impl<const N: usize> Strategy for HiddenSubset<N> {
    fn name(&self) -> &'static str {
        match N {
            2 => "hidden pair",
            3 => "hidden triple",
            _ => "hidden subset",
        }
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for house in &ctx.houses {
            let undecided: Vec<CellId> = house
                .cells
                .iter()
                .copied()
                .filter(|&c| !board.is_decided(c))
                .collect();

            // Map each digit to the undecided cells that contain it
            let mut digit_cells: [Vec<CellId>; 10] = std::array::from_fn(|_| Vec::new());
            for &cell in &undecided {
                for d in board.candidates(cell).iter() {
                    digit_cells[d as usize].push(cell);
                }
            }

            // Only consider digits present in 1..=N cells
            let active_digits: Vec<u8> = (1..=9u8)
                .filter(|&d| {
                    let count = digit_cells[d as usize].len();
                    count > 0 && count <= N
                })
                .collect();

            if active_digits.len() < N {
                continue;
            }

            let result = find_in_combinations(active_digits.len(), N, |indices| {
                let digits: Vec<u8> = indices.iter().map(|&i| active_digits[i]).collect();

                // Collect unique cells that contain any of these digits
                let mut cells_set: Vec<CellId> = Vec::new();
                for &d in &digits {
                    for &cell in &digit_cells[d as usize] {
                        if !cells_set.contains(&cell) {
                            cells_set.push(cell);
                        }
                    }
                }

                if cells_set.len() != N {
                    return None;
                }

                // Sort for deterministic elimination order
                cells_set.sort_by_key(|c| c.0);

                // Build a candidate mask for the hidden digits
                let hidden = digits.iter().fold(Candidates::EMPTY, |acc, &d| {
                    acc.union(Candidates::single(d))
                });

                // Eliminate all non-hidden candidates from the N cells
                let eliminations: Vec<(CellId, u8)> = cells_set
                    .iter()
                    .flat_map(|&c| {
                        board
                            .candidates(c)
                            .iter()
                            .filter(move |&d| !hidden.contains(d))
                            .map(move |d| (c, d))
                    })
                    .collect();

                if eliminations.is_empty() {
                    return None;
                }

                Some(Deduction {
                    strategy: self.name(),
                    difficulty: if N == 2 { 4 } else { 5 },
                    placements: vec![],
                    eliminations,
                    explanation: Explanation::hidden_subset(&cells_set, &digits),
                })
            });

            if result.is_some() {
                return result;
            }
        }
        None
    }
}

pub struct Fish<const N: usize>;

impl<const N: usize> Strategy for Fish<N> {
    fn name(&self) -> &'static str {
        match N {
            2 => "x-wing",
            3 => "swordfish",
            _ => "fish",
        }
    }

    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for digit in 1..=9u8 {
            // Try rows as base, columns as cover
            if let Some(d) = find_fish(board, digit, N, true, self.name()) {
                return Some(d);
            }
            // Try columns as base, rows as cover
            if let Some(d) = find_fish(board, digit, N, false, self.name()) {
                return Some(d);
            }
        }
        None
    }
}

fn find_fish(
    board: &Board,
    digit: u8,
    n: usize,
    rows_as_base: bool,
    strategy: &'static str,
) -> Option<Deduction> {
    // For each base line, collect which cover positions contain the digit
    let mut base_covers: Vec<(u8, Vec<u8>)> = Vec::new();

    for base in 0..9u8 {
        let covers: Vec<u8> = (0..9u8)
            .filter(|&cover| {
                let cell = if rows_as_base {
                    CellId::from_rc(base, cover)
                } else {
                    CellId::from_rc(cover, base)
                };
                !board.is_decided(cell) && board.candidates(cell).contains(digit)
            })
            .collect();

        if covers.len() >= 2 && covers.len() <= n {
            base_covers.push((base, covers));
        }
    }

    if base_covers.len() < n {
        return None;
    }

    find_in_combinations(base_covers.len(), n, |indices| {
        // Union of cover positions
        let mut cover_union: Vec<u8> = Vec::new();
        for &i in indices {
            for &c in &base_covers[i].1 {
                if !cover_union.contains(&c) {
                    cover_union.push(c);
                }
            }
        }

        if cover_union.len() != n {
            return None;
        }

        let base_set: Vec<u8> = indices.iter().map(|&i| base_covers[i].0).collect();

        // Eliminate digit from cover lines outside the base lines
        let eliminations: Vec<(CellId, u8)> = cover_union
            .iter()
            .flat_map(|&cover| {
                (0..9u8)
                    .filter(|other| !base_set.contains(other))
                    .filter_map(|other| {
                        let cell = if rows_as_base {
                            CellId::from_rc(other, cover)
                        } else {
                            CellId::from_rc(cover, other)
                        };
                        if !board.is_decided(cell) && board.candidates(cell).contains(digit) {
                            Some((cell, digit))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        if eliminations.is_empty() {
            return None;
        }

        // Locked cells: cells in the fish pattern
        let locked: Vec<CellId> = indices
            .iter()
            .flat_map(|&i| {
                let base = base_covers[i].0;
                base_covers[i].1.iter().map(move |&cover| {
                    if rows_as_base {
                        CellId::from_rc(base, cover)
                    } else {
                        CellId::from_rc(cover, base)
                    }
                })
            })
            .collect();

        let base_names: Vec<String> = base_set
            .iter()
            .map(|&b| {
                if rows_as_base {
                    format!("row {}", b + 1)
                } else {
                    format!("column {}", b + 1)
                }
            })
            .collect();
        let cover_names: Vec<String> = cover_union
            .iter()
            .map(|&c| {
                if rows_as_base {
                    format!("column {}", c + 1)
                } else {
                    format!("row {}", c + 1)
                }
            })
            .collect();

        Some(Deduction {
            strategy,
            difficulty: if n == 2 { 6 } else { 7 },
            placements: vec![],
            eliminations,
            explanation: Explanation::fish(&locked, digit, &base_names, &cover_names),
        })
    })
}

pub struct YWing;

impl Strategy for YWing {
    fn name(&self) -> &'static str {
        "y-wing"
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        // Collect bivalue cells (exactly 2 candidates)
        let bivalues: Vec<(CellId, u8, u8)> = (0..81u8)
            .filter_map(|i| {
                let id = CellId(i);
                if board.is_decided(id) {
                    return None;
                }
                let cands = board.candidates(id);
                if cands.count() != 2 {
                    return None;
                }
                let mut iter = cands.iter();
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();
                Some((id, a, b))
            })
            .collect();

        for &(pivot, a, b) in &bivalues {
            // Find wing1: shares candidate 'a' with pivot, has {a, c} where c != b
            for &(w1, w1_x, w1_y) in &bivalues {
                if w1 == pivot {
                    continue;
                }
                if !ctx.sees(pivot, w1) {
                    continue;
                }

                let c = if w1_x == a && w1_y != b {
                    w1_y
                } else if w1_y == a && w1_x != b {
                    w1_x
                } else {
                    continue;
                };

                // Find wing2: shares candidate 'b' with pivot, has {b, c}
                for &(w2, w2_x, w2_y) in &bivalues {
                    if w2 == pivot || w2 == w1 {
                        continue;
                    }
                    if !ctx.sees(pivot, w2) {
                        continue;
                    }
                    if !((w2_x == b && w2_y == c) || (w2_y == b && w2_x == c)) {
                        continue;
                    }

                    // Eliminate c from cells seeing both wings
                    let eliminations: Vec<(CellId, u8)> = (0..81u8)
                        .filter_map(|i| {
                            let cell = CellId(i);
                            if cell == pivot || cell == w1 || cell == w2 {
                                return None;
                            }
                            if board.is_decided(cell) {
                                return None;
                            }
                            if !board.candidates(cell).contains(c) {
                                return None;
                            }
                            if !ctx.sees(cell, w1) {
                                return None;
                            }
                            if !ctx.sees(cell, w2) {
                                return None;
                            }
                            Some((cell, c))
                        })
                        .collect();

                    if !eliminations.is_empty() {
                        return Some(Deduction {
                            strategy: self.name(),
                            difficulty: 6,
                            placements: vec![],
                            eliminations,
                            explanation: Explanation::y_wing(pivot, w1, w2, a, b, c),
                        });
                    }
                }
            }
        }
        None
    }
}
