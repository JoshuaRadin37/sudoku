//! Create a game using a json formatted string

use crate::game_creator::GameCreator;
use crate::GameBoard;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Stores the JSON string to load the game from.
///
/// JSON strings should be formatted as follows:
/// ```json
/// [
///     {
///         "x": <column>
///         "y": <row>
///         "val": <value>
///     },
///     .
///     .
///     .
/// ]
/// ```
pub struct JSONLoader(String);

impl JSONLoader {
    /// Creates the JSONLoader from a string
    pub fn from_string<S: AsRef<str>>(string: S) -> JSONLoader {
        JSONLoader(string.as_ref().to_string())
    }

    /// Tries to create a JSONLoader from the contents of a file
    ///
    /// # Error:
    /// This function will result in an error if an [IO error] occurs
    ///
    /// [IO error]: std::io::Error
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<JSONLoader> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(JSONLoader(buffer))
    }
}

#[derive(Deserialize, Serialize)]
struct JSONCellEntry {
    x: usize,
    y: usize,
    val: u8,
}

impl GameCreator for JSONLoader {
    type Error = serde_json::Error;

    fn into_game(self) -> Result<GameBoard, Self::Error> {
        let values: Vec<JSONCellEntry> = serde_json::from_str(self.0.as_str())?;

        let iter = values.into_iter().map(|entry| {
            let JSONCellEntry { x, y, val } = entry;
            ((x, y), val)
        });

        let board = GameBoard::new().with_presets(iter);

        Ok(board)
    }
}
