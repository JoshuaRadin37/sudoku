//! Validity trait for Sudoku components

use crate::game_board::CellIndex;
use crate::{GameBoard, SIZE, CellValue};
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

/// Maximum solution set size before solver automatically stops
pub const MAX_SOLUTION_WIDTH: usize = 32;

/// Creates a tree representing the different solutions of a sudoku board
pub struct SolutionsTree {
    head: Node,
}

impl SolutionsTree {
    /// Creates a tree of solutions for the board
    pub fn solve(board: &GameBoard) -> Option<Self> {
        let ref mut counter = 0usize;
        let maybe_ret = Node::solve(board, counter).map(|head| Self { head });
        if *counter >= MAX_SOLUTION_WIDTH {
            None
        } else {
            maybe_ret
        }
    }

    /// Gets the number of solutions
    pub fn num_solutions(&self) -> usize {
        self.head.leaves()
    }

    /// Gets the first solution for the solutions tree
    pub fn solution(&self) -> &GameBoard {
        self.head.first_solution()
    }
}

struct Node {
    board: GameBoard,
    node_type: NodeType,
}

enum NodeType {
    Leaf,
    Branch {
        next_cell: CellIndex,
        children: HashMap<u8, Node>,
    },
}

impl Node {
    pub fn new(board: GameBoard, node_type: NodeType) -> Self {
        Node { board, node_type }
    }

    fn solve(board: &GameBoard, counter: &mut usize) -> Option<Self> {
        if *counter >= MAX_SOLUTION_WIDTH {
            return None;
        }

        let mut cell: Option<CellIndex> = None;
        'OUTER: for j in 0..9 {
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
                // Iterate through all values 0 through 9
                // Check if that value can be place. If so, create a new board with that filled and solve
                // from there, add result to this present

                let mut map = HashMap::new();

                for val in 1..=9 {
                    let mut next = board.clone();
                    next[cell_index] = CellValue::Value(val);
                    if next.is_valid() {

                        if let Some(child) = Node::solve(&next, counter) {
                            map.insert(val, child);
                        }
                    }
                }

                if map.is_empty() {
                    None
                } else {
                    let inner = NodeType::Branch {
                        next_cell: cell_index,
                        children: map
                    };
                    Some(Node::new(board.clone(), inner))
                }
            }
            None => {
                if board.is_valid() && board.is_complete() {
                    *counter += 1;
                    Some(Node::new(board.clone(), NodeType::Leaf))
                } else {
                    None
                }
            }
        }
    }

    fn leaves(&self) -> usize {
        match self.node_type {
            NodeType::Leaf => { 1 }
            NodeType::Branch { next_cell: _ , ref children } => {
                children
                    .values()
                    .map(|children| children.leaves())
                    .sum()
            }
        }
    }

    fn first_solution(&self) -> &GameBoard {
        match &self.node_type {
            NodeType::Leaf => { &self.board }
            NodeType::Branch { next_cell: _, children } => {
                for i in 1..=9 {
                    if let Some(next) = children.get(&i) {
                        return next.first_solution()
                    }
                }
                unreachable!()
            }
        }
    }
}
