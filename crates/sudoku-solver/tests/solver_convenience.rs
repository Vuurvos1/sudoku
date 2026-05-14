use sudoku_solver::{
    board::Board,
    constraint::standard_constraints,
    solver::{SolveResult, Solver, SolverContext},
};

#[test]
fn default_solver_solves_easy_puzzle() {
    let line = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let mut board = Board::from_line(line);
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();

    match solver.solve(&mut board, &ctx) {
        SolveResult::Solved(_) => {
            assert!(board.is_solved());
        }
        SolveResult::Stuck(_) => panic!("default solver got stuck on easy puzzle"),
        SolveResult::Invalid(_) => panic!("default solver flagged valid puzzle as invalid"),
    }
}

#[test]
fn hint_returns_deduction_for_easy_puzzle() {
    let line = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let mut board = Board::from_line(line);
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();

    let hint = solver.hint(&mut board, &ctx);
    assert!(hint.is_some(), "expected a hint for easy puzzle");

    let deduction = hint.unwrap();
    assert!(!deduction.placements.is_empty() || !deduction.eliminations.is_empty());
}

#[test]
fn hint_returns_none_for_solved_board() {
    let solved: [u8; 81] = [
        5, 3, 4, 6, 7, 8, 9, 1, 2,
        6, 7, 2, 1, 9, 5, 3, 4, 8,
        1, 9, 8, 3, 4, 2, 5, 6, 7,
        8, 5, 9, 7, 6, 1, 4, 2, 3,
        4, 2, 6, 8, 5, 3, 7, 9, 1,
        7, 1, 3, 9, 2, 4, 8, 5, 6,
        9, 6, 1, 5, 3, 7, 2, 8, 4,
        2, 8, 7, 4, 1, 9, 6, 3, 5,
        3, 4, 5, 2, 8, 6, 1, 7, 9,
    ];
    let mut board = Board::from_array(&solved);
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();

    let hint = solver.hint(&mut board, &ctx);
    assert!(hint.is_none(), "no hint expected for solved board");
}
