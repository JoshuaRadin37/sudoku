//! The algorithms that attempts to solve a sudoku board

use crate::advanced_solver::techniques::*;
use crate::GameBoard;
use std::time::{Duration, Instant};

/// The difficulty of the sudoku board
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Difficulty {
    /// Easy
    Easy = 0,
    /// Medium
    Medium = 1,
    /// Hard
    Hard = 2,
    /// Expert
    Expert = 3,
    /// Pro
    Pro = 4
}

impl From<u64> for Difficulty {
    fn from(points: u64) -> Self {
        use Difficulty::*;
        match points {
            0..=999 => Easy,
            1000..=1999 => Medium,
            2000..=2999 => Hard,
            3000..=3999 => Expert,
            _ => Pro
        }
    }
}

/// Stores the solution for a sudoku game
pub struct Solution {
    /// The solution to the game
    pub solved_board: GameBoard,
    /// The amount of points the solver got while solving the board
    pub points: u64,
    /// The difficulty of the solve
    pub difficulty: Difficulty,
    /// A list of moves made, listed as their (short, long) names
    pub moves: Vec<(String, String)>
}

/// A sudoku solver
pub struct Solver {
    techniques: Vec<Box<dyn Technique>>,
    timeout_duration: Duration
}

macro_rules! techniques {
    ($($cons:expr),*) => {
        vec![$(Box::new($cons)),*]
    };
}

impl Solver {
    /// Creates a new instance of the solver, that can timeout
    pub fn new(timeout: Duration) -> Self {
        let mut techniques: Vec<Box<dyn Technique>> =
            techniques![
                NakedSingle,
                HiddenSingle,
                NakedPair
            ];

        techniques.sort_by_key(
            |technique|
                technique.points()
        );


        Solver { techniques, timeout_duration: timeout }
    }

    /// Attempts to solve the board using known techniques. Returns either the solution, or an
    /// incomplete board that the known techniques were able to achieve.
    ///
    /// Will not brute force.
    pub fn solve(&self, board: &GameBoard) -> Result<Solution, GameBoard> {
        let mut board = board.clone(); // create solvers own sandbox for the board
        board.clear_notes(); // clear all notes in the board
        board.auto_note(); // creates own notes that are only maybes
        let mut points: u64 = 0;
        let mut moves = vec![];

        let start = Instant::now();

        let mut cont = true;
        while cont {
            cont = false;
            for technique in &self.techniques {
                if start.elapsed() >= self.timeout_duration {
                    break;
                }

                if let Ok(new_board) = technique.apply_to(&board) {
                    points += technique.points();
                    moves.push((technique.short_name(), technique.long_name()));

                    board = new_board;
                    cont = true;
                    break;
                }
            }
        }

        if board.is_victory() {
            Ok(
                Solution {
                    solved_board: board,
                    points,
                    difficulty: Difficulty::from(points),
                    moves
                }
            )
        } else {
            Err(board)
        }

    }

}