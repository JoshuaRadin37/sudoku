//! Validity trait for Sudoku components

use crate::game_board::CellIndex;
use crate::{SIZE, GameBoard};
use std::collections::HashMap;

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

/// Creates a tree representing the different solutions of a sudoku board
pub struct SolutionsTree {
    head: Node
}

impl SolutionsTree {

    /// Creates a tree of solutions for the board
    pub fn solve(board: &GameBoard) -> Option<Self> {
        Node::solve(board).map(|head| Self { head })
    }

    /// Gets the number of solutions
    pub fn num_solutions(&self) -> usize {
        self.head.leaves()
    }
}

struct Node {
    board: GameBoard,
    cell: Option<CellIndex>,
    children: HashMap<u8, Node>
}

impl Node {
    fn new(board: GameBoard, cell: Option<CellIndex>, children: HashMap<u8, Node>) -> Self {
        Node { board, cell, children }
    }

    fn solve(board: &GameBoard) -> Option<Self> {
        let mut cell: Option<CellIndex> = None;
        'OUTER:
        for j in 0..9 {
            for i in 0..9 {
                let index: CellIndex = (j, i);
                let value = board.cell_value(index);
                if value.as_value().is_none() {
                    cell = Some(index);
                    break 'OUTER;
                }
            }
        }

        match cell {
            Some(cell_index) => {
                todo!()
            },
            None => {
                if board.is_valid() && board.is_complete() {
                    Some(Node::new(board.clone(), None, HashMap::new()))
                } else {
                    None
                }
            }
        }
    }

    fn leaves(&self) -> usize {
        match self.cell {
            None => { 1 }
            Some(_) => {
                self.children.values()
                    .map(|children| children.leaves())
                    .sum()
            }
        }
    }
}

