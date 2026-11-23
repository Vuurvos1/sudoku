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
    fn difficulty(&self) -> u8 {
        1
    }

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
                    strategy: self.name(),
                    difficulty: self.difficulty(),
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
    fn difficulty(&self) -> u8 {
        2
    }

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
                            strategy: self.name(),
                            difficulty: self.difficulty(),
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
    fn difficulty(&self) -> u8 {
        3
    }

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
        strategy: PointingPairs.name(),
        difficulty: PointingPairs.difficulty(),
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
    fn difficulty(&self) -> u8 {
        3
    }

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
            strategy: BoxLineReduction.name(),
            difficulty: BoxLineReduction.difficulty(),
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
    fn difficulty(&self) -> u8 {
        if N == 2 {
            4
        } else {
            5
        }
    }

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
                    difficulty: self.difficulty(),
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
    fn difficulty(&self) -> u8 {
        if N == 2 {
            4
        } else {
            5
        }
    }

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
                    difficulty: self.difficulty(),
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
    fn difficulty(&self) -> u8 {
        match N {
            2 => 6,
            3 => 7,
            _ => 9,
        }
    }

    fn name(&self) -> &'static str {
        match N {
            2 => "x-wing",
            3 => "swordfish",
            4 => "jellyfish",
            _ => "fish",
        }
    }

    fn find(&self, board: &Board, _ctx: &SolverContext) -> Option<Deduction> {
        for digit in 1..=9u8 {
            // Try rows as base, columns as cover
            if let Some(d) = find_fish(board, digit, N, true, self.name(), self.difficulty()) {
                return Some(d);
            }
            // Try columns as base, rows as cover
            if let Some(d) = find_fish(board, digit, N, false, self.name(), self.difficulty()) {
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
    difficulty: u8,
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
            difficulty,
            placements: vec![],
            eliminations,
            explanation: Explanation::fish(&locked, digit, &base_names, &cover_names),
        })
    })
}

pub struct YWing;

impl Strategy for YWing {
    fn difficulty(&self) -> u8 {
        6
    }

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
                    let eliminations = seen_by_all(board, ctx, c, &[w1, w2]);

