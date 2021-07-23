//! The different ways that the solver can attempt to solve a Sudoku board.
//!
//! Each technique has a name and an associated amount of points the technique is worth.

use crate::GameBoard;

/// Represents a technique to solve a sudoku board.
///
/// All techniques must be "sound." This means that it's deduction is provable true.
pub trait Technique {
    /// The number of points this technique is worth. The large the value, the more difficult
    /// the technique.
    fn points(&self) -> u64;

    /// Apply the technique once to the game board.
    ///
    /// # Return
    ///
    /// If the technique was successfully applied, the new board is returned as `Ok(board)`. Otherwise,
    /// `Err(())` is returned.
    fn apply_to(&self, game_board: &GameBoard) -> Result<GameBoard, ()>;

    /// Gets the long form of the name of the technique
    fn long_name(&self) -> String;

    /// gets the short form of the name of the technique
    fn short_name(&self) -> String;
}

mod naked_single;
pub use naked_single::NakedSingle;

mod hidden_single;
pub use hidden_single::HiddenSingle;

mod naked_pair;
pub use naked_pair::NakedPair;