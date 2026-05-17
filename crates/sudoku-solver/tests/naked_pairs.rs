use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::SolverContext,
    strategies::NakedSubset,
    strategy::Strategy,
};

#[test]
fn finds_naked_pair_in_row() {
    let mut board = Board::new(&[]);

    // Strip R1C1 and R1C2 down to candidates {3, 7}
    for d in [1u8, 2, 4, 5, 6, 8, 9] {
        board.remove_candidate(CellId::from_rc(0, 0), d);
        board.remove_candidate(CellId::from_rc(0, 1), d);
    }

    let constraints = standard_constraints();
    let ctx = SolverContext {
        constraints: &constraints,
    };

    let deduction = NakedSubset::<2>
        .find(&board, &ctx)
        .expect("expected to find a naked pair {3, 7} at R1C1, R1C2");

    assert_eq!(deduction.strategy, "naked pair");
    assert!(deduction.placements.is_empty());

    // Should eliminate 3 and 7 from R1C3..R1C9
    let expected: Vec<(CellId, u8)> = (2..9u8)
        .flat_map(|c| [(CellId::from_rc(0, c), 3), (CellId::from_rc(0, c), 7)])
        .collect();
    assert_eq!(deduction.eliminations, expected);
}
