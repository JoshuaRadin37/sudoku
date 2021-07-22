//! Validity trait for Sudoku components

use crate::game_board::CellIndex;
use crate::SIZE;

/// A helper trait that is used to determine whether a component of a cell is invalid
pub trait SudokuCorrectness {
    /// Checks whether the component is valid, by not having repeating cells
    fn is_valid(&self) -> bool {
        self.invalid_cells().is_empty()
    }
    /// Cell indices for invalid cells
    fn invalid_cells(&self) -> Vec<CellIndex> {
        let mut found_array: [Result<Option<CellIndex>, ()>; SIZE] = [Ok(None); SIZE];
        let mut invalid = vec![];

        for (index, val) in self.indices_and_values() {
            match found_array[(val - 1) as usize] {
                Ok(None) => {
                    found_array[(val - 1) as usize] = Ok(Some(index));
                }
                Ok(Some(old)) => {
                    invalid.push(old);
                    invalid.push(index);
                    found_array[(val - 1) as usize] = Err(());
                }
                Err(_) => invalid.push(index),
            }
        }

        invalid
    }

    /// Whether all cells in the component are filled in and the component is valid
    fn is_complete(&self) -> bool {
        let mut found = [0; SIZE];
        for (_, val) in self.indices_and_values() {
            found[(val - 1) as usize] += 1;
        }

        found.iter().all(|&v| v == 1)
    }

    /// Gets the index and value for each FILLED cell
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)>;
}
