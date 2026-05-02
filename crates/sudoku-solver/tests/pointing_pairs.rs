use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::SolverContext,
    strategies::PointingPairs,
    strategy::Strategy,
};

#[test]
fn finds_pointing_pair_in_box_locked_to_row() {
    // In box 0, eliminate digit 5 from rows 0 and 1, leaving 5's candidates
    // in box 0 confined to row 2 cells (R3C1, R3C2, R3C3).
    // PointingPairs should eliminate 5 from the rest of row 2 (R3C4..R3C9).
    let mut board = Board::new(&[]);
    for c in 0..3u8 {
        board.remove_candidate(CellId::from_rc(0, c), 5);
        board.remove_candidate(CellId::from_rc(1, c), 5);
    }

    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };

    let deduction = PointingPairs
        .find(&board, &ctx)
        .expect("expected a pointing pair eliminating 5 from rest of row 2");

    assert!(deduction.placements.is_empty());

    let expected: Vec<(CellId, u8)> = (3..=8u8)
        .map(|c| (CellId::from_rc(2, c), 5))
        .collect();
    assert_eq!(deduction.eliminations, expected);
}
