use std::fmt::format;
use serde_json::json;

use sudoku::Sudoku;
use sudoku::strategy;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_sudoku() -> String {
    let s = Sudoku::generate();
    s.to_string()
}

#[tauri::command]
fn solve_sudoku(sudoku_line: String) -> Vec<u8> {
    let sudoku = Sudoku::from_str_line(&sudoku_line).unwrap();

    if let Some(solution) = sudoku.solution() {
        println!("{}", solution);

        println!("{:?}", solution.to_bytes());
        let cell_contents = solution.to_bytes();
        return cell_contents.to_vec();
    }

    // return vec![1,2,3,4]
    // TODO: throw error or something
    return [0; 81].to_vec();
    // return "No solution found".to_string();
}

// -> String
#[tauri::command]
fn hint_sudoku(sudoku_line: String) -> String  {
    let sudoku = Sudoku::from_str_line(&sudoku_line).unwrap();

    let solver = strategy::StrategySolver::from_sudoku(sudoku);

    // TODO: get all strategies
    let strategies = [strategy::Strategy::HiddenSingles, strategy::Strategy::HiddenPairs, strategy::Strategy::HiddenTriples, strategy::Strategy::HiddenQuads, strategy::Strategy::NakedSingles, strategy::Strategy::NakedPairs, strategy::Strategy::NakedTriples, strategy::Strategy::NakedQuads, strategy::Strategy::XWing, strategy::Strategy::Swordfish, strategy::Strategy::Jellyfish];
    let res = solver.solve(&strategies).unwrap();

    println!("{:?}", res.1);

    

    // return format!("{:?}", json!(res.1));


    // let strategies = [strategy::Strategy::HiddenSingles, strategy::Strategy::HiddenPairs, strategy::Strategy::HiddenTriples, strategy::Strategy::HiddenQuads, strategy::Strategy::NakedSingles, strategy::Strategy::NakedPairs, strategy::Strategy::NakedTriples, strategy::Strategy::NakedQuads, strategy::Strategy::PointingPairs, strategy::Strategy::PointingTriples, strategy::Strategy::BoxLineReduction, strategy::Strategy::XWing, strategy::Strategy::Swordfish, strategy::Strategy::Jellyfish, strategy::Strategy::XYWing, strategy::Strategy::XYZWing, strategy::Strategy::WXYZWing, strategy::Strategy::SueDeCoq, strategy::Strategy::GroupedXWing, strategy::Strategy::GroupedSueDeCoq, strategy::Strategy::GroupedJellyfish, strategy::Strategy::GroupedSwordfish, strategy::Strategy::GroupedXYZWing, strategy::Strategy::GroupedWXYZWing];

    // let res = solver.solve(&strategies);

    // return "No hint found".to_string();


    // {
    //     return hint.to_string();
    // }

    return "No hint found".to_string();

    // let hint = strategy::deduction::Deduction::from(&sudoku);
    
    // if let Some(hint) = hint {
    //     return hint.to_string();
    // }

    // return "No hint found".to_string();


    // strategy::deduction::Deduction::from
    // if let Some(hint) =


}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![generate_sudoku, solve_sudoku, hint_sudoku])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
