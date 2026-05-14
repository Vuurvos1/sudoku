mod types;

use sudoku::Sudoku;
use sudoku_solver::{
    board::{Board, CellId},
    constraint::{all_peers, standard_constraints},
    solver::{SolveResult, Solver, SolverContext},
};

use crate::types::hint_response_from;

fn is_valid_sudoku_line(s: &str) -> bool {
    s.len() == 81 && s.chars().all(|c| matches!(c, '0'..='9' | '.'))
}

#[tauri::command]
fn generate_sudoku() -> String {
    let s = Sudoku::generate();
    s.to_string()
}

#[tauri::command]
fn solve_sudoku(sudoku_line: String) -> Result<Vec<u8>, String> {
    if !is_valid_sudoku_line(&sudoku_line) {
        return Err("Invalid puzzle string".to_string());
    }

    let mut board = Board::from_line(&sudoku_line);
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();

    if let SolveResult::Solved(_) = solver.solve(&mut board, &ctx) {
        return Ok((0..81u8)
            .map(|i| board.value(CellId(i)).unwrap_or(0))
            .collect());
    }

    // Fallback: external crate brute-force solver
    let sudoku = Sudoku::from_str_line(&sudoku_line)
        .map_err(|e| format!("Invalid puzzle: {}", e))?;
    match sudoku.solution() {
        Some(solution) => Ok(solution.to_bytes().to_vec()),
        None => Err("No solution found".to_string()),
    }
}

#[tauri::command]
fn hint_sudoku(sudoku_line: String) -> Result<types::HintResponse, String> {
    if !is_valid_sudoku_line(&sudoku_line) {
        return Err("Invalid puzzle string".to_string());
    }

    let mut board = Board::from_line(&sudoku_line);
    let constraints = standard_constraints();
    let ctx = SolverContext { constraints: &constraints };
    let solver = Solver::default();

    match solver.hint(&mut board, &ctx) {
        Some(deduction) => Ok(hint_response_from(&deduction)),
        None => Err("No hint available".to_string()),
    }
}

#[tauri::command]
fn get_peers() -> Vec<Vec<u8>> {
    let constraints = standard_constraints();
    all_peers(&constraints)
        .into_iter()
        .map(|peers| peers.into_iter().map(|c| c.0).collect())
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![generate_sudoku, solve_sudoku, hint_sudoku, get_peers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
