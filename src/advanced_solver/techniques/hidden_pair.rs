//! The hidden pair technique

use crate::advanced_solver::techniques::Technique;
use crate::GameBoard;

/// Detects a hidden pair
pub struct HiddenPair;

impl Technique for HiddenPair {
    fn points(&self) -> u64 {
        150
    }

    fn apply_to(&self, game_board: &GameBoard) -> Result<GameBoard, ()> {




        Err(())
    }

    fn long_name(&self) -> String {
        "Hidden Pair".to_string()
    }

    fn short_name(&self) -> String {
        "hdpr".to_string()
    }
}