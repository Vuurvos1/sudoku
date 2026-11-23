#![allow(dead_code)]

use sudoku_solver::board::{Board, CellId};

/// Leave only `keep` as candidates of `cell`.
pub fn keep_only(board: &mut Board, cell: CellId, keep: &[u8]) {
    for d in 1..=9u8 {
        if !keep.contains(&d) {
            board.remove_candidate(cell, d);
        }
    }
}

/// Leave `digit` only at positions `keep` of row `line`, or column `line`
/// when `is_row` is false.
pub fn keep_in_line(board: &mut Board, line: u8, is_row: bool, digit: u8, keep: &[u8]) {
    for pos in 0..9u8 {
        if !keep.contains(&pos) {
            let cell = if is_row {
                CellId::from_rc(line, pos)
            } else {
                CellId::from_rc(pos, line)
            };
            board.remove_candidate(cell, digit);
        }
    }
}
