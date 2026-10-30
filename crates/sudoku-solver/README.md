# sudoku-solver

A constraint-based Sudoku solver with step-by-step hint generation. The solver is designed to be extensible: add custom constraints for variant puzzles and plug in new solving strategies.

## Quick start

```rust
use sudoku_solver::{
    board::Board,
    solver::{Solver, SolverContext, SolveResult},
    strategies::*,
};

// 0 = empty cell
let puzzle: [u8; 81] = [
    5,3,0, 0,7,0, 0,0,0,
    6,0,0, 1,9,5, 0,0,0,
    0,9,8, 0,0,0, 0,6,0,
    8,0,0, 0,6,0, 0,0,3,
    4,0,0, 8,0,3, 0,0,1,
    7,0,0, 0,2,0, 0,0,6,
    0,6,0, 0,0,0, 2,8,0,
    0,0,0, 4,1,9, 0,0,5,
    0,0,0, 0,8,0, 0,7,9,
];

let mut board = Board::from_array(&puzzle);
let ctx = SolverContext::standard();

let solver = Solver::new(vec![
    Box::new(NakedSingle),
    Box::new(HiddenSingle),
    Box::new(PointingPairs),
    Box::new(BoxLineReduction),
    Box::new(NakedSubset::<2>),  // naked pairs
    Box::new(NakedSubset::<3>),  // naked triples
    Box::new(HiddenSubset::<2>), // hidden pairs
    Box::new(HiddenSubset::<3>), // hidden triples
]);

match solver.solve(&mut board, &ctx) {
    SolveResult::Solved(steps) => {
        println!("Solved in {} steps", steps.len());
    }
    SolveResult::Stuck(steps) => {
        println!("Got stuck after {} steps", steps.len());
    }
    SolveResult::Invalid(steps) => {
        println!("Contradiction found after {} steps", steps.len());
    }
}
```

## Step-by-step hints

Use `Solver::step` to get one deduction at a time. Each `Deduction` includes an `Explanation` with human-readable text and cell highlights for rendering hints in a UI.

```rust
if let Some(deduction) = solver.step(&board, &ctx) {
    println!("{}", deduction.explanation.summary);
    // e.g. "Naked Pair: R1C2, R1C5 share candidates {3, 7}"
}
```

## Variant rules

A `SolverContext` holds a puzzle's rules:

- **Houses**: groups of 9 cells that contain each digit exactly once (rows, columns, boxes, X-sudoku diagonals). House-based strategies such as `HiddenSingle`, `NakedSubset` and `HiddenSubset` only look at these.
- **Constraints**: every other rule (anti-knight, cages, thermos, …), implemented with the `Constraint` trait.
- **Peers**: cell pairs that may not share a digit, derived from the houses plus each constraint's `distinct_pairs()`. Peer elimination and `ctx.sees(a, b)` use this table.

Built-in variants are described by the serializable `ConstraintSpec`:

```rust
use sudoku_solver::{constraint::ConstraintSpec, solver::SolverContext};

let ctx = SolverContext::new(&[ConstraintSpec::Diagonals, ConstraintSpec::AntiKnight]);
```

### Adding a constraint

Implement `Constraint`. Every method except `name()` and `cells()` has a default that does nothing:

- `distinct_pairs()`: cell pairs that may not repeat a digit; they become peers.
- `propagate()`: variant-specific candidate elimination (peer elimination is already done by the solver).
- `allows()`: whether a digit fits at a cell given a partial grid; the brute-force search (`count_solutions_for`, `solve_brute`, `Generator::generate_for`) relies on it.

Pass custom constraints with `SolverContext::from_parts(houses, constraints)`, or add a `ConstraintSpec` variant and build it in `constraint::build`. Houses live in the `house` module (`House`, `HouseKind`, `standard_houses()`).

## Adding custom strategies

Implement the `Strategy` trait. Strategies receive a read-only `&Board` and return an `Option<Deduction>` describing what to place or eliminate. The solver applies the deduction; strategies never modify the board directly.

```rust
use sudoku_solver::{
    board::Board,
    solver::SolverContext,
    strategy::{Deduction, Explanation, Strategy},
};

struct MyStrategy;

impl Strategy for MyStrategy {
    fn name(&self) -> &'static str { "my strategy" }

    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction> {
        // Search for a pattern, return a Deduction if found
        None
    }
}
```

Register it with the solver in the order you want it tried (simpler strategies first):

```rust
let solver = Solver::new(vec![
    Box::new(NakedSingle),
    Box::new(HiddenSingle),
    // ...
    Box::new(MyStrategy),
]);
```

## Available strategies

| Strategy                | Type        | Difficulty |
| ----------------------- | ----------- | ---------- |
| `NakedSingle`           | Placement   | 1          |
| `HiddenSingle`          | Placement   | 2          |
| `PointingPairs`         | Elimination | 3          |
| `BoxLineReduction`      | Elimination | 3          |
| `NakedSubset::<2>`      | Elimination | 4          |
| `HiddenSubset::<2>`     | Elimination | 4          |
| `NakedSubset::<3>`      | Elimination | 5          |
| `HiddenSubset::<3>`     | Elimination | 5          |
| `Fish::<2>` (x wing)    | Elimination | 6          |
| `YWing`                 | Elimination | 6          |
| `Fish::<3>` (swordfish) | Elimination | 7          |