                    if !eliminations.is_empty() {
                        return Some(Deduction {
                            strategy: self.name(),
                            difficulty: self.difficulty(),
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

/// Whether `cell` is still open and could hold `digit`.
fn could_be(board: &Board, cell: CellId, digit: u8) -> bool {
    !board.is_decided(cell) && board.candidates(cell).contains(digit)
}

/// Open cells with exactly `count` candidates.
fn cells_with_count(board: &Board, count: u32) -> impl Iterator<Item = CellId> + '_ {
    (0..81u8)
        .map(CellId)
        .filter(move |&c| !board.is_decided(c) && board.candidates(c).count() == count)
}

/// `digit` eliminated from every open cell that sees all of `sources`.
fn seen_by_all(
    board: &Board,
    ctx: &SolverContext,
    digit: u8,
    sources: &[CellId],
) -> Vec<(CellId, u8)> {
    ctx.peers(sources[0])
        .iter()
        .copied()
        .filter(|&c| could_be(board, c, digit) && sources[1..].iter().all(|&s| ctx.sees(c, s)))
        .map(|c| (c, digit))
        .collect()
}

/// Conjugate pairs on `digit`: for each cell, the other spot of every house
/// where the digit has exactly two. One of the two must hold it.
fn conjugate_links(board: &Board, ctx: &SolverContext, digit: u8) -> Vec<Vec<CellId>> {
    let mut links = vec![vec![]; 81];
    for house in &ctx.houses {
        let mut spots = house
            .cells
            .iter()
            .copied()
            .filter(|&c| could_be(board, c, digit));
        if let (Some(a), Some(b), None) = (spots.next(), spots.next(), spots.next()) {
            links[a.0 as usize].push(b);
            links[b.0 as usize].push(a);
        }
    }
    links
}

/// Breadth-first search over chain states numbered `0..states`, so the
/// shortest chain wins. `next` adds the states one link on; `found` gets a
/// state and its chain length in links, and ends the search with a result.
/// Returns the states along the chain with that result.
fn shortest_chain<T>(
    first: usize,
    states: usize,
    mut next: impl FnMut(usize, &mut Vec<usize>),
    mut found: impl FnMut(usize, usize) -> Option<T>,
) -> Option<(Vec<usize>, T)> {
    const UNSEEN: u16 = u16::MAX;
    let mut parent = vec![UNSEEN; states];
    parent[first] = first as u16;
    let mut queue = std::collections::VecDeque::from([(first, 0)]);
    let mut buf = Vec::new();

    while let Some((state, links)) = queue.pop_front() {
        if let Some(result) = found(state, links) {
            let mut path = vec![state];
            while *path.last().unwrap() != first {
                path.push(parent[*path.last().unwrap()] as usize);
            }
            path.reverse();
            return Some((path, result));
        }
        buf.clear();
        next(state, &mut buf);
        for &n in &buf {
            if parent[n] == UNSEEN {
                parent[n] = state as u16;
                queue.push_back((n, links + 1));
            }
        }
    }
    None
}

/// Simple coloring: follow a digit's conjugate pairs and color the chain in
/// two alternating colors. Exactly one color holds the digit.
pub struct SimpleColoring;

impl Strategy for SimpleColoring {
    fn difficulty(&self) -> u8 {
        7
    }

    fn name(&self) -> &'static str {
        "simple coloring"
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for digit in 1..=9u8 {
            let links = conjugate_links(board, ctx, digit);
            let mut color: [Option<bool>; 81] = [None; 81];

            for start in (0..81u8).map(CellId) {
                if color[start.0 as usize].is_some() || links[start.0 as usize].is_empty() {
                    continue;
                }
                let Some([on, off]) = color_chain(start, &links, &mut color) else {
                    continue;
                };
                // A single pair is not a chain
                if on.len() + off.len() < 3 {
                    continue;
                }
                let deduction = |eliminations, wrap, [a, b]: [&[CellId]; 2]| Deduction {
                    strategy: self.name(),
                    difficulty: self.difficulty(),
                    placements: vec![],
                    eliminations,
                    explanation: Explanation::simple_coloring(digit, a, b, wrap),
                };

                // Color wrap: two cells of one color see each other, so that
                // color is false everywhere
                for (wrong, right) in [(&on, &off), (&off, &on)] {
                    let sees_itself = wrong
                        .iter()
                        .enumerate()
                        .any(|(i, &a)| wrong[i + 1..].iter().any(|&b| ctx.sees(a, b)));
                    if sees_itself {
                        let eliminations = wrong.iter().map(|&c| (c, digit)).collect();
                        return Some(deduction(eliminations, true, [wrong, right]));
                    }
                }

                // Color trap: a cell that sees both colors can't hold the digit.
                // No chain cell does, or its color would have wrapped.
                let eliminations: Vec<(CellId, u8)> = (0..81u8)
                    .map(CellId)
                    .filter(|&c| could_be(board, c, digit))
                    .filter(|&c| on.iter().any(|&a| ctx.sees(c, a)))
                    .filter(|&c| off.iter().any(|&b| ctx.sees(c, b)))
                    .map(|c| (c, digit))
                    .collect();
                if !eliminations.is_empty() {
                    return Some(deduction(eliminations, false, [&on, &off]));
                }
            }
        }
        None
    }
}

/// Color every cell linked to `start`, alternating, and return both colors
/// sorted. `None` when a cell would need both colors, which only happens on
/// a board with a wrong digit.
fn color_chain(
    start: CellId,
    links: &[Vec<CellId>],
    color: &mut [Option<bool>; 81],
) -> Option<[Vec<CellId>; 2]> {
    let mut sides = [vec![start], vec![]];
    color[start.0 as usize] = Some(true);
    let mut todo = vec![start];
    let mut consistent = true;

    while let Some(cell) = todo.pop() {
        let next = !color[cell.0 as usize].unwrap();
        for &linked in &links[cell.0 as usize] {
            match color[linked.0 as usize] {
                None => {
                    color[linked.0 as usize] = Some(next);
                    sides[usize::from(!next)].push(linked);
                    todo.push(linked);
                }
                Some(c) => consistent &= c == next,
            }
        }
    }
    for side in &mut sides {
        side.sort_by_key(|c| c.0);
    }
    consistent.then_some(sides)
}

pub struct XyzWing;

impl Strategy for XyzWing {
    fn difficulty(&self) -> u8 {
        8
    }

    fn name(&self) -> &'static str {
        "xyz-wing"
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        let bivalues: Vec<CellId> = cells_with_count(board, 2).collect();

        for pivot in cells_with_count(board, 3) {
            let pivot_cands = board.candidates(pivot);
            // Wings see the pivot and hold two of its three digits
            let wings: Vec<CellId> = bivalues
                .iter()
                .copied()
                .filter(|&w| ctx.sees(pivot, w) && board.candidates(w).is_subset_of(pivot_cands))
                .collect();

            for (i, &wing1) in wings.iter().enumerate() {
                for &wing2 in &wings[i + 1..] {
                    let (c1, c2) = (board.candidates(wing1), board.candidates(wing2));
                    if c1 == c2 {
                        continue;
                    }
                    // Two different pairs out of three digits share exactly one: Z
                    let z = c1.intersect(c2).iter().next().unwrap();

                    let eliminations = seen_by_all(board, ctx, z, &[pivot, wing1, wing2]);
                    if !eliminations.is_empty() {
                        return Some(Deduction {
                            strategy: self.name(),
                            difficulty: self.difficulty(),
                            placements: vec![],
                            eliminations,
                            explanation: Explanation::xyz_wing(pivot, wing1, wing2, pivot_cands, z),
                        });
                    }
                }
            }
        }
        None
    }
}

/// X-Cycle: a chain on one digit that alternates strong links (conjugate
/// pairs) and weak links (cells that see each other). Starting with "A is
/// not the digit", every strong link turns a cell on and every weak link
/// turns the next one off. When the chain reaches a cell B that is on, A or
/// B holds the digit, so cells seeing both can't.
pub struct XCycle;

impl Strategy for XCycle {
    fn difficulty(&self) -> u8 {
        9
    }

