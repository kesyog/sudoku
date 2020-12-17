use std::collections::HashSet;
use std::convert::{From, TryInto};
use std::fmt;
use std::iter::{FromIterator, Iterator};
use std::str::FromStr;

/// Check whether the given set of numbers violates the rules of Sudoku i.e. contains a repeated
/// number.
///
/// 0's are ignored as they are used as placeholders for slots that have not yet been filled in.
///
/// # Arguments
///
/// * `values` - An iterator of values to check
fn is_set_legal<T: Iterator<Item = u32>>(values: T) -> bool {
    let mut seen = HashSet::<u32>::with_capacity(9);
    for val in values.filter(|i| *i != 0_u32) {
        debug_assert!(val <= 9);
        if !seen.insert(val) {
            return false;
        }
    }
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grid {
    // Grid cells in row1, row2, etc. order
    board: [u32; 81],
}

impl Grid {
    pub const fn from_array(input: &[u32; 81]) -> Self {
        Self { board: *input }
    }

    pub fn solve(&self) -> Option<Self> {
        const ALL_SUDOKU_DIGITS: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut boards_to_check = Vec::<Self>::with_capacity(100);
        boards_to_check.push(*self);
        while !boards_to_check.is_empty() {
            let mut board = boards_to_check.pop().unwrap();
            if board.is_solved() {
                return Some(board);
            }
            // TODO: Speed up the DFS by finding the zero with the least possible digits (as
            // checked by is_legal. Can also fill in zeros that only have one possibility along the
            // way.
            let first_zero_idx = board.board.iter().position(|i| *i == 0_u32).unwrap();
            for digit in &ALL_SUDOKU_DIGITS {
                board.board[first_zero_idx] = *digit;
                if board.is_legal() {
                    boards_to_check.push(board);
                }
            }
        }
        None
    }

    pub fn as_slice(&self) -> &[u32; 81] {
        return &self.board;
    }

    fn is_solved(&self) -> bool {
        !self.board.contains(&0_u32)
    }

    fn is_legal(&self) -> bool {
        // Indices of the top left corners of each box of nine squares in a sudoku puzzle
        const NINTHS_IDXS: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];

        for i in 0..9 {
            // Check rows for repeats
            if !is_set_legal(self.board[9 * i..9 * i + 9].iter().copied()) {
                return false;
            }
            // Check columns for repeats
            if !is_set_legal(self.board[i..].iter().step_by(9).copied()) {
                return false;
            }
        }

        // Check 3x3 subsections for repeats
        for i in NINTHS_IDXS.iter().copied() {
            if !is_set_legal(
                self.board[i..i + 3]
                    .iter()
                    .chain(self.board[i + 9..i + 9 + 3].iter())
                    .chain(self.board[i + 18..i + 18 + 3].iter())
                    .copied(),
            ) {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..9).map(|i| 9 * i) {
            let row = &self.board[i..i + 9];
            for j in &row[..3] {
                write!(f, "{}", j)?;
            }
            write!(f, "|")?;
            for j in &row[3..6] {
                write!(f, "{}", j)?;
            }
            write!(f, "|")?;
            for j in &row[6..9] {
                write!(f, "{}", j)?;
            }
            // Omit newline after the last row
            if i != 8 * 9 {
                writeln!(f)?;
            }
            if i == 2 * 9 || i == 5 * 9 {
                writeln!(f, "---+---+---")?;
            }
        }
        Ok(())
    }
}

impl FromIterator<u32> for Grid {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<u32>>().into()
    }
}

