//! A way of measuring the "uncertainty" of a value
//!
//! This value is based off of the amount of possible values per a cell

use crate::GameBoard;

/// Represents the amount of possibilities that a board has based on the quantity of maybes that the
/// board contains. This value is calculated based on the sum of the factorial of the quantity of
/// maybes in a cell.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Entropy(u64);

impl Entropy {
    /// Gets the approximate entropy of a board
    pub fn entropy(board: &GameBoard) -> Self {
        let mut entropy: u64 = 0;
        for cell in board {
            if let Some(maybe) = cell.maybe_values() {
                entropy += factorial(maybe.len());
            }
        }
        Self(entropy)
    }
}

fn factorial(n: usize) -> u64 {
    match n {
        0 | 1 => 1,
        n => n as u64 * factorial(n - 1),
    }
}
