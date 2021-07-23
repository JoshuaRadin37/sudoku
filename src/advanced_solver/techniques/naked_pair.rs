//! The naked pair technique

use crate::advanced_solver::techniques::Technique;
use crate::{GameBoard, CellValue, AffectedComponents};
use crate::game_board_controller::NoteMode;
use crate::validity::SudokuCorrectness;

/// Detects a naked pair
pub struct NakedPair;

impl Technique for NakedPair {
    fn points(&self) -> u64 {
        50
    }

    fn apply_to(&self, game_board: &GameBoard) -> Result<GameBoard, ()> {
        for cell_index in game_board.iter_unset() {
            let cell = game_board[cell_index];
            if let CellValue::Notes { status } = cell {
                let maybes = cell.maybe_values().unwrap();
                if maybes.len() == 2 {

                    let affected = AffectedComponents::new(game_board, cell_index);
                    let row = affected.row();
                    let column = affected.column();
                    let house = affected.house();





                }
            }
        }

        Err(())
    }

    fn long_name(&self) -> String {
        "Naked Pair".to_string()
    }

    fn short_name(&self) -> String {
        "nkpr".to_string()
    }
}