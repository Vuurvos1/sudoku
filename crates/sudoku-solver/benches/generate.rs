use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use sudoku_solver::generator::{Difficulty, Generator};

// Generation time swings per seed, as the number of attempts it takes to reach
// the difficulty varies. Every iteration generates the same batch of seeds, so
// runs stay comparable between changes. Attempts run on every core, so compare
// runs on the same machine only.
const SEEDS: u64 = 8;

fn bench_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));

    for difficulty in [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Expert,
    ] {
        group.bench_function(format!("{difficulty:?}"), |b| {
            b.iter(|| {
                for seed in 0..SEEDS {
                    Generator::seeded(seed).generate(difficulty);
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
