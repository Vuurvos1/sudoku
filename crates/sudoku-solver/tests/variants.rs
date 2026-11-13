use sudoku_solver::{
    board::{Board, CellId},
    constraint::{ConstraintSpec, LineKind, MarkerKind},
    generator::{count_solutions_for, solve_brute, Difficulty, Generator},
    solver::{SolveResult, Solver, SolverContext},
    strategies::HiddenSingle,
    strategy::Strategy,
};

mod showcase;

fn rc(r: u8, c: u8) -> CellId {
    CellId::from_rc(r, c)
}

/// Asserts no two peers in a complete grid hold the same digit.
fn assert_valid(ctx: &SolverContext, grid: &[u8; 81]) {
    for i in 0..81u8 {
        let d = grid[i as usize];
        assert!((1..=9).contains(&d), "{} is empty", CellId(i));
        for &p in ctx.peers(CellId(i)) {
            assert_ne!(
                d,
                grid[p.0 as usize],
                "{} and {} both hold {}",
                CellId(i),
                p,
                d
            );
        }
    }
}

#[test]
fn standard_peers_are_row_col_box() {
    let ctx = SolverContext::standard();
    assert_eq!(ctx.peers(rc(4, 4)).len(), 20);
    assert!(ctx.sees(rc(4, 4), rc(4, 0)));
    assert!(ctx.sees(rc(4, 4), rc(3, 3)));
    assert!(!ctx.sees(rc(4, 4), rc(2, 5)));
}

#[test]
fn anti_knight_adds_knight_peers() {
    let ctx = SolverContext::new(&[ConstraintSpec::AntiKnight]);
    assert!(ctx.sees(rc(4, 4), rc(2, 5)));
    assert!(ctx.sees(rc(2, 5), rc(4, 4)));
    // all 8 knight moves from the centre leave its row, column and box
    assert_eq!(ctx.peers(rc(4, 4)).len(), 20 + 8);
    // a corner's two knight moves stay inside its box
    assert_eq!(ctx.peers(rc(0, 0)).len(), 20);
}

#[test]
fn anti_king_adds_only_cross_box_diagonals() {
    let ctx = SolverContext::new(&[ConstraintSpec::AntiKing]);
    // R3C3 and R4C4 are diagonal neighbours in different boxes
    assert!(ctx.sees(rc(2, 2), rc(3, 3)));
    // centre of a box: all king moves are inside the box already
    assert_eq!(ctx.peers(rc(4, 4)).len(), 20);
}

#[test]
fn diagonals_are_houses() {
    let ctx = SolverContext::new(&[ConstraintSpec::Diagonals]);
    assert_eq!(ctx.houses.len(), 29);
    assert!(ctx.sees(rc(0, 0), rc(8, 8)));
    assert!(ctx.sees(rc(0, 8), rc(8, 0)));
    assert!(!ctx.sees(rc(0, 1), rc(8, 8)));
}

#[test]
fn hidden_single_on_diagonal() {
    // 9 sits in every row/column that crosses the main diagonal except at R1C1,
    // so the main diagonal can only take its 9 at R1C1.
    let givens: Vec<(CellId, u8)> = [
        (1, 3),
        (2, 6),
        (3, 1),
        (4, 7),
        (5, 2),
        (6, 4),
        (7, 8),
        (8, 5),
    ]
    .iter()
    .map(|&(r, c)| (rc(r, c), 9))
    .collect();
    let mut board = Board::new(&givens);
    let ctx = SolverContext::new(&[ConstraintSpec::Diagonals]);
    ctx.propagate(&mut board);

    let deduction = HiddenSingle.find(&board, &ctx).expect("hidden single");
    assert_eq!(deduction.placements, vec![(rc(0, 0), 9)]);
}

#[test]
fn conflicting_givens_have_no_solutions() {
    let ctx = SolverContext::new(&[ConstraintSpec::AntiKnight]);
    let givens = [(rc(4, 4), 5), (rc(2, 5), 5)];
    assert_eq!(count_solutions_for(&ctx, &givens, 2), 0);
    assert!(solve_brute(&ctx, &givens).is_none());
}

