mod common;

use common::keep_only;
use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::XyzWing,
    strategy::Strategy,
};

#[test]
fn finds_xyz_wing() {
    let mut board = Board::new(&[]);

    // Pivot R1C1 = {1, 2, 3}, wing R1C5 = {1, 3} (same row),
    // wing R3C2 = {2, 3} (same box). One of the three is 3.
    keep_only(&mut board, CellId::from_rc(0, 0), &[1, 2, 3]);
    keep_only(&mut board, CellId::from_rc(0, 4), &[1, 3]);
    keep_only(&mut board, CellId::from_rc(2, 1), &[2, 3]);

    let ctx = SolverContext::standard();
    let deduction = XyzWing
        .find(&board, &ctx)
        .expect("expected an XYZ-Wing eliminating 3");

    assert_eq!(deduction.strategy, "xyz-wing");
    assert_eq!(deduction.difficulty, 8);
    assert!(deduction.placements.is_empty());

    // only R1C2 and R1C3 see the pivot and both wings
    let expected: Vec<(CellId, u8)> = vec![(CellId::from_rc(0, 1), 3), (CellId::from_rc(0, 2), 3)];
    assert_eq!(deduction.eliminations, expected);
}

#[test]
fn needs_wings_inside_the_pivot() {
    let mut board = Board::new(&[]);

    // R1C5 = {1, 4} brings in a digit the pivot doesn't have
    keep_only(&mut board, CellId::from_rc(0, 0), &[1, 2, 3]);
    keep_only(&mut board, CellId::from_rc(0, 4), &[1, 4]);
    keep_only(&mut board, CellId::from_rc(2, 1), &[2, 3]);

    assert!(XyzWing.find(&board, &SolverContext::standard()).is_none());
}
