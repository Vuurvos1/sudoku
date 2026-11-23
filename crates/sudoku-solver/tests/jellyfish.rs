mod common;

use common::keep_in_line;
use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::Fish,
    strategy::Strategy,
};

#[test]
fn finds_jellyfish_in_rows() {
    let mut board = Board::new(&[]);

    // 3 in rows 1, 3, 5 and 7 only in columns 1, 4, 6 and 8
    for (row, keep) in [(0u8, [0u8, 3]), (2, [3, 5]), (4, [5, 7]), (6, [0, 7])] {
        keep_in_line(&mut board, row, true, 3, &keep);
    }

    let deduction = Fish::<4>
        .find(&board, &SolverContext::standard())
        .expect("expected a Jellyfish on 3");

    assert_eq!(deduction.strategy, "jellyfish");
    assert_eq!(deduction.difficulty, 9);

    // 3 leaves the rest of those four columns
    let mut expected: Vec<(CellId, u8)> = [0u8, 3, 5, 7]
        .iter()
        .flat_map(|&c| [1u8, 3, 5, 7, 8].map(|r| (CellId::from_rc(r, c), 3)))
        .collect();
    expected.sort_by_key(|(c, _)| c.0);
    let mut found = deduction.eliminations.clone();
    found.sort_by_key(|(c, _)| c.0);
    assert_eq!(found, expected);
}
