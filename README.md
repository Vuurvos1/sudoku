# Sudoku

A Rust constraint-based Sudoku solver paired with a (work-in-progress) Tauri + SvelteKit app.

## Project structure

- [`crates/sudoku-solver`](crates/sudoku-solver) - Constraint-based solver with pluggable strategies (naked single, hidden single, pointing pair, box/line reduction) that produce step-by-step `Deduction`s with explanations, intended for both solving and hint generation.
- [`src`](src), [`src-tauri`](src-tauri) — SvelteKit frontend and Tauri shell for the desktop app. Still WIP.

## Running the app (WIP)

```sh
pnpm install
pnpm tauri dev
```

## Solver tests

```sh
cargo test --package sudoku-solver
```

Or a single test file, e.g.:

```sh
cargo test --package sudoku-solver --test hidden_single
```

## Solver benchmarks

Criterion benches live in [`crates/sudoku-solver/benches`](crates/sudoku-solver/benches).

```sh
cargo bench --package sudoku-solver --bench solve
```

Re-runs compare against the previous run stored in `target/criterion/`, so regressions are reported automatically.
