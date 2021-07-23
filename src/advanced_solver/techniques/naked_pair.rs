//! The naked pair technique

use std::collections::HashMap;

use crate::advanced_solver::techniques::Technique;
use crate::game_board_controller::NoteMode;
use crate::validity::SudokuCorrectness;
use crate::{AffectedComponents, CellIndex, CellValue, GameBoard};

/// Detects a naked pair
pub struct NakedPair;

impl NakedPair {
    /// Tries to find a naked pair.
    ///
    /// Returns a pair of cell indexes if this is successful
    pub fn find_pair<'a, I>(&self, iter: I) -> Option<(CellIndex, CellIndex)>
    where
        I: IntoIterator<Item = (CellIndex, &'a CellValue)>,
    {
        let mut pair_mapping: HashMap<_, Vec<_>> = HashMap::new();
        for (cell_index, value) in iter {
            if let Some(maybes) = value.maybe_values() {
                match maybes.as_slice() {
                    &[v1, v2] => {
                        pair_mapping.entry((v1, v2)).or_default().push(cell_index);
                    }
                    _ => {}
                }
            }
        }

        for (_key, indices) in pair_mapping {
            if let &[index1, index2] = indices.as_slice() {
                return Some((index1, index2));
            }
        }

        None
    }

    /// Enforces a pair in a row
    pub fn enforce_row_pair(
        &self,
        pair: (CellIndex, CellIndex),
        board: &GameBoard,
    ) -> Option<GameBoard> {
        let affected_row = AffectedComponents::new(&board, pair.0).row();

        self.enforce(pair, board, affected_row)
    }

    /// Enforces a pair in a column
    pub fn enforce_column_pair(
        &self,
        pair: (CellIndex, CellIndex),
        board: &GameBoard,
    ) -> Option<GameBoard> {
        let affected_col = AffectedComponents::new(&board, pair.0).column();

        self.enforce(pair, board, affected_col)
    }

    /// Enforces a pair in a house
    pub fn enforce_house_pair(
        &self,
        pair: (CellIndex, CellIndex),
        board: &GameBoard,
    ) -> Option<GameBoard> {
        let affected_house = AffectedComponents::new(&board, pair.0).house();

        self.enforce(pair, board, affected_house)
    }

    /// Enforces a generic pair for sudoku
    pub fn enforce<S: SudokuCorrectness>(
        &self,
        pair: (CellIndex, CellIndex),
        board: &GameBoard,
        comp: S,
    ) -> Option<GameBoard> {
        let mut next_board = board.clone();
        let values = board[pair.0].maybe_values().unwrap();
        let mut changed = false;
        for (index, cell) in comp
            .indices_and_cells()
            .into_iter()
            .filter(|(index, _)| *index != pair.0 && *index != pair.1)
        {
            if let Some(maybes) = cell.maybe_values() {
                if maybes.contains(&values[0]) || maybes.contains(&values[1]) {
                    next_board.set(index, &NoteMode::Deny, values[0]);
                    next_board.set(index, &NoteMode::Deny, values[1]);
                    changed = true;
                }
            }
        }

        if changed {
            Some(next_board)
        } else {
            None
        }
    }
}

impl Technique for NakedPair {
    fn points(&self) -> u64 {
        50
    }

    fn apply_to(&self, game_board: &GameBoard) -> Result<GameBoard, ()> {
        for row in game_board.rows() {
            if let Some(pair) = self.find_pair(row.indices_and_cells()) {
                if let Some(ret) = self.enforce_row_pair(pair, game_board) {
                    return Ok(ret);
                }
            }
        }

        for col in game_board.columns() {
            if let Some(pair) = self.find_pair(col.indices_and_cells()) {
                if let Some(ret) = self.enforce_column_pair(pair, game_board) {
                    return Ok(ret);
                }
            }
        }

        for house in game_board.houses() {
            if let Some(pair) = self.find_pair(house.indices_and_cells()) {
                if let Some(ret) = self.enforce_house_pair(pair, game_board) {
                    return Ok(ret);
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
