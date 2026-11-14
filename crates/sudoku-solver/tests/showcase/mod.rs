//! A hand-made test puzzle that uses every variant marker.

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use sudoku_solver::{
    board::{Board, CellId},
    constraint::{ConstraintSpec, LineKind, MarkerKind},
    generator::count_solutions_for,
    solver::{SolveResult, Solver, SolverContext},
};

/// The solution every constraint below was read off.
pub const SOLUTION: [u8; 81] = [
    5, 3, 4, 6, 7, 8, 9, 1, 2, //
    6, 7, 2, 1, 9, 5, 3, 4, 8, //
    1, 9, 8, 3, 4, 2, 5, 6, 7, //
    8, 5, 9, 7, 6, 1, 4, 2, 3, //
    4, 2, 6, 8, 5, 3, 7, 9, 1, //
    7, 1, 3, 9, 2, 4, 8, 5, 6, //
    9, 6, 1, 5, 3, 7, 2, 8, 4, //
    2, 8, 7, 4, 1, 9, 6, 3, 5, //
    3, 4, 5, 2, 8, 6, 1, 7, 9, //
];

fn at(r: u8, c: u8) -> u8 {
    r * 9 + c
}

pub fn constraints() -> Vec<ConstraintSpec> {
    use ConstraintSpec::{Arrow, Cage, Line, Marker};
    let marker = |kind, a, b| Marker {
        kind,
        cells: [a, b],
    };
    vec![
        Cage {
            cells: vec![at(0, 0), at(0, 1), at(1, 0)],
            sum: Some(14),
        },
        Cage {
            cells: vec![at(4, 3), at(4, 4), at(5, 4)],
            sum: Some(15),
        },
        Line {
            kind: LineKind::GermanWhisper,
            cells: vec![at(6, 0), at(7, 0), at(7, 1), at(6, 2)],
        },
        Line {
            kind: LineKind::DutchWhisper,
            cells: vec![at(3, 1), at(3, 2), at(4, 1), at(5, 0)],
        },
        Line {
            kind: LineKind::Thermo,
            cells: vec![at(8, 6), at(7, 7), at(7, 8), at(8, 8)],
        },
        // 1, 2, 4, 3
        Line {
            kind: LineKind::Renban,
            cells: vec![at(0, 7), at(0, 8), at(1, 7), at(1, 6)],
        },
        // 9 = 3 + 4 + 2, bending so the renderer has corners to draw
        Arrow {
            cells: vec![at(5, 3), at(6, 4), at(7, 3), at(8, 3)],
        },
        marker(MarkerKind::White, at(0, 3), at(0, 4)),
        marker(MarkerKind::White, at(2, 6), at(2, 7)),
        marker(MarkerKind::Black, at(2, 4), at(2, 5)),
        marker(MarkerKind::Black, at(6, 7), at(6, 8)),
        marker(MarkerKind::X, at(7, 4), at(7, 5)),
        marker(MarkerKind::X, at(4, 7), at(4, 8)),
        marker(MarkerKind::Chevron, at(5, 5), at(5, 4)),
        marker(MarkerKind::Chevron, at(1, 4), at(1, 5)),
    ]
}

/// The puzzle's givens, as found by `find_givens`. Finding them runs a
/// uniqueness search per cell and takes seconds, so they're stored here; a
/// test checks they still match when the constraints or solver change.
pub const GIVENS: &str =
    "....7...2.....5.....8..................8.37.1......................1...5.........";

/// Givens as an 81-char line, `.` for empty. Removes digits from the full
/// solution in a fixed order for as long as the puzzle stays unique and
/// solvable by the logical solver.
pub fn find_givens() -> String {
    let ctx = SolverContext::new(&constraints());
    let solver = Solver::default();
    let mut givens: Vec<Option<u8>> = SOLUTION.iter().copied().map(Some).collect();

    let mut order: Vec<usize> = (0..81).collect();
    order.shuffle(&mut SmallRng::seed_from_u64(7));

    for pos in order {
        let digit = givens[pos].take();
        let current: Vec<(CellId, u8)> = givens
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| Some((CellId(i as u8), d?)))
            .collect();
        let solvable = count_solutions_for(&ctx, &current, 2) == 1
            && matches!(
                solver.solve(&mut Board::new(&current), &ctx),
                SolveResult::Solved(_)
            );
        if !solvable {
            givens[pos] = digit;
        }
    }

    givens
        .iter()
        .map(|d| d.map_or('.', |d| (b'0' + d) as char))
        .collect()
}
