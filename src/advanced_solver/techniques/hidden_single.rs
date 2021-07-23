//! The hidden single technique

use crate::advanced_solver::techniques::Technique;
use crate::game_board_controller::NoteMode;
use crate::{AffectedComponents, GameBoard};

/// Detects a hidden single, where a cell isn't alone in a cell, but it's the only cell that can be
/// that value in it's house, row, or column
pub struct HiddenSingle;

impl Technique for HiddenSingle {
    fn points(&self) -> u64 {
        10
    }

    fn apply_to(&self, game_board: &GameBoard) -> Result<GameBoard, ()> {
        for cell_index in game_board.iter_unset() {
            let cell = game_board[cell_index];

            let affected = AffectedComponents::new(game_board, cell_index);
            let row = affected.row();
            let column = affected.column();
            let house = affected.house();

            let maybes = cell.maybe_values().unwrap();
            for maybe in maybes {
                if row
                    .iter()
                    .map(|cell| if cell.is_or_maybe(maybe) { 1 } else { 0 })
                    .sum::<usize>()
                    == 1
                {
                    let mut next = game_board.clone();
                    next.set(cell_index, &NoteMode::Value, maybe);
                    return Ok(next);
                }

                if column
                    .iter()
                    .map(|cell| if cell.is_or_maybe(maybe) { 1 } else { 0 })
                    .sum::<usize>()
                    == 1
                {
                    let mut next = game_board.clone();
                    next.set(cell_index, &NoteMode::Value, maybe);
                    return Ok(next);
                }

                if house
                    .iter()
                    .flat_map(|row| row.iter())
                    .map(move |cell| if cell.is_or_maybe(maybe) { 1 } else { 0 })
                    .sum::<usize>()
                    == 1
                {
                    let mut next = game_board.clone();
                    next.set(cell_index, &NoteMode::Value, maybe);
                    return Ok(next);
                }
                /*




                for row in house.iter() {
                    if row.iter().map(|cell| if cell.is_or_maybe(maybe) { 1 } else { 0 })
                        .sum::<usize>() == 1
                    {

                    }
                }

                  */
            }
        }

        Err(())
    }

    fn long_name(&self) -> String {
        "Hidden Single".to_string()
    }

    fn short_name(&self) -> String {
        "hdns".to_string()
    }
}
