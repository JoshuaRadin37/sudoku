//! Methods for creating sudoku games
//!
//! There can be multiple methods for creating games:
//!     1. loading a new game from a byte string
//!     2. creating a game from a random generator
//!     3. creating a new game that can be exported
//!     4. creating a game from a json

use crate::GameBoard;
use std::error::Error;

mod json_create_game;
pub use json_create_game::JSONLoader;

mod byte_string_create_game;
pub use byte_string_create_game::ByteStringLoader;

mod random_create_game;

/// Helper trait for generating games
pub trait GameCreator {
    /// The error type if something goes wrong while generating a game
    type Error: Error;

    /// builds the game creator into a game board
    fn into_game(self) -> Result<GameBoard, Self::Error>;
}
