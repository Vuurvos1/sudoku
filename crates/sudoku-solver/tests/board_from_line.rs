use sudoku_solver::board::{Board, CellId};

#[test]
fn from_line_parses_givens() {
    let line = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let board = Board::from_line(line);
    assert_eq!(board.value(CellId(0)), Some(5));
    assert_eq!(board.value(CellId(1)), Some(3));
    assert_eq!(board.value(CellId(2)), None); // '.' → empty
    assert!(board.is_given(CellId(0)));
    assert!(!board.is_decided(CellId(2)));
}

#[test]
fn from_line_parses_zeros_as_empty() {
    let line = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let board = Board::from_line(line);
    assert_eq!(board.value(CellId(0)), Some(5));
    assert_eq!(board.value(CellId(2)), None); // '0' → empty
}

#[test]
#[should_panic]
fn from_line_rejects_short_string() {
    Board::from_line("53..7");
}
