mod types;

use sudoku_solver::{
    board::{Board, CellId},
    constraint::ConstraintSpec,
    generator::{solve_brute, Difficulty, Generator},
    showcase,
    solver::{SolveResult, Solver, SolverContext},
};

use crate::types::{hint_response_from, Puzzle};

fn is_valid_sudoku_line(s: &str) -> bool {
    s.len() == 81 && s.chars().all(|c| matches!(c, '0'..='9' | '.'))
}

fn parse_board(puzzle: &Puzzle) -> Result<Board, String> {
    if !is_valid_sudoku_line(&puzzle.givens) {
        return Err("Invalid puzzle string".to_string());
    }
    Ok(Board::from_line(&puzzle.givens))
}

fn build_context(constraints: &[ConstraintSpec]) -> Result<SolverContext, String> {
    for spec in constraints {
        spec.validate()?;
    }
    Ok(SolverContext::new(constraints))
}

#[tauri::command]
fn generate_sudoku(
    difficulty: u8,
    constraints: Option<Vec<ConstraintSpec>>,
) -> Result<Puzzle, String> {
    let diff = match difficulty {
        1 => Difficulty::Easy,
        2 => Difficulty::Medium,
        3 => Difficulty::Hard,
        4 => Difficulty::Expert,
        _ => return Err(format!("Invalid difficulty: {}", difficulty)),
    };

    let constraints = constraints.unwrap_or_default();
    let ctx = build_context(&constraints)?;
    let board = Generator::new().generate_for(diff, &ctx);

    let givens: String = (0..81)
        .map(|i| {
            board
                .value(CellId(i))
                .map(|d| (b'0' + d) as char)
                .unwrap_or('.')
        })
        .collect();

    Ok(Puzzle {
        givens,
        constraints,
    })
}

#[tauri::command]
fn solve_sudoku(puzzle: Puzzle) -> Result<Vec<u8>, String> {
    let mut board = parse_board(&puzzle)?;
    let ctx = build_context(&puzzle.constraints)?;

    if let SolveResult::Solved(_) = Solver::default().solve(&mut board, &ctx) {
        return Ok((0..81u8)
            .map(|i| board.value(CellId(i)).unwrap_or(0))
            .collect());
    }

    // Fallback: brute-force search under the same rules
    let givens: Vec<(CellId, u8)> = puzzle
        .givens
        .bytes()
        .enumerate()
        .filter(|&(_, b)| matches!(b, b'1'..=b'9'))
        .map(|(i, b)| (CellId(i as u8), b - b'0'))
        .collect();
    match solve_brute(&ctx, &givens) {
        Some(solution) => Ok(solution.to_vec()),
        None => Err("No solution found".to_string()),
    }
}

#[tauri::command]
fn hint_sudoku(puzzle: Puzzle) -> Result<types::HintResponse, String> {
    let mut board = parse_board(&puzzle)?;
    let ctx = build_context(&puzzle.constraints)?;

    match Solver::default().hint(&mut board, &ctx) {
        Some(deduction) => Ok(hint_response_from(&deduction)),
        None => Err("No hint available".to_string()),
    }
}

#[tauri::command]
fn get_peers(constraints: Vec<ConstraintSpec>) -> Result<Vec<Vec<u8>>, String> {
    let ctx = build_context(&constraints)?;
    Ok((0..81u8)
        .map(|i| ctx.peers(CellId(i)).iter().map(|c| c.0).collect())
        .collect())
}

/// Hand-made puzzle using every variant marker, for testing.
#[tauri::command]
fn showcase_puzzle() -> Puzzle {
    Puzzle {
        givens: showcase::givens(),
        constraints: showcase::constraints(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            generate_sudoku,
            solve_sudoku,
            hint_sudoku,
            get_peers,
            showcase_puzzle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
