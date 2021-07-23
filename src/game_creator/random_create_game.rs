//! Create a game using a random number generator.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Instant;

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::game_board_controller::NoteMode;
use crate::game_creator::GameCreator;
use crate::validity::{can_be_completed, SudokuCorrectness};
use crate::{CellIndex, CellValue, GameBoard};

/// Contains a random generator to create a board
pub struct RandomLoader<R: Rng> {
    rng: R,
    /// The number of starting cells
    pub num_starting_cells: usize,
}

impl RandomLoader<ThreadRng> {
    /// Creates a new random generator to create a board
    pub fn new() -> Self {
        RandomLoader {
            rng: thread_rng(),
            num_starting_cells: 24,
        }
    }
}

impl RandomLoader<Pcg64> {
    /// use a preset seed for the rng
    pub fn from_seed(seed: u64) -> Self {
        RandomLoader {
            rng: Pcg64::seed_from_u64(seed),
            num_starting_cells: 24,
        }
    }
}

/// Contains error information for the random creator
#[derive(Debug)]
pub enum RandomCreatorError {
    /// A board that doesn't adhere to sudoku rules was created
    InvalidBoardCreated,
    /// While creating the random, the selected cell was already set before
    SelectedCellAlreadySet,
    /// The created board couldn't be undone to make a new board
    CorruptedBoardIntractable,
}

impl Display for RandomCreatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RandomCreatorError {}

struct RandomMove(CellIndex, u8);

impl RandomMove {
    fn do_move(&self, board: &mut GameBoard) {
        board.set(self.0, &NoteMode::Value, self.1)
    }

    fn undo_move(&self, board: &mut GameBoard, available_cells: &mut Vec<CellIndex>) {
        board.reset(self.0);
        board.auto_note();
        available_cells.push(self.0);
    }
}

impl<R: Rng> GameCreator for RandomLoader<R> {
    type Error = RandomCreatorError;

    fn into_game(mut self) -> Result<GameBoard, Self::Error> {
        let mut game_board = GameBoard::new();
        game_board.auto_note(); // create all notes

        let mut available_cells: Vec<CellIndex> = (0..9)
            .into_iter()
            .flat_map(move |i| (0..9).into_iter().map(move |j| (j, i)))
            .collect();

        let mut move_stack: Vec<RandomMove> = vec![];

        let start_initial_board_start = Instant::now();
        while game_board.is_valid() && !game_board.is_complete() {
            let next_index = self.rng.gen_range(0..(available_cells.len()));
            let next_cell = available_cells.remove(next_index);

            let cell = game_board[next_cell];
            if let CellValue::Notes { status: _ } = cell {
                let maybe_values = cell.maybe_values().unwrap();

                if maybe_values.is_empty() {
                    println!("I'm not sure how this wasn't already detected");
                    return Ok(game_board);
                }

                print!("Maybe values for {:?}: {:?}", next_cell, maybe_values);
                let index = self.rng.gen_range(0..maybe_values.len());
                let value = maybe_values[index];

                let next_move = RandomMove(next_cell, value);

                next_move.do_move(&mut game_board);
                move_stack.push(next_move);

                // game_board.set(next_cell, &NoteMode::Value, value);
                println!(", set to {}", value);
            } else {
                return Err(RandomCreatorError::SelectedCellAlreadySet);
            }

            println!("Checking if can be completed...");
            let time = Instant::now();
            while !can_be_completed(&game_board) {
                match move_stack.pop() {
                    None => return Err(RandomCreatorError::CorruptedBoardIntractable),
                    Some(my_move) => {
                        println!("Undoing {:?} <- {}", my_move.0, my_move.1);
                        my_move.undo_move(&mut game_board, &mut available_cells);
                    }
                }
            }
            let duration = time.elapsed();
            println!("Done in {:.3} sec", duration.as_secs_f64());
        }

        // after generating all values, if the board is not complete and valid, an error occured
        if game_board.is_complete() && !game_board.is_valid() {
            return Err(RandomCreatorError::InvalidBoardCreated);
        }

        println!(
            "Initial board created in {:.3} sec",
            start_initial_board_start.elapsed().as_secs_f64()
        );

        // Swap rows and columns

        let num_swaps = self.rng.gen_range(4..=16);

        for _ in 0..num_swaps {
            let swap_column: bool = self.rng.gen();

            let base_index = self.rng.gen_range(0usize..3) * 3;

            let index1 = self.rng.gen_range(0usize..3);
            let index2 = loop {
                let v = self.rng.gen_range(0usize..3);
                if v != index1 {
                    break v;
                }
            };

            match swap_column {
                // swap columns
                true => {
                    let col1 = base_index + index1;
                    let col2 = base_index + index2;
                    println!("Swapping columns {} and {}", col1, col2);
                    game_board.swap_columns(col1, col2);
                }
                // swap rows
                false => {
                    let row1 = base_index + index1;
                    let row2 = base_index + index2;
                    println!("Swapping rows {} and {}", row1, row2);
                    game_board.swap_rows(row1, row2);
                }
            }
        }

        let mut cells_removed = 0;

        let mut available_cells: Vec<CellIndex> = (0..9)
            .into_iter()
            .flat_map(move |i| (0..9).into_iter().map(move |j| (j, i)))
            .collect();

        let mut buffer: Vec<CellIndex> = vec![];

        while cells_removed < (81 - self.num_starting_cells) {
            if available_cells.is_empty() {
                break;
            }
            let next_index = self.rng.gen_range(0..available_cells.len());
            let index = available_cells.remove(next_index);

            let mut next = game_board.clone();

            next.reset(index);

            /*
            println!(
                "Attempting to remove {:?}",
                index
            );

             */
            if let Some(sol) = next.solutions() {
                if sol.num_solutions() == 1 {
                    println!(
                        "Cell Removal Progress: {:3.2}%",
                        cells_removed as f64 / (81 - self.num_starting_cells) as f64 * 100.0
                    );
                    game_board = next;
                    cells_removed += 1;
                    available_cells.extend(buffer);
                    buffer = vec![];
                } else {
                    buffer.push(index);
                }
            } else {
                // println!("Failed");
                buffer.push(index);
            }
        }
        for cell in (0usize..9)
            .into_iter()
            .flat_map(move |i| (0usize..9).into_iter().map(move |j| (j, i)))
        {
            if let CellValue::Value(v) = game_board[cell] {
                game_board[cell] = CellValue::Preset(v);
            }
        }

        println!("Number of starting cells: {}", 81 - cells_removed);
        println!(
            "Generated board in {:.3} sec.\nSeed: {}",
            start_initial_board_start.elapsed().as_secs_f64(),
            game_board.as_byte_string()
        );

        Ok(game_board)
    }
}
