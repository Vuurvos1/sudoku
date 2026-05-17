use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::SolverContext,
    strategies::YWing,
    strategy::Strategy,
};

#[test]
fn finds_y_wing() {
    let mut board = Board::new(&[]);

    // Pivot: R1C1 (0,0) = {3, 7}
    for d in [1u8, 2, 4, 5, 6, 8, 9] {
        board.remove_candidate(CellId::from_rc(0, 0), d);
    }
    // Wing1: R1C5 (0,4) = {3, 5} — shares row 0 with pivot, shares candidate 3
    for d in [1u8, 2, 4, 6, 7, 8, 9] {
        board.remove_candidate(CellId::from_rc(0, 4), d);
    }
    // Wing2: R4C1 (3,0) = {5, 7} — shares column 0 with pivot, shares candidate 7
    for d in [1u8, 2, 3, 4, 6, 8, 9] {
        board.remove_candidate(CellId::from_rc(3, 0), d);
    }

    let constraints = standard_constraints();
    let ctx = SolverContext {
        constraints: &constraints,
    };

    let deduction = YWing
        .find(&board, &ctx)
        .expect("expected to find a Y-Wing eliminating 5");

    assert_eq!(deduction.strategy, "y-wing");
    assert!(deduction.placements.is_empty());

    // R4C5 (3,4) sees both wings (shares row 3 with wing2, column 4 with wing1)
    // and has candidate 5, so 5 should be eliminated
    let expected: Vec<(CellId, u8)> = vec![(CellId::from_rc(3, 4), 5)];
    assert_eq!(deduction.eliminations, expected);
}
