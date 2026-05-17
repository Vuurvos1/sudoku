use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::SolverContext,
    strategies::Fish,
    strategy::Strategy,
};

#[test]
fn finds_swordfish_in_rows() {
    let mut board = Board::new(&[]);

    // Set up a Swordfish for digit 3:
    // Row 0: digit 3 only in columns 1 and 4
    // Row 3: digit 3 only in columns 1 and 7
    // Row 6: digit 3 only in columns 4 and 7
    // Union of columns = {1, 4, 7} = 3 = N
    let remove_from = |board: &mut Board, row: u8, keep: [u8; 2]| {
        for c in 0..9u8 {
            if c != keep[0] && c != keep[1] {
                board.remove_candidate(CellId::from_rc(row, c), 3);
            }
        }
    };
    remove_from(&mut board, 0, [1, 4]);
    remove_from(&mut board, 3, [1, 7]);
    remove_from(&mut board, 6, [4, 7]);

    let constraints = standard_constraints();
    let ctx = SolverContext {
        constraints: &constraints,
    };

    let deduction = Fish::<3>
        .find(&board, &ctx)
        .expect("expected to find a Swordfish for digit 3 in rows 1, 4, 7");

    assert_eq!(deduction.strategy, "swordfish");
    assert!(deduction.placements.is_empty());

    // Should eliminate 3 from columns 1, 4, 7 in rows other than 0, 3, 6
    let expected: Vec<(CellId, u8)> = [1u8, 4, 7]
        .iter()
        .flat_map(|&col| {
            [1u8, 2, 4, 5, 7, 8]
                .iter()
                .map(move |&row| (CellId::from_rc(row, col), 3))
        })
        .collect();
    assert_eq!(deduction.eliminations, expected);
}