#[test]
fn generates_unique_solvable_variant_puzzles() {
    let rule_sets = [
        vec![ConstraintSpec::Diagonals],
        vec![ConstraintSpec::AntiKnight],
        vec![ConstraintSpec::AntiKing],
        vec![ConstraintSpec::AntiKnight, ConstraintSpec::AntiKing],
    ];
    for (seed, specs) in rule_sets.iter().enumerate() {
        let ctx = SolverContext::new(specs);
        let mut gen = Generator::seeded(seed as u64);

        assert_valid(&ctx, &gen.fill_grid_for(&ctx));

        let board = gen.generate_for(Difficulty::Medium, &ctx);
        let givens: Vec<(CellId, u8)> = (0..81u8)
            .filter_map(|i| Some((CellId(i), board.value(CellId(i))?)))
            .collect();
        assert_eq!(count_solutions_for(&ctx, &givens, 2), 1, "{:?}", specs);

        let mut solved = Board::new(&givens);
        let SolveResult::Solved(_) = Solver::default().solve(&mut solved, &ctx) else {
            panic!("logical solver failed on {:?}", specs);
        };
        let grid: [u8; 81] = std::array::from_fn(|i| solved.value(CellId(i as u8)).unwrap());
        assert_valid(&ctx, &grid);
        assert_eq!(Some(grid), solve_brute(&ctx, &givens));
    }
}

#[test]
fn relations_hold_both_ways() {
    use sudoku_solver::variants::Relation;
    assert!(Relation::Consecutive.holds(4, 5) && Relation::Consecutive.holds(5, 4));
    assert!(Relation::Ratio2.holds(3, 6) && Relation::Ratio2.holds(6, 3));
    assert!(!Relation::Ratio2.holds(3, 5));
    assert!(Relation::Sum(10).holds(3, 7));
    assert!(Relation::Greater.holds(5, 2) && !Relation::Greater.holds(2, 5));
    assert!(Relation::DiffAtLeast(5).holds(1, 6) && !Relation::DiffAtLeast(5).holds(5, 9));
}

#[test]
fn german_whisper_removes_five() {
    let ctx = SolverContext::new(&[ConstraintSpec::Line {
        kind: LineKind::GermanWhisper,
        cells: vec![0, 1, 2],
    }]);
    let mut board = Board::new(&[]);
    ctx.propagate(&mut board);
    for cell in [0, 1, 2] {
        assert!(!board.candidates(CellId(cell)).contains(5));
    }
}

