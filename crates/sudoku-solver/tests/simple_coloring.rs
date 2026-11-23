mod common;

use common::keep_in_line;
use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::SimpleColoring,
    strategy::Strategy,
};

#[test]
fn eliminates_from_cells_seeing_both_colors() {
    let mut board = Board::new(&[]);

    // Chain of 5s: R1C2 -row- R1C7 -column- R5C7 -row- R5C3,
    // colored A, B, A, B. One color is true, so either an A or a B is 5.
    keep_in_line(&mut board, 0, true, 5, &[1, 6]);
    keep_in_line(&mut board, 6, false, 5, &[0, 4]);
    keep_in_line(&mut board, 4, true, 5, &[2, 6]);

    let ctx = SolverContext::standard();
    let deduction = SimpleColoring
        .find(&board, &ctx)
        .expect("expected Simple Coloring on 5");

    assert_eq!(deduction.strategy, "simple coloring");
    assert_eq!(deduction.difficulty, 7);
    assert!(deduction.placements.is_empty());

    // R2C3 and R3C3 see R1C2 (A, box) and R5C3 (B, column);
    // R4C2 and R6C2 see R1C2 (A, column) and R5C3 (B, box)
    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(1, 2), 5),
        (CellId::from_rc(2, 2), 5),
        (CellId::from_rc(3, 1), 5),
        (CellId::from_rc(5, 1), 5),
    ];
    assert_eq!(deduction.eliminations, expected);
}

#[test]
fn removes_a_color_that_sees_itself() {
    let mut board = Board::new(&[]);

    // Chain of 5s: R1C1 -row- R1C5 -column- R5C5 -row- R5C2 -column- R3C2,
    // colored A, B, A, B, A. R1C1 and R3C2 are both A and share box 1,
    // so A can't be 5.
    keep_in_line(&mut board, 0, true, 5, &[0, 4]);
    keep_in_line(&mut board, 4, false, 5, &[0, 4]);
    keep_in_line(&mut board, 4, true, 5, &[1, 4]);
    keep_in_line(&mut board, 1, false, 5, &[2, 4]);

    let deduction = SimpleColoring
        .find(&board, &SolverContext::standard())
        .expect("expected Simple Coloring on 5");

    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(0, 0), 5),
        (CellId::from_rc(2, 1), 5),
        (CellId::from_rc(4, 4), 5),
    ];
    assert_eq!(deduction.eliminations, expected);
}

#[test]
fn needs_a_chain_of_two_links() {
    let mut board = Board::new(&[]);

    // a single pair of 5s is not a chain
    keep_in_line(&mut board, 0, true, 5, &[1, 6]);

    assert!(SimpleColoring
        .find(&board, &SolverContext::standard())
        .is_none());
}
