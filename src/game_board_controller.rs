//! GameBoard controller

use piston::input::GenericEvent;

use crate::GameBoard;

/// Handles events for the game board
pub struct GameBoardController {
    /// Stores the game board state
    pub game_board: GameBoard,
    /// Selected cell
    pub selected_cell: Option<(usize, usize)>,
    cursor_pos: [f64; 2],
    /// Note mode
    pub note_mode: NoteMode,
    /// Set if a number should be highlighted
    pub maybe_highlighted_number: Option<u8>
}

/// The method that the controller inputs numbers in the game board
pub enum NoteMode {
    /// Set cell to this value
    Value,
    /// Set a potential value for a cell
    Maybe,
    /// Set a value that can't be in a cell
    Deny,
}

impl GameBoardController {
    /// Creates a new game board controller
    pub fn new(game_board: GameBoard) -> Self {
        GameBoardController {
            game_board,
            selected_cell: None,
            cursor_pos: [0.0; 2],
            note_mode: NoteMode::Value,
            maybe_highlighted_number: None
        }
    }

    /// Handle an event
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // find relative position of position to upper left corner
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];

            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // compute cell position
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some((cell_x, cell_y));
            } else {
                self.selected_cell = None;
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key  {
                Key::V => self.note_mode = NoteMode::Value,
                Key::D => self.note_mode = NoteMode::Deny,
                Key::M => self.note_mode = NoteMode::Maybe,
                Key::E => {
                    let string = self.game_board.as_byte_string();
                    println!("{}", string);
                }
                _ => { }
            }
            if let Some(ind) = self.selected_cell {
                match key {
                    Key::D1 => self.game_board.set(ind, &self.note_mode, 1),
                    Key::D2 => self.game_board.set(ind, &self.note_mode, 2),
                    Key::D3 => self.game_board.set(ind, &self.note_mode, 3),
                    Key::D4 => self.game_board.set(ind, &self.note_mode, 4),
                    Key::D5 => self.game_board.set(ind, &self.note_mode, 5),
                    Key::D6 => self.game_board.set(ind, &self.note_mode, 6),
                    Key::D7 => self.game_board.set(ind, &self.note_mode, 7),
                    Key::D8 => self.game_board.set(ind, &self.note_mode, 8),
                    Key::D9 => self.game_board.set(ind, &self.note_mode, 9),
                    Key::Delete | Key::Backspace => self.game_board.reset(ind),
                    _ => {}
                }
                self.maybe_highlighted_number = None;
                //self.selected_cell = None;
            } else {
                match key {
                    Key::D1 => self.maybe_highlighted_number = Some(1),
                    Key::D2 => self.maybe_highlighted_number = Some(2),
                    Key::D3 => self.maybe_highlighted_number = Some(3),
                    Key::D4 => self.maybe_highlighted_number = Some(4),
                    Key::D5 => self.maybe_highlighted_number = Some(5),
                    Key::D6 => self.maybe_highlighted_number = Some(6),
                    Key::D7 => self.maybe_highlighted_number = Some(7),
                    Key::D8 => self.maybe_highlighted_number = Some(8),
                    Key::D9 => self.maybe_highlighted_number = Some(9),
                    _ => {
                        self.maybe_highlighted_number = None;
                    }
                }
            }
        }
    }
}
