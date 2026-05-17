use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use sudoku_solver::{
    board::Board,
    constraint::standard_constraints,
    solver::{SolveResult, Solver, SolverContext},
    strategies::{BoxLineReduction, HiddenSingle, NakedSingle, PointingPairs},
    strategy::Strategy,
};

// An easy puzzle derived from a complete grid by erasing ~22 cells.
// Solvable with naked + hidden singles + propagation only.
const EASY: [u8; 81] = [
    5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 0, 0, 9, 5, 3, 0, 8, 1, 0, 8, 3, 0, 2, 0, 6, 7, 8, 0, 9, 7, 6,
    0, 4, 2, 0, 4, 2, 0, 8, 5, 0, 7, 9, 1, 0, 1, 3, 9, 0, 4, 8, 0, 6, 9, 0, 1, 0, 3, 7, 2, 8, 0, 0,
    8, 7, 4, 0, 9, 6, 3, 0, 3, 4, 5, 2, 8, 0, 0, 7, 9,
];

// A medium puzzle that exercises pointing pairs and box/line reduction.
const MEDIUM: [u8; 81] = [
    0, 0, 0, 2, 6, 0, 7, 0, 1, 6, 8, 0, 0, 7, 0, 0, 9, 0, 1, 9, 0, 0, 0, 4, 5, 0, 0, 8, 2, 0, 1, 0,
    0, 0, 4, 0, 0, 0, 4, 6, 0, 2, 9, 0, 0, 0, 5, 0, 0, 0, 3, 0, 2, 8, 0, 0, 9, 3, 0, 0, 0, 7, 4, 0,
    4, 0, 0, 5, 0, 0, 3, 6, 7, 0, 3, 0, 1, 8, 0, 0, 0,
];

fn build_solver() -> Solver {
    let strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(NakedSingle),
        Box::new(HiddenSingle),
        Box::new(PointingPairs),
        Box::new(BoxLineReduction),
    ];
    Solver::new(strategies)
}

fn assert_solvable(label: &str, puzzle: &[u8; 81], solver: &Solver, ctx: &SolverContext) {
    let mut board = Board::from_array(puzzle);
    match solver.solve(&mut board, ctx) {
        SolveResult::Solved(_) => {}
        SolveResult::Stuck(_) => panic!("benchmark puzzle '{}': solver got stuck", label),
        SolveResult::Invalid(_) => panic!("benchmark puzzle '{}': solver flagged invalid", label),
    }
}

fn bench_solve(c: &mut Criterion) {
    let constraints = standard_constraints();
    let solver = build_solver();
    let ctx = SolverContext {
        constraints: &constraints,
    };

    // Sanity check: bail out loudly if a puzzle isn't solvable with the current
    // strategy set, so a "stuck" run can't masquerade as a fast solve.
    assert_solvable("easy", &EASY, &solver, &ctx);
    assert_solvable("medium", &MEDIUM, &solver, &ctx);

    c.bench_function("solve_easy", |b| {
        b.iter_batched(
            || Board::from_array(&EASY),
            |mut board| solver.solve(&mut board, &ctx),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("solve_medium", |b| {
        b.iter_batched(
            || Board::from_array(&MEDIUM),
            |mut board| solver.solve(&mut board, &ctx),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_solve);
criterion_main!(benches);
