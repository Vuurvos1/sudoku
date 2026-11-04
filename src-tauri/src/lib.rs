mod types;

use sudoku_solver::{
    board::{Board, CellId},
    constraint::ConstraintSpec,
    generator::{solve_brute, Difficulty, Generator},
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

// generate, solve and hint are async: Tauri runs those off the main thread, so
// slow work doesn't freeze the window
#[tauri::command]
async fn generate_sudoku(
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
    // generation keeps every core busy for a moment, so it gets a thread of its
    // own rather than one of the async runtime's
    let board =
        tauri::async_runtime::spawn_blocking(move || Generator::new().generate_for(diff, &ctx))
            .await
            .map_err(|e| e.to_string())?;

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
async fn solve_sudoku(puzzle: Puzzle) -> Result<Vec<u8>, String> {
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
async fn hint_sudoku(puzzle: Puzzle) -> Result<types::HintResponse, String> {
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

/// Filled cells that break a rule, for highlighting mistakes without the solution.
#[tauri::command]
fn find_conflicts(constraints: Vec<ConstraintSpec>, grid: Vec<u8>) -> Result<Vec<u8>, String> {
    let ctx = build_context(&constraints)?;
    let grid: [u8; 81] = grid
        .try_into()
        .map_err(|_| "grid must have 81 cells".to_string())?;
    if grid.iter().any(|&d| d > 9) {
        return Err("grid digits must be 0 to 9".to_string());
    }
    Ok(ctx.conflicts(&grid).iter().map(|c| c.0).collect())
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
            find_conflicts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
