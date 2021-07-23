//! The naked single technique

use crate::advanced_solver::techniques::Technique;
use crate::{GameBoard, CellValue};
use crate::game_board_controller::NoteMode;

/// Detects a naked single
pub struct NakedSingle;

impl Technique for NakedSingle {
    fn points(&self) -> u64 {
        5
    }

    fn apply_to(&self, game_board: &GameBoard) -> Result<GameBoard, ()> {

        for cell_index in game_board.iter_unset() {
            let cell = game_board.cell_value(cell_index);
            if let CellValue::Notes { .. } = cell {
                let maybe = cell.maybe_values().unwrap();
                if let &[val] = maybe.as_slice() {
                    let mut next = game_board.clone();
                    next.set(cell_index, &NoteMode::Value, val);
                    return Ok(next);
                }
            }
        }

        Err(())
    }

    fn long_name(&self) -> String {
        "Naked Single".to_string()
    }

    fn short_name(&self) -> String {
        "nkds".to_string()
    }
}

