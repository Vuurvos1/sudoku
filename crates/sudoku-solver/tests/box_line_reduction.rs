use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::BoxLineReduction,
    strategy::Strategy,
};

#[test]
fn finds_box_line_reduction_for_row_locked_to_box() {
    // In row 0, eliminate digit 5 from columns 3..=8, leaving 5's candidates
    // in row 0 confined to columns 0..=2 — i.e., box 0.
    // BoxLineReduction should eliminate 5 from the rest of box 0 (rows 1 and 2).
    let mut board = Board::new(&[]);
    for c in 3..9u8 {
        board.remove_candidate(CellId::from_rc(0, c), 5);
    }

    let ctx = SolverContext::standard();

    let deduction = BoxLineReduction
        .find(&board, &ctx)
        .expect("expected a box/line reduction eliminating 5 from rest of box 0");

    assert!(deduction.placements.is_empty());

    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(1, 0), 5),
        (CellId::from_rc(1, 1), 5),
        (CellId::from_rc(1, 2), 5),
        (CellId::from_rc(2, 0), 5),
        (CellId::from_rc(2, 1), 5),
        (CellId::from_rc(2, 2), 5),
    ];
    assert_eq!(deduction.eliminations, expected);
}