impl From<Vec<u32>> for Grid {
    fn from(vec: Vec<u32>) -> Self {
        Self {
            board: vec.try_into().expect("Bad input"),
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let board = input
            .chars()
            .filter_map(|chr| chr.to_digit(10))
            .take(81)
            .collect::<Vec<u32>>();
        if board.len() != 81 {
            return Err("Not enough valid digits given. Valid values are 0-9.".to_string());
        }
        Ok(board.into())
    }
}

impl PartialEq<[u32; 81]> for Grid {
    fn eq(&self, other: &[u32; 81]) -> bool {
        self.board == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_set_legal() {
        assert!(!is_set_legal([1_u32, 2, 2].iter().copied()));
        assert!(is_set_legal([1_u32, 2, 3].iter().copied()));

        // 0's have no effect
        assert!(!is_set_legal([1_u32, 2, 2, 0].iter().copied()));
        assert!(!is_set_legal([1_u32, 2, 2, 0, 0].iter().copied()));
        assert!(is_set_legal([1_u32, 2, 3, 0].iter().copied()));
        assert!(is_set_legal([1_u32, 2, 3, 0, 0].iter().copied()));
    }

    #[test]
    fn solve_from_string() {
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
            .expect("Parsing error");
        let expected: [u32; 81] = [
            4, 8, 3, 9, 2, 1, 6, 5, 7, 9, 6, 7, 3, 4, 5, 8, 2, 1, 2, 5, 1, 8, 7, 6, 4, 9, 3, 5, 4,
            8, 1, 3, 2, 9, 7, 6, 7, 2, 9, 5, 6, 4, 1, 3, 8, 1, 3, 6, 7, 9, 8, 2, 4, 5, 3, 7, 2, 6,
            8, 9, 5, 1, 4, 8, 1, 4, 2, 5, 3, 7, 6, 9, 6, 9, 5, 4, 1, 7, 3, 8, 2,
        ];
        let solution = puzzle.solve();
        assert!(solution.is_some(), "No solution found for {}", puzzle);
        let solution = solution.unwrap();
        assert!(
            expected == solution.board,
            "\nExpected:\n{}\n\nActual:\n{}",
            Grid::from_array(&expected),
            solution
        );
        assert_eq!(&expected, solution.as_slice());
    }

    #[test]
    fn solve_from_array() {
        let puzzle: Grid = Grid::from_array(&[
            0, 0, 3, 0, 2, 0, 6, 0, 0, 9, 0, 0, 3, 0, 5, 0, 0, 1, 0, 0, 1, 8, 0, 6, 4, 0, 0, 0, 0,
            8, 1, 0, 2, 9, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 6, 7, 0, 8, 2, 0, 0, 0, 0, 2, 6,
            0, 9, 5, 0, 0, 8, 0, 0, 2, 0, 3, 0, 0, 9, 0, 0, 5, 0, 1, 0, 3, 0, 0,
        ]);
        let expected: [u32; 81] = [
            4, 8, 3, 9, 2, 1, 6, 5, 7, 9, 6, 7, 3, 4, 5, 8, 2, 1, 2, 5, 1, 8, 7, 6, 4, 9, 3, 5, 4,
            8, 1, 3, 2, 9, 7, 6, 7, 2, 9, 5, 6, 4, 1, 3, 8, 1, 3, 6, 7, 9, 8, 2, 4, 5, 3, 7, 2, 6,
            8, 9, 5, 1, 4, 8, 1, 4, 2, 5, 3, 7, 6, 9, 6, 9, 5, 4, 1, 7, 3, 8, 2,
        ];
        let solution = puzzle.solve();
        assert!(solution.is_some(), "No solution found for {}", puzzle);
        let solution = solution.unwrap();
        assert!(
            expected == solution.board,
            "\nExpected:\n{}\n\nActual:\n{}",
            Grid::from_array(&expected),
            solution
        );
        assert_eq!(&expected, solution.as_slice());
        assert_eq!(4, solution.as_slice()[0]);
    }

    #[test]
    fn no_solution() {
        let known_bad_puzzle: Grid = Grid::from_array(&[
            1, 0, 3, 0, 2, 0, 6, 0, 0, 9, 0, 0, 3, 0, 5, 0, 0, 1, 0, 0, 1, 8, 0, 6, 4, 0, 0, 0, 0,
            8, 1, 0, 2, 9, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 6, 7, 0, 8, 2, 0, 0, 0, 0, 2, 6,
            0, 9, 5, 0, 0, 8, 0, 0, 2, 0, 3, 0, 0, 9, 0, 0, 5, 0, 1, 0, 3, 0, 0,
        ]);
        assert!(known_bad_puzzle.solve().is_none());
    }
}
