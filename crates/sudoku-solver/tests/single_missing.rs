use sudoku_solver::{
    board::{Board, CellId},
    constraint::standard_constraints,
    solver::{SolveResult, Solver, SolverContext},
    strategies::NakedSingle,
    strategy::Strategy,
};

#[test]
fn solves_single_missing_digit() {
    // A complete, valid sudoku solution.
    let mut puzzle: [u8; 81] = [
        5, 3, 4,  6, 7, 8,  9, 1, 2,
        6, 7, 2,  1, 9, 5,  3, 4, 8,
        1, 9, 8,  3, 4, 2,  5, 6, 7,

        8, 5, 9,  7, 6, 1,  4, 2, 3,
        4, 2, 6,  8, 5, 3,  7, 9, 1,
        7, 1, 3,  9, 2, 4,  8, 5, 6,

        9, 6, 1,  5, 3, 7,  2, 8, 4,
        2, 8, 7,  4, 1, 9,  6, 3, 5,
        3, 4, 5,  2, 8, 6,  1, 7, 9,
    ];

    // Erase one cell — R1C1, expected to be 5.
    let target = CellId::from_rc(0, 0);
    let expected = puzzle[target.0 as usize];
    puzzle[target.0 as usize] = 0;

    let mut board = Board::from_array(&puzzle);
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };

    let strategies: Vec<Box<dyn Strategy>> = vec![Box::new(NakedSingle)];
    let solver = Solver::new(strategies);

    match solver.solve(&mut board, &ctx) {
        SolveResult::Solved(steps) => {
            assert_eq!(board.value(target), Some(expected));
            // Sanity: at least one propagation deduction and one placement.
            assert!(steps.iter().any(|d| !d.placements.is_empty()),
                    "expected at least one placement deduction");
        }
        SolveResult::Stuck(_) => panic!("solver got stuck on a single missing digit"),
        SolveResult::Invalid(_) => panic!("solver flagged a valid puzzle as invalid"),
    }
}
