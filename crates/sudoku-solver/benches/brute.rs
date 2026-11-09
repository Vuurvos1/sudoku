use std::{fs, path::Path, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use serde::Deserialize;
use sudoku_solver::{
    board::CellId, constraint::ConstraintSpec, generator::count_solutions_for,
    solver::SolverContext,
};

// The app's own puzzles, read at run time so new ones are picked up.
const PUZZLE_FILES: &[&str] = &["classic.json", "variants.json"];

// Well-known classic puzzles that are hard for a backtracking search, which
// the app's classic puzzles (graded for people) mostly aren't.
const HARD_CLASSIC: &[(&str, &str, &str)] = &[
    (
        "ai-escargot",
        "AI Escargot",
        "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..",
    ),
    (
        "inkala-2012",
        "Inkala 2012",
        "8..........36......7..9.2...5...7.......457.....1...3...1....68..85...1..9....4..",
    ),
    (
        "easter-monster",
        "Easter Monster",
        "1.......2.9.4...5...6...7...5.9.3.......7.......85..4.7.....6...3...9.8...2.....1",
    ),
    // Wikipedia's example of a puzzle built against digit-order backtracking
    (
        "anti-brute-force",
        "Anti brute force",
        "..............3.85..1.2.......5.7.....4...1...9.......5......73..2.1........4...9",
    ),
];

#[derive(Deserialize)]
struct Collection {
    puzzles: Vec<Puzzle>,
}

#[derive(Deserialize)]
struct Puzzle {
    id: String,
    title: String,
    difficulty: String,
    givens: String,
    #[serde(default)]
    constraints: Vec<ConstraintSpec>,
}

fn load_puzzles() -> Vec<Puzzle> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/lib/puzzles");
    PUZZLE_FILES
        .iter()
        .flat_map(|file| {
            let path = dir.join(file);
            let json = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
            serde_json::from_str::<Collection>(&json)
                .unwrap_or_else(|e| panic!("parsing {}: {e}", path.display()))
                .puzzles
        })
        .chain(HARD_CLASSIC.iter().map(|&(id, title, givens)| Puzzle {
            id: id.to_string(),
            title: title.to_string(),
            difficulty: "brute".to_string(),
            givens: givens.to_string(),
            constraints: Vec::new(),
        }))
        .collect()
}

fn givens(line: &str) -> Vec<(CellId, u8)> {
    line.bytes()
        .enumerate()
        .filter(|&(_, b)| matches!(b, b'1'..=b'9'))
        .map(|(i, b)| (CellId(i as u8), b - b'0'))
        .collect()
}

// Counts up to two solutions, the uniqueness check: it walks the whole search
// tree, so the time doesn't hinge on how early the first solution turns up.
// Names carry the difficulty, so `cargo bench --bench brute -- expert` (or
// `-- brute`, for the classic puzzles above) runs just those.
fn bench_brute(c: &mut Criterion) {
    let mut group = c.benchmark_group("brute");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    for puzzle in load_puzzles() {
        let name = format!("{} {} {}", puzzle.difficulty, puzzle.id, puzzle.title);
        let ctx = SolverContext::new(&puzzle.constraints);
        let givens = givens(&puzzle.givens);
        // Checked in here so puzzles the filter leaves out aren't searched;
        // criterion calls this for warm-up and every sample, so only once
        let mut checked = false;
        group.bench_function(&name, |b| {
            if !checked {
                assert_eq!(
                    count_solutions_for(&ctx, &givens, 2),
                    1,
                    "{name} should have exactly one solution",
                );
                checked = true;
            }
            b.iter(|| count_solutions_for(&ctx, &givens, 2));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_brute);
criterion_main!(benches);
