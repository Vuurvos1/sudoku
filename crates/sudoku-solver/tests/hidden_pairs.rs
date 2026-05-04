use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::SolverContext,
    strategies::HiddenSubset,
    strategy::Strategy,
};

#[test]
fn finds_hidden_pair_in_row() {
    let mut board = Board::new(&[]);

    // R1C1 = {1, 3, 5, 7}
    for d in [2u8, 4, 6, 8, 9] {
        board.remove_candidate(CellId::from_rc(0, 0), d);
    }
    // R1C2 = {2, 3, 7, 9}
    for d in [1u8, 4, 5, 6, 8] {
        board.remove_candidate(CellId::from_rc(0, 1), d);
    }
    // Remove 3 and 7 from R1C3..R1C9 so they only appear in R1C1, R1C2
    for c in 2..9u8 {
        board.remove_candidate(CellId::from_rc(0, c), 3);
        board.remove_candidate(CellId::from_rc(0, c), 7);
    }

    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };

    let deduction = HiddenSubset::<2>
        .find(&board, &ctx)
        .expect("expected to find a hidden pair {3, 7} at R1C1, R1C2");

    assert_eq!(deduction.strategy, "hidden pair");
    assert!(deduction.placements.is_empty());

    // Should eliminate non-{3,7} candidates from R1C1 and R1C2
    // R1C1 {1, 3, 5, 7}: eliminate 1, 5
    // R1C2 {2, 3, 7, 9}: eliminate 2, 9
    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(0, 0), 1),
        (CellId::from_rc(0, 0), 5),
        (CellId::from_rc(0, 1), 2),
        (CellId::from_rc(0, 1), 9),
    ];
    assert_eq!(deduction.eliminations, expected);
}
