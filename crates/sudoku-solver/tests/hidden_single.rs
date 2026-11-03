use sudoku_solver::{
    board::{Board, CellId},
    solver::SolverContext,
    strategies::HiddenSingle,
    strategy::Strategy,
};

#[test]
fn finds_hidden_single_in_row() {
    // Place a 9 in every column 1..=8 (in rows/boxes that don't conflict),
    // so in row 0 the digit 9 can only go in column 0.
    // (R1C1 has many other candidates, so naked single would not fire.)
    let placements = [
        (3, 1),
        (6, 2),
        (1, 3),
        (4, 4),
        (7, 5),
        (2, 6),
        (5, 7),
        (8, 8),
    ];
    let givens: Vec<(CellId, u8)> = placements
        .iter()
        .map(|&(r, c)| (CellId::from_rc(r, c), 9))
        .collect();

    let mut board = Board::new(&givens);
    let ctx = SolverContext::standard();

    // Run propagation so candidate sets reflect the givens.
    ctx.propagate(&mut board);

    let deduction = HiddenSingle
        .find(&board, &ctx)
        .expect("expected to find a hidden single for 9 at R1C1");

    assert_eq!(deduction.placements, vec![(CellId::from_rc(0, 0), 9)]);
}
