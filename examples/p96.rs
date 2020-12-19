use rayon::prelude::*;
use std::fs;
use std::time::Instant;
use sudoku::Grid;

// Solve a puzzle and return the result of reading the first three digits of the first row as a
// three-digit number
fn get_first_three_digits(grid: &Grid) -> u32 {
    let solution = grid.solve().unwrap();
    let solution_digits = solution.as_slice();
    solution_digits[0] * 100 + solution_digits[1] * 10 + solution_digits[2]
}

fn main() {
    let input_file = fs::read_to_string("examples/p096_sudoku.txt").expect("Input file not found");
    let mut puzzles: Vec<Grid> = Vec::with_capacity(50);
    let mut temp = String::with_capacity(100);

    // Ugly parsing code that makes a lot of assumptions about how the input file is structured
    for line in input_file.split('\n') {
        if &line[0..1] == "G" {
            if !temp.is_empty() {
                puzzles.push(temp.parse::<Grid>().unwrap());
                temp.clear()
            }
            continue;
        }
        temp.push_str(line);
    }
    puzzles.push(temp.parse::<Grid>().unwrap());

    let start = Instant::now();
    let sum: u32 = puzzles
        .iter()
        .map(|grid| get_first_three_digits(grid))
        .sum();
    let duration = start.elapsed();
    println!("Single thread\nAnswer: {}\nTotal time: {:?}", sum, duration);

    let start = Instant::now();
    let sum: u32 = puzzles
        .par_iter()
        .map(|grid| get_first_three_digits(grid))
        .sum();
    let duration = start.elapsed();
    println!("Parallelized\nAnswer: {}\nTotal time: {:?}", sum, duration);
}