    fn name(&self) -> &'static str {
        "x-cycle"
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        for digit in 1..=9u8 {
            let links = conjugate_links(board, ctx, digit);

            for start in (0..81u8).map(CellId) {
                if links[start.0 as usize].is_empty() {
                    continue;
                }
                // state = cell * 2, plus 1 when the cell holds the digit
                let chain = shortest_chain(
                    start.0 as usize * 2,
                    162,
                    |state, next| {
                        let cell = CellId((state / 2) as u8);
                        if state % 2 == 0 {
                            next.extend(
                                links[cell.0 as usize].iter().map(|c| c.0 as usize * 2 + 1),
                            );
                        } else {
                            next.extend(
                                ctx.peers(cell)
                                    .iter()
                                    .filter(|&&c| could_be(board, c, digit))
                                    .map(|c| c.0 as usize * 2),
                            );
                        }
                    },
                    |state, links| {
                        let cell = CellId((state / 2) as u8);
                        if state % 2 == 0 || links < 3 || cell == start {
                            return None;
                        }
                        Some(seen_by_all(board, ctx, digit, &[start, cell]))
                            .filter(|e| !e.is_empty())
                    },
                );

                if let Some((states, eliminations)) = chain {
                    let cells: Vec<CellId> = states.iter().map(|s| CellId((s / 2) as u8)).collect();
                    return Some(Deduction {
                        strategy: self.name(),
                        difficulty: self.difficulty(),
                        placements: vec![],
                        eliminations,
                        explanation: Explanation::x_cycle(digit, &cells),
                    });
                }
            }
        }
        None
    }
}

/// XY-Chain: a chain of two-candidate cells, each seeing the next and
/// sharing a digit with it. If the first cell isn't Z, each cell is forced
/// in turn until the last one is Z, so cells seeing both ends can't be Z.
/// A Y-Wing is the shortest XY-Chain.
pub struct XyChain;

impl Strategy for XyChain {
    fn difficulty(&self) -> u8 {
        9
    }

    fn name(&self) -> &'static str {
        "xy-chain"
    }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        let pair = |c: CellId| {
            let cands = board.candidates(c);
            (!board.is_decided(c) && cands.count() == 2).then_some(cands)
        };
        // the digit a two-candidate cell is left with once `d` is ruled out
        let other = |cands: Candidates, d: u8| cands.iter().find(|&x| x != d).unwrap();

        for start in cells_with_count(board, 2) {
            let start_cands = board.candidates(start);
            for z in start_cands.iter() {
                // state = cell * 10 + the digit the cell is forced to be
                let chain = shortest_chain(
                    start.0 as usize * 10 + other(start_cands, z) as usize,
                    810,
                    |state, next| {
                        let (cell, forced) = (CellId((state / 10) as u8), (state % 10) as u8);
                        // being `forced` rules it out of the cells this one sees
                        next.extend(ctx.peers(cell).iter().filter_map(|&n| {
                            let cands = pair(n).filter(|c| c.contains(forced))?;
                            Some(n.0 as usize * 10 + other(cands, forced) as usize)
                        }));
                    },
                    |state, links| {
                        let cell = CellId((state / 10) as u8);
                        if state % 10 != z as usize || links < 2 || cell == start {
                            return None;
                        }
                        Some(seen_by_all(board, ctx, z, &[start, cell])).filter(|e| !e.is_empty())
                    },
                );

                if let Some((states, eliminations)) = chain {
                    let cells: Vec<CellId> =
                        states.iter().map(|s| CellId((s / 10) as u8)).collect();
                    return Some(Deduction {
                        strategy: self.name(),
                        difficulty: self.difficulty(),
                        placements: vec![],
                        eliminations,
                        explanation: Explanation::xy_chain(z, &cells),
                    });
                }
            }
        }
        None
    }
}
