use sudoku_solver::board::{Board, CellId};
use sudoku_solver::constraint::standard_constraints;
use sudoku_solver::generator::{count_solutions, Difficulty, Generator};
use sudoku_solver::solver::{SolveResult, Solver, SolverContext};

#[test]
fn fill_grid_produces_valid_complete_grid() {
    let mut gen = Generator::seeded(42);
    let grid = gen.fill_grid();

    // All 81 cells filled with 1-9
    assert_eq!(grid.len(), 81);
    for &d in &grid {
        assert!((1..=9).contains(&d), "digit {} out of range", d);
    }

    // Each row has digits 1-9
    for r in 0..9 {
        let row: Vec<u8> = (0..9).map(|c| grid[r * 9 + c]).collect();
        let mut sorted = row.clone();
        sorted.sort();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], "row {} invalid", r);
    }

    // Each column has digits 1-9
    for c in 0..9 {
        let col: Vec<u8> = (0..9).map(|r| grid[r * 9 + c]).collect();
        let mut sorted = col.clone();
        sorted.sort();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], "col {} invalid", c);
    }

    // Each 3x3 box has digits 1-9
    for br in 0..3 {
        for bc in 0..3 {
            let mut box_digits: Vec<u8> = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_digits.push(grid[(br * 3 + r) * 9 + (bc * 3 + c)]);
                }
            }
            box_digits.sort();
            assert_eq!(
                box_digits,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                "box ({},{}) invalid",
                br,
                bc
            );
        }
    }
}

#[test]
fn fill_grid_different_seeds_produce_different_grids() {
    let grid1 = Generator::seeded(1).fill_grid();
    let grid2 = Generator::seeded(2).fill_grid();
    assert_ne!(grid1, grid2);
}

#[test]
fn solved_board_has_one_solution() {
    let mut gen = Generator::seeded(42);
    let grid = gen.fill_grid();
    let givens: Vec<(CellId, u8)> = grid
        .iter()
        .enumerate()
        .map(|(i, &d)| (CellId(i as u8), d))
        .collect();
    assert_eq!(count_solutions(&givens, 2), 1);
}

#[test]
fn empty_board_has_multiple_solutions() {
    let givens: Vec<(CellId, u8)> = vec![];
    assert_eq!(count_solutions(&givens, 2), 2);
}

#[test]
fn known_unique_puzzle_has_one_solution() {
    let line = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let givens: Vec<(CellId, u8)> = line
        .chars()
        .enumerate()
        .filter_map(|(i, ch)| {
            let d = ch.to_digit(10)? as u8;
            if d > 0 {
                Some((CellId(i as u8), d))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(count_solutions(&givens, 2), 1);
}

#[test]
fn generate_easy_produces_solvable_unique_puzzle() {
    let mut gen = Generator::seeded(100);
    let board = gen.generate(Difficulty::Easy);

    // Collect givens from the board
    let givens: Vec<(CellId, u8)> = (0..81)
        .filter_map(|i| {
            let id = CellId(i);
            if board.is_given(id) {
                Some((id, board.value(id).unwrap()))
            } else {
                None
            }
        })
        .collect();

    // Must have a unique solution
    assert_eq!(count_solutions(&givens, 2), 1);

    // Must be solvable by the strategy solver
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();
    let mut solve_board = Board::new(&givens);
    match solver.solve(&mut solve_board, &ctx) {
        SolveResult::Solved(deductions) => {
            let max_diff = deductions.iter().map(|d| d.difficulty).filter(|&d| d > 0).max().unwrap_or(0);
            assert!(max_diff <= 2, "easy puzzle required difficulty {}", max_diff);
        }
        other => panic!("expected Solved, got {:?}", match other {
            SolveResult::Stuck(_) => "Stuck",
            SolveResult::Invalid(_) => "Invalid",
            _ => unreachable!(),
        }),
    }
}

#[test]
fn generate_medium_requires_at_least_difficulty_3() {
    let mut gen = Generator::seeded(200);
    let board = gen.generate(Difficulty::Medium);

    let givens: Vec<(CellId, u8)> = (0..81)
        .filter_map(|i| {
            let id = CellId(i);
            if board.is_given(id) {
                Some((id, board.value(id).unwrap()))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(count_solutions(&givens, 2), 1);

    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();
    let mut solve_board = Board::new(&givens);
    match solver.solve(&mut solve_board, &ctx) {
        SolveResult::Solved(deductions) => {
            let max_diff = deductions.iter().map(|d| d.difficulty).filter(|&d| d > 0).max().unwrap_or(0);
            assert!(max_diff >= 3, "medium puzzle max difficulty was only {}", max_diff);
            assert!(max_diff <= 3, "medium puzzle exceeded ceiling: {}", max_diff);
        }
        other => panic!("expected Solved, got {:?}", match other {
            SolveResult::Stuck(_) => "Stuck",
            SolveResult::Invalid(_) => "Invalid",
            _ => unreachable!(),
        }),
    }
}
