//! Create a game using a custom formatted byte string
//!
//! Byte strings are formatted where the bottom 6 bits of 2 bytes are used to store the x+1, y+1, and val+1
//! for each cell. This byte string is concluded by a 0,0,0 entry. The first two bits are always 10.
//!
//! # Example
//!
//! Let's say that cell 0,0 is 1 and cell 2,3 is 3. The byte string would be:
//! `[0b01000100, 0b01010010, 0b01001101, 0b01000100]`, which would be represented by the string
//! `"DRMD"`

use crate::game_creator::GameCreator;
use crate::GameBoard;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Stores the byte string that represents a game board
pub struct ByteStringLoader(Vec<u8>);

#[derive(Debug)]
pub struct ByteStringFormError(String);

impl Display for ByteStringFormError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ByteStringFormError {}

impl ByteStringLoader {
    /// Creates a new byte string loader from a string
    pub fn from_string<S: AsRef<str>>(string: S) -> Self {
        let bytes = string.as_ref().bytes().collect();
        Self(bytes)
    }
}

bitfield! {
    struct CellBytes(u16);

    u8, x, _: 11, 8;
    u8, y, _: 7, 4;
    u8, val, _: 3, 0;
}

impl CellBytes {
    fn new(upper: u8, lower: u8) -> Self {
        let upper = upper & !0b11000000;
        let lower = lower & !0b11000000;
        let expanded = (upper as u16) << 6;
        let full = expanded | (lower as u16);
        CellBytes(full)
    }
}

impl GameCreator for ByteStringLoader {
    type Error = ByteStringFormError;

    fn into_game(self) -> Result<GameBoard, Self::Error> {
        if self.0.len() % 2 != 0 {
            return Err(ByteStringFormError(
                "Odd number of bytes present in byte string".to_string(),
            ));
        }

        let mut vector: Vec<((usize, usize), u8)> = vec![];

        let mut iterator = self.0.into_iter();
        loop {
            let upper = iterator.next().ok_or_else(|| {
                ByteStringFormError("Iterator empty when not expected".to_string())
            })?;
            let lower = iterator.next().ok_or_else(|| {
                ByteStringFormError("Iterator empty when not expected".to_string())
            })?;

            let cell = CellBytes::new(upper, lower);
            if cell.0 == 0 {
                break;
            }

            let x = cell.x() as usize - 1;
            let y = cell.y() as usize - 1;
            let val = cell.val() - 1;

            vector.push(((x, y), val));
        }

        Ok(GameBoard::new().with_presets(vector))
    }
}
