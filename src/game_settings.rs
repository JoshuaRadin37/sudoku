//! Global game settings

/// Contains global game settings
pub struct GameSettings {
    /// Automatically fill in all possible maybe values at beginning of game
    pub auto_note: bool,
    /// Automatically fill in cells with notes where no other value can be placed
    pub auto_fill: bool,
    /// When a cell is filled, remove maybes from other cells hat contain such value and couldn't now
    pub auto_remove: bool,
    /// Show if error cells are present
    pub show_errors: bool,
}

impl GameSettings {
    /// Creates a new game settings
    pub fn new() -> Self {
        Self {
            auto_note: true,
            auto_fill: false,
            auto_remove: true,
            show_errors: true,
        }
    }
}
