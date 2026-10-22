use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::NakedSubset,
    strategy::Strategy,
};

#[test]
fn finds_naked_triple_in_row() {
    let mut board = Board::new(&[]);

    // R1C1 = {2, 5}
    for d in [1u8, 3, 4, 6, 7, 8, 9] {
        board.remove_candidate(CellId::from_rc(0, 0), d);
    }
    // R1C2 = {5, 8}
    for d in [1u8, 2, 3, 4, 6, 7, 9] {
        board.remove_candidate(CellId::from_rc(0, 1), d);
    }
    // R1C3 = {2, 8}
    for d in [1u8, 3, 4, 5, 6, 7, 9] {
        board.remove_candidate(CellId::from_rc(0, 2), d);
    }

    let ctx = SolverContext::standard();

    let deduction = NakedSubset::<3>
        .find(&board, &ctx)
        .expect("expected to find a naked triple {2, 5, 8} at R1C1, R1C2, R1C3");

    assert_eq!(deduction.strategy, "naked triple");
    assert!(deduction.placements.is_empty());

    // Should eliminate 2, 5, 8 from R1C4..R1C9
    let expected: Vec<(CellId, u8)> = (3..9u8)
        .flat_map(|c| {
            [
                (CellId::from_rc(0, c), 2),
                (CellId::from_rc(0, c), 5),
                (CellId::from_rc(0, c), 8),
            ]
        })
        .collect();
    assert_eq!(deduction.eliminations, expected);
}
