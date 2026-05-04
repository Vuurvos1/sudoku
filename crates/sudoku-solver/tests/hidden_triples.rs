use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::SolverContext,
    strategies::HiddenSubset,
    strategy::Strategy,
};

#[test]
fn finds_hidden_triple_in_row() {
    let mut board = Board::new(&[]);

    // R1C1 = {1, 2, 5, 9}
    for d in [3u8, 4, 6, 7, 8] {
        board.remove_candidate(CellId::from_rc(0, 0), d);
    }
    // R1C2 = {4, 5, 8}
    for d in [1u8, 2, 3, 6, 7, 9] {
        board.remove_candidate(CellId::from_rc(0, 1), d);
    }
    // R1C3 = {2, 6, 8}
    for d in [1u8, 3, 4, 5, 7, 9] {
        board.remove_candidate(CellId::from_rc(0, 2), d);
    }
    // Remove 2, 5, 8 from R1C4..R1C9 so they only appear in R1C1-R1C3
    for c in 3..9u8 {
        board.remove_candidate(CellId::from_rc(0, c), 2);
        board.remove_candidate(CellId::from_rc(0, c), 5);
        board.remove_candidate(CellId::from_rc(0, c), 8);
    }

    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };

    let deduction = HiddenSubset::<3>
        .find(&board, &ctx)
        .expect("expected to find a hidden triple {2, 5, 8} at R1C1, R1C2, R1C3");

    assert_eq!(deduction.strategy, "hidden triple");
    assert!(deduction.placements.is_empty());

    // Should eliminate non-{2,5,8} from R1C1, R1C2, R1C3
    // R1C1 {1, 2, 5, 9}: eliminate 1, 9
    // R1C2 {4, 5, 8}: eliminate 4
    // R1C3 {2, 6, 8}: eliminate 6
    let expected: Vec<(CellId, u8)> = vec![
        (CellId::from_rc(0, 0), 1),
        (CellId::from_rc(0, 0), 9),
        (CellId::from_rc(0, 1), 4),
        (CellId::from_rc(0, 2), 6),
    ];
    assert_eq!(deduction.eliminations, expected);
}
