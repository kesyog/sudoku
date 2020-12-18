# sudoku 🔢

A Sudoku solver in Rust written to shake off some Rust cobwebs and make some [progress](https://projecteuler.net/profile/kesyog.png)
on Project Euler.

It uses a non-recursive backtracking (depth-first search) algorithm and returns the first solution
found, if any.

## Usage

The full documentation is published [here](https://kesyog.github.io/sudoku), but the examples below
should be clear enough to show how it works.

### Examples

Ingesting a string and pretty-printing the solution:

```rust
use sudoku::Grid;

let puzzle: Grid = " \
    003020600 \
    900305001 \
    001806400 \
    008102900 \
    700000008 \
    006708200 \
    002609500 \
    800203009 \
    005010300"
    .parse()
    .unwrap();
let solution = puzzle.solve().expect("No solution found");
println!("Solution:\n{}", solution);
```

The above code will print:

```plaintext
483|921|657
967|345|821
251|876|493
---+---+---
548|132|976
729|564|138
136|798|245
---+---+---
372|689|514
814|253|769
695|417|382
```

Ingesting a `u32` array and reading the first digit of the solution:

```rust
use sudoku::Grid;

let puzzle: Grid = Grid::from_array(&[
    0, 0, 3, 0, 2, 0, 6, 0, 0, 9, 0, 0, 3, 0, 5, 0, 0, 1, 0, 0, 1, 8, 0, 6, 4, 0, 0, 0, 0,
    8, 1, 0, 2, 9, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 6, 7, 0, 8, 2, 0, 0, 0, 0, 2, 6,
    0, 9, 5, 0, 0, 8, 0, 0, 2, 0, 3, 0, 0, 9, 0, 0, 5, 0, 1, 0, 3, 0, 0,
]);
let solution = puzzle.solve().expect("No solution found");
assert_eq!(4_u32, solution.as_slice()[0]);
```
