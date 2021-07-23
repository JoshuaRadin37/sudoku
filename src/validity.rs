//! Validity trait for Sudoku components

use crate::game_board::CellIndex;
use crate::{CellValue, GameBoard, SIZE};
use std::collections::HashMap;
use std::time::{Duration, Instant};

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
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.indices_and_cells()
            .into_iter()
            .filter_map(|(index, cell)| cell.as_value().map(|value| (index, value)))
            .collect()
    }

    /// Gets the index and value for each cell
    fn indices_and_cells(&self) -> Vec<(CellIndex, &CellValue)>;
}

/// Allows for iterating through the indices and values that are mutable
pub trait SudokuCorrectnessMut: SudokuCorrectness {
    /// Gets the index and value for each cell
    fn indices_and_cells_mut(&mut self) -> Vec<(CellIndex, &mut CellValue)>;
}

/// Maximum solution set size before solver automatically stops
pub const MAX_SOLUTION_SIZE: usize = 128;

/// Maximum time the solver can be spent attempting to solve the board
pub const SOLVER_TIMEOUT_TIME: Duration =
    Duration::from_millis(if cfg!(debug_assertions) { 3000 } else { 500 });

/// Creates a tree representing the different solutions of a sudoku board
pub struct SolutionsTree {
    head: Node,
}

impl SolutionsTree {
    /// Creates a tree of solutions for the board
    pub fn solve(board: &GameBoard) -> Option<Self> {
        let ref mut counter = 0usize;
        let maybe_ret = Node::solve(board, counter).map(|head| Self { head });
        if *counter >= MAX_SOLUTION_SIZE {
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
        #[allow(unused)]
        next_cell: CellIndex,
        children: HashMap<u8, Node>,
    },
}

impl Node {
    pub fn new(board: GameBoard, node_type: NodeType) -> Self {
        Node { board, node_type }
    }

    fn solve_helper(board: &GameBoard, counter: &mut usize, instant: Instant) -> Option<Self> {
        if *counter >= MAX_SOLUTION_SIZE || instant.elapsed() >= SOLVER_TIMEOUT_TIME {
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
                        if let Some(child) = Node::solve_helper(&next, counter, instant) {
                            map.insert(val, child);
                        }
                    }
                    if *counter >= MAX_SOLUTION_SIZE || instant.elapsed() >= SOLVER_TIMEOUT_TIME {
                        break;
                    }
                }

                if map.is_empty() {
                    None
                } else {
                    let inner = NodeType::Branch {
                        next_cell: cell_index,
                        children: map,
                    };
                    *counter += 1;
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

    fn solve(board: &GameBoard, counter: &mut usize) -> Option<Self> {
        Self::solve_helper(board, counter, Instant::now())
    }

    fn leaves(&self) -> usize {
        match self.node_type {
            NodeType::Leaf => 1,
            NodeType::Branch {
                next_cell: _,
                ref children,
            } => children.values().map(|children| children.leaves()).sum(),
        }
    }

    fn first_solution(&self) -> &GameBoard {
        match &self.node_type {
            NodeType::Leaf => &self.board,
            NodeType::Branch {
                next_cell: _,
                children,
            } => {
                for i in 1..=9 {
                    if let Some(next) = children.get(&i) {
                        return next.first_solution();
                    }
                }
                unreachable!()
            }
        }
    }
}

/// Checks if the board at the current state can actually be finished
pub fn can_be_completed(board: &GameBoard) -> bool {
    let mut board = board.clone();

    if !board.is_valid() {
        return false;
    }
    board.clear_notes();
    board.auto_note();

    for cell in &board {
        match cell {
            CellValue::Notes { .. } => {
                if let Some(maybe) = cell.maybe_values() {
                    if maybe.is_empty() {
                        return false;
                    }
                }
            }
            CellValue::Empty => return false,
            _ => {}
        }
    }

    let mut counter = 0;
    if let Some(_) = Node::solve(&mut board, &mut counter) {
        counter > 0
    } else {
        false
    }
}
