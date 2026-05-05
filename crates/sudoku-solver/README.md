# sudoku-solver

A constraint-based Sudoku solver with step-by-step hint generation. The solver is designed to be extensible: add custom constraints for variant puzzles and plug in new solving strategies.

## Quick start

```rust
use sudoku_solver::{
    board::Board,
    constraint::standard_constraints,
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
let constraints = standard_constraints();
let ctx = SolverContext { constraints: &constraints };

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

## Adding custom constraints

Implement the `Constraint` trait to add new rules. The default `propagate()` handles the common case (eliminating a decided cell's digit from its peers), so most constraints only need `name()` and `cells()`.

### Example: diagonal constraint

In diagonal Sudoku, the two main diagonals must also contain digits 1-9 without repetition.

```rust
use sudoku_solver::{
    board::CellId,
    constraint::Constraint,
};

struct Diagonal {
    cells: Vec<CellId>,
    name: String,
}

impl Diagonal {
    fn main() -> Self {
        Diagonal {
            cells: (0..9).map(|i| CellId::from_rc(i, i)).collect(),
            name: "main diagonal".into(),
        }
    }

    fn anti() -> Self {
        Diagonal {
            cells: (0..9).map(|i| CellId::from_rc(i, 8 - i)).collect(),
            name: "anti diagonal".into(),
        }
    }
}

impl Constraint for Diagonal {
    fn name(&self) -> &str { &self.name }
    fn cells(&self) -> &[CellId] { &self.cells }
    // propagate() is inherited, no extra code needed
}
```

Then include them alongside the standard constraints:

```rust
let mut constraints = standard_constraints();
constraints.push(Box::new(Diagonal::main()));
constraints.push(Box::new(Diagonal::anti()));

let ctx = SolverContext { constraints: &constraints };
```

All generic strategies (`HiddenSingle`, `NakedSubset`, `HiddenSubset`) will automatically apply to the diagonals too.

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

| Strategy             | Type        | Difficulty |
| -------------------- | ----------- | ---------- |
| `NakedSingle`        | Placement   | 1          |
| `HiddenSingle`       | Placement   | 2          |
| `PointingPairs`      | Elimination | 3          |
| `BoxLineReduction`   | Elimination | 3          |
| `NakedSubset::<2>`   | Elimination | 4          |
| `HiddenSubset::<2>`  | Elimination | 4          |
| `NakedSubset::<3>`   | Elimination | 5          |
| `HiddenSubset::<3>`  | Elimination | 5          |
