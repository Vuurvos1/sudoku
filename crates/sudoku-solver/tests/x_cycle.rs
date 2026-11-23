mod common;

use common::keep_in_line;
use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::XCycle,
    strategy::Strategy,
};

#[test]
fn eliminates_from_cells_seeing_both_chain_ends() {
    let mut board = Board::new(&[]);

    // 4 in row 2 only at R2C3 and R2C7, in row 6 only at R6C3 and R6C8.
    // R2C7 off -> R2C3 on -> R6C3 off -> R6C8 on: R2C7 or R6C8 is 4.
    keep_in_line(&mut board, 1, true, 4, &[2, 6]);
    keep_in_line(&mut board, 5, true, 4, &[2, 7]);

    let deduction = XCycle
        .find(&board, &SolverContext::standard())
        .expect("expected an X-Cycle on 4");

    assert_eq!(deduction.strategy, "x-cycle");
    assert_eq!(deduction.difficulty, 9);
    assert!(deduction.placements.is_empty());

    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(0, 7), 4),
        (CellId::from_rc(2, 7), 4),
        (CellId::from_rc(3, 6), 4),
        (CellId::from_rc(4, 6), 4),
    ];
    assert_eq!(deduction.eliminations, expected);
}

#[test]
fn uses_a_box_to_eliminate() {
    let mut board = Board::new(&[]);

    // 4s: row 1 only R1C1 and R1C5, row 5 only R5C2 and R5C5,
    // box 1 only R1C1 and R3C2.
    // R1C1 off -> R1C5 on (row) -> R5C5 off (column) -> R5C2 on (row),
    // so R1C1 or R5C2 is 4.
    keep_in_line(&mut board, 0, true, 4, &[0, 4]);
    keep_in_line(&mut board, 4, true, 4, &[1, 4]);
    for (r, c) in [(1u8, 0u8), (1, 1), (1, 2), (2, 0), (2, 2)] {
        board.remove_candidate(CellId::from_rc(r, c), 4);
    }

    let deduction = XCycle
        .find(&board, &SolverContext::standard())
        .expect("expected an X-Cycle on 4");

    // R3C2 sees R1C1 (box) and R5C2 (column);
    // R4C1 and R6C1 see R1C1 (column) and R5C2 (box)
    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(2, 1), 4),
        (CellId::from_rc(3, 0), 4),
        (CellId::from_rc(5, 0), 4),
    ];
    assert_eq!(deduction.eliminations, expected);
}

#[test]
fn needs_at_least_three_links() {
    let mut board = Board::new(&[]);
    keep_in_line(&mut board, 1, true, 4, &[2, 6]);

    assert!(XCycle.find(&board, &SolverContext::standard()).is_none());
}
