mod common;

use common::keep_only;
use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::XyChain,
    strategy::Strategy,
};

#[test]
fn finds_xy_chain() {
    let mut board = Board::new(&[]);

    // R1C1 {1,2} -row- R1C5 {2,3} -column- R5C5 {3,4} -row- R5C9 {4,1}.
    // If R1C1 isn't 1 it's 2, so R1C5 is 3, R5C5 is 4 and R5C9 is 1.
    keep_only(&mut board, CellId::from_rc(0, 0), &[1, 2]);
    keep_only(&mut board, CellId::from_rc(0, 4), &[2, 3]);
    keep_only(&mut board, CellId::from_rc(4, 4), &[3, 4]);
    keep_only(&mut board, CellId::from_rc(4, 8), &[1, 4]);

    let deduction = XyChain
        .find(&board, &SolverContext::standard())
        .expect("expected an XY-Chain on 1");

    assert_eq!(deduction.strategy, "xy-chain");
    assert_eq!(deduction.difficulty, 9);
    assert!(deduction.placements.is_empty());

    // R1C9 and R5C1 see both ends
    let expected: Vec<(CellId, u8)> = vec![(CellId::from_rc(0, 8), 1), (CellId::from_rc(4, 0), 1)];
    assert_eq!(deduction.eliminations, expected);
}

#[test]
fn needs_a_broken_chain_to_find_nothing() {
    let mut board = Board::new(&[]);

    // the last cell is {4,5}, so the chain never comes back to 1
    keep_only(&mut board, CellId::from_rc(0, 0), &[1, 2]);
    keep_only(&mut board, CellId::from_rc(0, 4), &[2, 3]);
    keep_only(&mut board, CellId::from_rc(4, 4), &[3, 4]);
    keep_only(&mut board, CellId::from_rc(4, 8), &[4, 5]);

    assert!(XyChain.find(&board, &SolverContext::standard()).is_none());
}
