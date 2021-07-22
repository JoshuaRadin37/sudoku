//! Create a game using a random number generator.

use crate::game_creator::GameCreator;
use crate::{GameBoard, CellIndex, CellValue};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, SeedableRng};
use rand_pcg::Pcg64;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::validity::SudokuCorrectness;
use std::collections::HashSet;
use crate::game_board_controller::NoteMode;

/// Contains a random generator to create a board
pub struct RandomLoader<R: Rng> {
    rng: R,
}

impl RandomLoader<ThreadRng> {
    /// Creates a new random generator to create a board
    pub fn new() -> Self {
        RandomLoader { rng: thread_rng() }
    }
}

impl RandomLoader<Pcg64> {
    /// use a preset seed for the rng
    pub fn from_seed(seed: u64) -> Self {
        RandomLoader {
            rng: Pcg64::seed_from_u64(seed),
        }
    }
}

/// Contains error information for the random creator
#[derive(Debug)]
pub enum RandomCreatorError {
    /// A board that doesn't adhere to sudoku rules was created
    InvalidBoardCreated,
    /// While creating the random, the selected cell was already set before
    SelectedCellAlreadySet
}

impl Display for RandomCreatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RandomCreatorError {}



impl<R: Rng> GameCreator for RandomLoader<R> {
    type Error = RandomCreatorError;

    fn into_game(mut self) -> Result<GameBoard, Self::Error> {

        let mut game_board = GameBoard::new();
        game_board.auto_note(); // create all notes

        let mut available_cells: Vec<CellIndex> =
            (0..9).into_iter()
                .flat_map(move |i| {
                    (0..9).into_iter()
                        .map(move |j| {
                            (j, i)
                        })
                })
                .collect();



        while game_board.is_valid() && !game_board.is_complete() {
            let next_index = self.rng.gen_range(0..(available_cells.len()));
            let next_cell = available_cells.remove(next_index);

            let cell = game_board[next_cell];
            if let CellValue::Notes { status: _ } = cell {
                let maybe_values = cell.maybe_values().unwrap();
                println!("Maybe values for {:?}: {:?}", next_cell, maybe_values);
                let index = self.rng.gen_range(0..maybe_values.len());
                let value = maybe_values[index];

                game_board.set(next_cell, &NoteMode::Value, value)
            } else {
                return Err(RandomCreatorError::SelectedCellAlreadySet)
            }

        }

        // after generating all values, if the board is not complete and valid, an error occured
        if game_board.is_complete() && !game_board.is_valid() {
            return Err(RandomCreatorError::InvalidBoardCreated);
        }




        Ok(game_board)
    }
}
