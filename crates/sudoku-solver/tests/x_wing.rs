use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::Fish,
    strategy::Strategy,
};

#[test]
fn finds_x_wing_in_rows() {
    let mut board = Board::new(&[]);

    // Set up an X-Wing for digit 5:
    // Row 0: digit 5 only in columns 2 and 6
    // Row 4: digit 5 only in columns 2 and 6
    for c in [0u8, 1, 3, 4, 5, 7, 8] {
        board.remove_candidate(CellId::from_rc(0, c), 5);
        board.remove_candidate(CellId::from_rc(4, c), 5);
    }

    let ctx = SolverContext::standard();

    let deduction = Fish::<2>
        .find(&board, &ctx)
        .expect("expected to find an X-Wing for digit 5 in rows 1, 5");

    assert_eq!(deduction.strategy, "x-wing");
    assert!(deduction.placements.is_empty());

    // Should eliminate 5 from columns 2 and 6 in rows other than 0 and 4
    let expected: Vec<(CellId, u8)> = [2u8, 6]
        .iter()
        .flat_map(|&col| {
            [1u8, 2, 3, 5, 6, 7, 8]
                .iter()
                .map(move |&row| (CellId::from_rc(row, col), 5))
        })
        .collect();
    assert_eq!(deduction.eliminations, expected);
}