#[test]
fn thermo_bounds_by_position() {
    let ctx = SolverContext::new(&[ConstraintSpec::Line {
        kind: LineKind::Thermo,
        cells: vec![0, 1, 2, 3],
    }]);
    let mut board = Board::new(&[]);
    ctx.propagate(&mut board);
    let digits = |c: u8| board.candidates(CellId(c)).iter().collect::<Vec<_>>();
    assert_eq!(digits(0), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(digits(3), vec![4, 5, 6, 7, 8, 9]);
}

fn digits_of(board: &Board, c: u8) -> Vec<u8> {
    board.candidates(CellId(c)).iter().collect()
}

fn renban(cells: Vec<u8>) -> ConstraintSpec {
    ConstraintSpec::Line {
        kind: LineKind::Renban,
        cells,
    }
}

#[test]
fn renban_keeps_runs_through_placed_digits() {
    // a 5 on a 3-cell line leaves the runs 3-5, 4-6 and 5-7
    let ctx = SolverContext::new(&[renban(vec![0, 10, 20])]);
    let mut board = Board::new(&[(CellId(0), 5)]);
    ctx.propagate(&mut board);
    assert_eq!(digits_of(&board, 10), vec![3, 4, 6, 7]);
    assert_eq!(digits_of(&board, 20), vec![3, 4, 6, 7]);

    // 1 and 4 on a 4-cell line pin it to 1-4
    let ctx = SolverContext::new(&[renban(vec![0, 1, 2, 3])]);
    let mut board = Board::new(&[(CellId(0), 1), (CellId(3), 4)]);
    ctx.propagate(&mut board);
    assert_eq!(digits_of(&board, 1), vec![2, 3]);
    assert_eq!(digits_of(&board, 2), vec![2, 3]);
}

#[test]
fn renban_needs_every_digit_of_its_run() {
    // the 4 in R1C9 can't be on the line, so a 4-cell line in row 1 can't
    // hold a run through 4, even though each cell alone could take 1, 2 or 3
    let ctx = SolverContext::new(&[renban(vec![0, 1, 2, 3])]);
    let mut board = Board::new(&[(CellId(8), 4)]);
    ctx.propagate(&mut board);
    for cell in 0..4 {
        assert_eq!(digits_of(&board, cell), vec![5, 6, 7, 8, 9]);
    }

    // an empty 5-cell line rules nothing out: every digit is in some run
    let ctx = SolverContext::new(&[renban(vec![0, 1, 2, 3, 4])]);
    let mut board = Board::new(&[]);
    ctx.propagate(&mut board);
    assert_eq!(digits_of(&board, 0), (1..=9).collect::<Vec<_>>());
}

#[test]
fn renban_holds_in_search_and_conflicts() {
    // bending through two boxes, so the houses alone don't keep it distinct
    let cells = vec![18, 19, 29, 30, 31, 40];
    let ctx = SolverContext::new(&[renban(cells.clone())]);
    let grid = solve_brute(&ctx, &[(CellId(18), 7)]).unwrap();
    assert_valid(&ctx, &grid);
    let mut digits: Vec<u8> = cells.iter().map(|&c| grid[c as usize]).collect();
    digits.sort();
    assert!(digits.windows(2).all(|w| w[1] == w[0] + 1), "{:?}", digits);
    assert!(digits.contains(&7));

    let ctx = SolverContext::new(&[renban(vec![0, 10, 20])]);
    let mut grid = [0u8; 81];
    grid[0] = 2;
    grid[10] = 4;
    assert!(ctx.conflicts(&grid).is_empty());
    // 2 and 5 are too far apart for three cells
    grid[10] = 5;
    let cells = ctx.conflicts(&grid).iter().map(|c| c.0).collect::<Vec<_>>();
    assert_eq!(cells, vec![0, 10]);
}

#[test]
fn cage_keeps_only_combination_digits() {
    // 3-cell 7 cage can only be {1, 2, 4}
    let ctx = SolverContext::new(&[ConstraintSpec::Cage {
        cells: vec![0, 1, 2],
        sum: Some(7),
    }]);
    let mut board = Board::new(&[]);
    ctx.propagate(&mut board);
    for cell in [0, 1, 2] {
        assert_eq!(
            board.candidates(CellId(cell)).iter().collect::<Vec<_>>(),
            vec![1, 2, 4]
        );
    }
}

#[test]
fn arrow_bounds_circle_and_shaft() {
    let ctx = SolverContext::new(&[ConstraintSpec::Arrow {
        cells: vec![0, 1, 2],
    }]);
    let mut board = Board::new(&[]);
    ctx.propagate(&mut board);
    let digits = |board: &Board, c: u8| board.candidates(CellId(c)).iter().collect::<Vec<_>>();
    // two shaft cells sum to at least 2, and leave at most 8 for either one
    assert_eq!(digits(&board, 0), vec![2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(digits(&board, 1), vec![1, 2, 3, 4, 5, 6, 7, 8]);

    // a 3 in the circle leaves 1 and 2 for the shaft
    let mut board = Board::new(&[(CellId(0), 3)]);
    ctx.propagate(&mut board);
    assert_eq!(digits(&board, 1), vec![1, 2]);
    assert_eq!(digits(&board, 2), vec![1, 2]);
}

#[test]
fn arrow_rejects_wrong_totals_in_search() {
    // R1C1 = R2C1 + R2C2 + R3C1 has to hold in every solution
    let ctx = SolverContext::new(&[ConstraintSpec::Arrow {
        cells: vec![0, 9, 10, 18],
    }]);
    let grid = solve_brute(&ctx, &[]).unwrap();
    assert_eq!(grid[0], grid[9] + grid[10] + grid[18]);
}

#[test]
fn conflicts_flag_repeats_and_broken_rules() {
    let ctx = SolverContext::new(&[
        ConstraintSpec::Cage {
            cells: vec![18, 19],
            sum: Some(5),
        },
        ConstraintSpec::Line {
            kind: LineKind::Thermo,
            cells: vec![36, 37],
        },
    ]);
    let mut grid = [0u8; 81];
    // a repeat in row 1
    grid[0] = 4;
    grid[8] = 4;
    // fine so far: a 1 in the 5 cage, a 2 on the thermo bulb
    grid[18] = 1;
    grid[36] = 2;
    let cells = |grid: &[u8; 81]| ctx.conflicts(grid).iter().map(|c| c.0).collect::<Vec<_>>();
    assert_eq!(cells(&grid), vec![0, 8]);

    // 1 + 6 overshoots the cage, and the thermo has to rise
    grid[19] = 6;
    grid[37] = 1;
    assert_eq!(cells(&grid), vec![0, 8, 18, 19, 36, 37]);
}

#[test]
fn markers_restrict_neighbours() {
    let ctx = SolverContext::new(&[
        ConstraintSpec::Marker {
            kind: MarkerKind::Black,
            cells: [0, 1],
        },
        ConstraintSpec::Marker {
            kind: MarkerKind::Chevron,
            cells: [9, 10],
        },
    ]);
    let mut board = Board::new(&[(CellId(0), 3)]);
    ctx.propagate(&mut board);
    assert_eq!(
        board.candidates(CellId(1)).iter().collect::<Vec<_>>(),
        vec![6]
    );
    // R2C1 > R2C2, so R2C1 can't be 1 and R2C2 can't be 9
    assert!(!board.candidates(CellId(9)).contains(1));
    assert!(!board.candidates(CellId(10)).contains(9));
}

#[test]
fn invalid_specs_are_rejected() {
    let line = |cells: Vec<u8>| ConstraintSpec::Line {
        kind: LineKind::Thermo,
        cells,
    };
    assert!(line(vec![0, 1]).validate().is_ok());
    assert!(line(vec![0, 2]).validate().is_err());
    assert!(line(vec![0, 81]).validate().is_err());
    let diagonal_marker = ConstraintSpec::Marker {
        kind: MarkerKind::X,
        cells: [0, 10],
    };
    assert!(diagonal_marker.validate().is_err());
    let arrow = |cells: Vec<u8>| ConstraintSpec::Arrow { cells };
    assert!(arrow(vec![0, 10]).validate().is_ok());
    assert!(arrow(vec![0]).validate().is_err());
    assert!(arrow(vec![0, 2]).validate().is_err());
    // ten different digits can't be consecutive
    assert!(renban((0..9).collect()).validate().is_ok());
    assert!(renban((0..9).chain([17]).collect()).validate().is_err());
}

#[test]
fn showcase_is_valid_unique_and_solvable() {
    let specs = showcase::constraints();
    for spec in &specs {
        spec.validate().unwrap();
    }
    let ctx = SolverContext::new(&specs);

    // every rule holds on the solution it was read from
    assert_valid(&ctx, &showcase::SOLUTION);
    let mut grid = [0u8; 81];
    for i in 0..81u8 {
        let d = showcase::SOLUTION[i as usize];
        assert!(
            ctx.legal_digits(&grid, CellId(i)).contains(d),
            "{} = {}",
            CellId(i),
            d
        );
        grid[i as usize] = d;
    }

    // the stored givens are what the search finds today
    assert_eq!(showcase::find_givens(), showcase::GIVENS);
    let line = showcase::GIVENS;
    let board = Board::from_line(line);
    let givens: Vec<(CellId, u8)> = (0..81u8)
        .filter_map(|i| Some((CellId(i), board.value(CellId(i))?)))
        .collect();
    assert_eq!(count_solutions_for(&ctx, &givens, 2), 1);
    assert_eq!(solve_brute(&ctx, &givens), Some(showcase::SOLUTION));
    println!("showcase givens ({}): {}", givens.len(), line);
}
