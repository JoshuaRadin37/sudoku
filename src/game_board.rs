//! Game board logic

use crate::game_board_controller::NoteMode;
use crate::validity::{SudokuCorrectness, SolutionsTree};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// The size of the game board
pub const SIZE: usize = 9;

#[derive(Clone)]
/// Stores game board information
pub struct GameBoard {
    /// Stores the contents of the cells.
    /// 0 is an empty cell
    pub cells: [[CellValue; SIZE]; SIZE],
}

/// Type for the row index
pub type RowIndex = usize;
/// Type for the column index
pub type ColumnIndex = usize;
/// Type for a cell in the game board
pub type CellIndex = (ColumnIndex, RowIndex);

/// Column type
pub struct Column<'a> {
    /// The cells within the column
    pub cells: Vec<&'a CellValue>,
    col_n: usize,
}

impl<'a> Deref for Column<'a> {
    type Target = Vec<&'a CellValue>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl SudokuCorrectness for Column<'_> {
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                if let Some(value) = cell.as_value() {
                    Some(((self.col_n, index), value))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// A mutable reference to a column in a gameboard
pub struct ColumnMut<'a> {
    board: &'a mut GameBoard,
    col_n: usize,
}

impl<'a> ColumnMut<'a> {
    fn new(board: &'a mut GameBoard, col_n: usize) -> Self {
        ColumnMut { board, col_n }
    }

    /// Gets the mutable cell at an index
    pub fn cell_mut(&mut self, row: usize) -> Option<&mut CellValue> {
        match row {
            0..=8 => self.board.cells[row].get_mut(self.col_n),
            _ => None,
        }
    }

    /// Gets the cell at an index
    pub fn cell(&self, row: usize) -> Option<&CellValue> {
        match row {
            0..=8 => self.board.cells[row].get(self.col_n),
            _ => None,
        }
    }

    /// Gets the cells within the column
    pub fn cells(&self) -> impl IntoIterator<Item = &CellValue> {
        self.board.column(self.col_n).unwrap().cells
    }
}

impl Index<usize> for ColumnMut<'_> {
    type Output = CellValue;

    fn index(&self, index: usize) -> &Self::Output {
        self.cell(index).unwrap()
    }
}

impl IndexMut<usize> for ColumnMut<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.cell_mut(index).unwrap()
    }
}

impl SudokuCorrectness for ColumnMut<'_> {
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        let mut ret = vec![];

        for i in 0..9 {
            if let Some(value) = self.cell(i).unwrap().as_value() {
                ret.push(((self.col_n, i), value));
            }
        }

        ret
    }
}

/// Row type
pub struct Row<'a> {
    /// The cells within the row
    pub cells: &'a [CellValue; SIZE],
    row_n: usize,
}

impl Deref for Row<'_> {
    type Target = [CellValue; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl SudokuCorrectness for Row<'_> {
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                if let Some(value) = cell.as_value() {
                    Some(((index, self.row_n), value))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Mutable row type
pub struct RowMut<'a> {
    /// The cells within the row
    pub cells: &'a mut [CellValue; SIZE],
    row_n: usize,
}

impl<'a> RowMut<'a> {
    /// Converts a mutable row into an immutable row
    pub fn into_row(self) -> Row<'a> {
        Row {
            cells: self.cells,
            row_n: self.row_n,
        }
    }
}

impl Deref for RowMut<'_> {
    type Target = [CellValue; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl DerefMut for RowMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl SudokuCorrectness for RowMut<'_> {
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                if let Some(value) = cell.as_value() {
                    Some(((index, self.row_n), value))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// House type
pub struct House<'a> {
    /// House cells
    pub cells: Vec<&'a [CellValue]>,
    house_first_x: usize,
    house_first_y: usize,
}

impl<'a> Deref for House<'a> {
    type Target = Vec<&'a [CellValue]>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl SudokuCorrectness for House<'_> {
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(row_n, &row)| {
                let true_row = self.house_first_y + row_n;
                row.iter().enumerate().filter_map(move |(col_n, cell)| {
                    if let Some(val) = cell.as_value() {
                        let true_col = self.house_first_x + col_n;
                        Some(((true_col, true_row), val))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }
}

/// Mutable House type
pub struct HouseMut<'a> {
    board: &'a mut GameBoard,
    house_first_x: usize,
    house_first_y: usize,
}

impl<'a> HouseMut<'a> {
    /// Gets the cells within the house
    pub fn cells(&self) -> impl IntoIterator<Item = &CellValue> {
        let mut ret = vec![];
        for j in 0..3 {
            for i in 0..3 {
                let x = self.house_first_x + i;
                let y = self.house_first_y + j;
                let cell = self.board.cell_value((x, y));
                ret.push(cell);
            }
        }
        ret
    }

    /// Gets a cell in the house, treated as a 3,3 array
    pub fn cell(&self, x: usize, y: usize) -> Option<&CellValue> {
        let x = self.house_first_x + x;
        let y = self.house_first_y + y;
        self.board.cells.get(y).map(move |row| row.get(x)).flatten()
    }

    /// Gets the mutable cell in the house, treated as a 3,3 array
    pub fn mut_cell(&mut self, x: usize, y: usize) -> Option<&mut CellValue> {
        let x = self.house_first_x + x;
        let y = self.house_first_y + y;
        self.board
            .cells
            .get_mut(y)
            .map(move |row| row.get_mut(x))
            .flatten()
    }
}

impl<'a> Index<usize> for HouseMut<'a> {
    type Output = [CellValue];

    fn index(&self, index: usize) -> &Self::Output {
        let adjusted = &self.board.cells[self.house_first_y..(self.house_first_y + 3)];
        let row = &adjusted[index][self.house_first_x..(self.house_first_x + 3)];
        row
    }
}

impl<'a> IndexMut<usize> for HouseMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let adjusted = &mut self.board.cells[self.house_first_y..(self.house_first_y + 3)];
        let row = &mut adjusted[index][self.house_first_x..(self.house_first_x + 3)];
        row
    }
}

impl SudokuCorrectness for HouseMut<'_> {
    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.board
            .cells
            .iter()
            .skip(self.house_first_y) // skip to the first in the house
            .take(3) // only take 3
            .enumerate()
            .map(|(row_n, row)| {
                let true_row = self.house_first_y + row_n;
                row.iter()
                    .skip(self.house_first_x) // skip to the first in the row
                    .take(3) // only take 3
                    .enumerate()
                    .filter_map(move |(col_n, cell)| {
                        if let Some(val) = cell.as_value() {
                            let true_col = self.house_first_x + col_n;
                            Some(((true_col, true_row), val))
                        } else {
                            None
                        }
                    })
            })
            .flatten()
            .collect()
    }
}

#[derive(Copy, Clone, Debug)]
/// The possible values that a cell can have
pub enum CellValue {
    /// A value present at the beginning of a sudoku game. Can not be changed
    Preset(u8),
    /// A value input by the user that can be changed
    Value(u8),
    /// Possible values set by the user
    Notes {
        /// All values of the board can have a status
        status: [Option<NoteStatus>; 9],
    },
    /// The cell is empty
    Empty,
}

impl CellValue {
    /// Gets the value of the cell, if it has a concrete one.
    pub fn as_value(&self) -> Option<u8> {
        match self {
            &CellValue::Preset(v) => Some(v),
            &CellValue::Value(v) => Some(v),
            CellValue::Notes { .. } => None,
            CellValue::Empty => None,
        }
    }

    /// Gets the values that this cell could be
    pub fn maybe_values(&self) -> Option<Vec<u8>> {
        match self {
            CellValue::Notes { status } => {
                let mut ret = vec![];
                for val_status in status.iter().enumerate().map(|(i, s)| (i as u8 + 1, s)) {
                    if let (val, Some(NoteStatus::Maybe)) = val_status {
                        ret.push(val);
                    }
                }
                Some(ret)
            }
            _ => None
        }
    }
}

/// Whether or not this note is number is maybe or deny
#[derive(Copy, Clone, Debug)]
pub enum NoteStatus {
    /// This cell can be this value
    Maybe,
    /// This cell can't be this value
    Deny,
}

impl GameBoard {
    /// Creates a new game board
    pub fn new() -> Self {
        Self {
            cells: [[CellValue::Empty; SIZE]; SIZE],
        }
    }

    /// Sets preset, immutable cells within the board
    pub fn with_presets<I>(mut self, presets: I) -> Self
    where
        I: IntoIterator<Item = ((usize, usize), u8)>,
    {
        for ((x, y), val) in presets {
            self.cells[y][x] = CellValue::Preset(val);
        }
        self
    }

    /// Gets the character at cell location
    pub fn cell_value(&self, ind: CellIndex) -> &CellValue {
        &self.cells[ind.1][ind.0]
    }

    /// Set cell value
    pub fn set(&mut self, ind: (usize, usize), mode: &NoteMode, val: u8) {
        let ref mut cell = self.cells[ind.1][ind.0];
        if let CellValue::Preset(_) = cell {
            return;
        }

        match mode {
            NoteMode::Value => {
                *cell = CellValue::Value(val);

                let affected_components = AffectedComponentsMut::new(self, ind);
                let row_mut = affected_components.row();
                for cell in row_mut.cells {
                    if let CellValue::Notes { status } = cell {
                        status[(val - 1) as usize] = None;
                    }
                }

                let affected_components = AffectedComponentsMut::new(self, ind);
                let mut column = affected_components.column();
                for i in 0..9 {
                    let cell = column.cell_mut(i).unwrap();
                    if let CellValue::Notes { status } = cell {
                        status[(val - 1) as usize] = None;
                    }
                }

                let affected_components = AffectedComponentsMut::new(self, ind);
                let mut house = affected_components.house();
                for j in 0..3 {
                    for i in 0..3 {
                        let cell = house.mut_cell(i, j).unwrap();
                        if let CellValue::Notes { status } = cell {
                            status[(val - 1) as usize] = None;
                        }
                    }
                }
            }
            NoteMode::Maybe => match cell {
                CellValue::Preset(_) => {}
                CellValue::Value(_) => {}
                CellValue::Notes { status } => {
                    if let Some(NoteStatus::Maybe) = status[(val - 1) as usize] {
                        status[(val - 1) as usize] = None;
                    } else {
                        status[(val - 1) as usize] = Some(NoteStatus::Maybe);
                    }
                }
                CellValue::Empty => {
                    let mut status = [None; SIZE];
                    status[(val - 1) as usize] = Some(NoteStatus::Maybe);
                    *cell = CellValue::Notes { status };
                }
            },
            NoteMode::Deny => match cell {
                CellValue::Preset(_) => {}
                CellValue::Value(_) => {}
                CellValue::Notes { status } => {
                    if let Some(NoteStatus::Deny) = status[(val - 1) as usize] {
                        status[(val - 1) as usize] = None;
                    } else {
                        status[(val - 1) as usize] = Some(NoteStatus::Deny);
                    }
                }
                CellValue::Empty => {
                    let mut status = [None; SIZE];
                    status[(val - 1) as usize] = Some(NoteStatus::Deny);
                    *cell = CellValue::Notes { status };
                }
            },
        }
        //println!("Cell {:?} set to {:?}", ind, cell);
    }

    /// Clears the value in a cell. Can't reset a preset cell
    pub fn reset(&mut self, ind: (usize, usize)) {
        match self.cells[ind.1][ind.0] {
            CellValue::Preset(_) => {}
            _all => {
                self.cells[ind.1][ind.0] = CellValue::Empty;
                //println!("Cell {:?} set to {:?}", ind, self.cells[ind.1][ind.0]);
            }
        }
    }

    /// Gets a row from the board
    pub fn row(&self, index: usize) -> Option<Row> {
        self.cells.get(index).map(|raw_row| Row {
            cells: raw_row,
            row_n: index,
        })
    }

    /// Gets a mutable row from the board
    pub fn row_mut(&mut self, index: usize) -> Option<RowMut> {
        self.cells.get_mut(index).map(|raw_row| RowMut {
            cells: raw_row,
            row_n: index,
        })
    }

    /// Gets a column from the board
    pub fn column(&self, index: usize) -> Option<Column> {
        match index {
            0..=8 => {
                let mut ret = vec![];

                for row in 0..9 {
                    let ref cell = self.cells[row][index];
                    ret.push(cell);
                }

                Some(Column {
                    cells: ret,
                    col_n: index,
                })
            }
            _ => None,
        }
    }

    /// Gets a column of mutable cells from the board
    pub fn column_mut(&mut self, index: usize) -> Option<ColumnMut> {
        match index {
            0..=8 => Some(ColumnMut::new(self, index)),
            _ => None,
        }
    }

    /// Gets the specified house, where houses are indexed as a 2D array of size 3,3
    pub fn house(&self, x: usize, y: usize) -> Option<House> {
        match (x, y) {
            (0..=2, 0..=2) => {
                let mut ret = vec![];
                let start_row = x * 3;
                let start_column = y * 3;
                let column_range = start_column..(start_column + 3);

                for j in 0..3 {
                    let ref row = self.cells[start_row + j][column_range.clone()];
                    ret.push(row);
                }

                Some(House {
                    cells: ret,
                    house_first_x: start_column,
                    house_first_y: start_row,
                })
            }
            _ => None,
        }
    }

    /// Gets the specified house of mutable cells, where houses are indexed as a 2D array of size 3,3
    pub fn house_mut(&mut self, x: usize, y: usize) -> Option<HouseMut> {
        match (x, y) {
            (0..=2, 0..=2) => {
                let start_row = x * 3;
                let start_column = y * 3;

                Some(HouseMut {
                    board: self,
                    house_first_x: start_column,
                    house_first_y: start_row,
                })
            }
            _ => None,
        }
    }

    /// Gets an iterator of all columns in the game board
    pub fn columns(&self) -> impl IntoIterator<Item = Column> {
        (0..9)
            .into_iter()
            .map(move |index| self.column(index).unwrap())
    }

    /// Gets an iterator of all rows in the game board
    pub fn rows(&self) -> impl IntoIterator<Item = Row> {
        (0..9)
            .into_iter()
            .map(move |index| self.row(index).unwrap())
    }

    /// Gets an iterator for all houses in the game board
    pub fn houses(&self) -> impl IntoIterator<Item = House> {
        (0..3)
            .into_iter()
            .map(move |row| {
                (0..3)
                    .into_iter()
                    .map(move |col| self.house(col, row).unwrap())
            })
            .flatten()
    }

    /// Gets an iterator of all components within the game board
    fn sudoku_components<'a>(
        &'a self,
    ) -> impl IntoIterator<Item = Box<dyn 'a + SudokuCorrectness>> {
        let mut vec: Vec<Box<dyn SudokuCorrectness>> = vec![];
        vec.extend(self.rows().into_iter().map(|row| {
            let ret: Box<dyn SudokuCorrectness> = Box::new(row);
            ret
        }));
        vec.extend(self.columns().into_iter().map(|row| {
            let ret: Box<dyn SudokuCorrectness> = Box::new(row);
            ret
        }));
        vec.extend(self.houses().into_iter().map(|row| {
            let ret: Box<dyn SudokuCorrectness> = Box::new(row);
            ret
        }));
        vec
    }

    /// gets the byte string equivalent of the board
    pub fn as_byte_string(&self) -> String {
        let mut buffer: Vec<u8> = Vec::new();

        for (row_n, row) in self.cells.iter().enumerate() {
            for (col_n, cell) in row.iter().enumerate() {
                if let Some(value) = cell.as_value() {
                    let col = col_n + 1;
                    let row = row_n + 1;
                    let val = value + 1;
                    let high = (0b01000000 | (col << 2) | (row >> 2)) as u8;
                    let low = 0b01000000 | ((row << 4) & 0b110000) as u8 | (val);
                    buffer.push(high);
                    buffer.push(low);
                }
            }
        }

        buffer.push(0b01000000);
        buffer.push(0b01000000);
        String::from_utf8(buffer).unwrap()
    }

    /// Automatically fully notes the game board
    pub fn auto_note(&mut self) {
        self.clear_notes();

        for row in 0usize..9 {
            for column in 0usize..9 {
                if !self.is_valid() {
                    return;
                }
                let cell_index = (column, row);
                if let None = self.cell_value(cell_index).as_value() {
                    let mut valid: Vec<u8> = vec![];
                    for val in 1u8..=9 {
                        let old = self.cells[row][column];
                        self.cells[row][column] = CellValue::Value(val);
                        let affected = AffectedComponents::new(self, cell_index);
                        if affected.house().is_valid()
                            && affected.row().is_valid()
                            && affected.column().is_valid()
                        {
                            valid.push(val);
                        }
                        self.cells[row][column] = old;
                    }
                    for value in valid {
                        self.set(cell_index, &NoteMode::Maybe, value);
                    }
                }
            }
        }
    }

    /// Clears all notes
    pub fn clear_notes(&mut self) {
        for row in 0usize..9 {
            for column in 0usize..9 {
                if let CellValue::Notes { .. } = self.cell_value((column, row)) {
                    self.reset((column, row));
                }
            }
        }
    }

    /// Solves the board. Returns whether the solve was successful
    pub fn solve(&mut self) -> bool {
        for row in 0usize..9 {
            for column in 0usize..9 {
                let cell_index = (column, row);
                if let None = self.cell_value(cell_index).as_value() {
                    let mut viable = false;
                    for val in 1u8..=9 {
                        self.cells[row][column] = CellValue::Value(val);
                        if self.is_valid() {
                            let mut next = self.clone();
                            if next.solve() {
                                *self = next;
                                viable = true;
                                break;
                            }
                        }
                    }
                    if !viable {
                        return false;
                    }
                }
            }
        }

        self.is_valid() && self.is_complete()
    }

    /// Returns a solutions tree for the given board
    pub fn solutions(&self) -> Option<SolutionsTree> {
        SolutionsTree::solve(self)
    }

}

impl SudokuCorrectness for GameBoard {
    fn is_valid(&self) -> bool {
        for component in self.sudoku_components() {
            if !component.is_valid() {
                return false;
            }
        }
        true
    }

    fn invalid_cells(&self) -> Vec<CellIndex> {
        let set: HashSet<_> = self
            .sudoku_components()
            .into_iter()
            .map(|comp| comp.invalid_cells())
            .flatten()
            .collect();

        Vec::from_iter(set)
    }

    fn is_complete(&self) -> bool {
        for component in self.sudoku_components() {
            if !component.is_complete() {
                return false;
            }
        }
        true
    }

    fn indices_and_values(&self) -> Vec<(CellIndex, u8)> {
        self.rows()
            .into_iter()
            .flat_map(|row| row.indices_and_values())
            .collect()
    }
}

impl Index<usize> for GameBoard {
    type Output = [CellValue; 9];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for GameBoard {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl Index<CellIndex> for GameBoard {
    type Output = CellValue;

    fn index(&self, index: CellIndex) -> &Self::Output {
        &self[index.1][index.0]
    }
}

impl IndexMut<CellIndex> for GameBoard {
    fn index_mut(&mut self, index: CellIndex) -> &mut Self::Output {
        &mut self[index.1][index.0]
    }
}

/// A convenience struct to get the row, column, and house "seen" by a cell at a given index
pub struct AffectedComponentsMut<'a> {
    index: CellIndex,
    board: &'a mut GameBoard,
}

impl<'a> AffectedComponentsMut<'a> {
    /// Creates a new instance
    pub fn new(board: &'a mut GameBoard, index: CellIndex) -> Self {
        AffectedComponentsMut { index, board }
    }

    /// The affected row
    pub fn row(self) -> RowMut<'a> {
        self.board.row_mut(self.index.1).unwrap()
    }

    /// The affected column
    pub fn column(self) -> ColumnMut<'a> {
        self.board.column_mut(self.index.0).unwrap()
    }

    /// The affected house
    pub fn house(self) -> HouseMut<'a> {
        self.board
            .house_mut(self.index.1 / 3, self.index.0 / 3)
            .unwrap()
    }
}

/// A convenience struct to get the row, column, and house "seen" by a cell at a given index
pub struct AffectedComponents<'a> {
    index: CellIndex,
    board: &'a GameBoard,
}

impl<'a> AffectedComponents<'a> {
    /// Creates a new instance
    pub fn new(board: &'a GameBoard, index: CellIndex) -> Self {
        AffectedComponents { index, board }
    }

    /// The affected row
    pub fn row(&self) -> Row<'a> {
        self.board.row(self.index.1).unwrap()
    }

    /// The affected column
    pub fn column(&self) -> Column<'a> {
        self.board.column(self.index.0).unwrap()
    }

    /// The affected house
    pub fn house(&self) -> House<'a> {
        self.board
            .house(self.index.1 / 3, self.index.0 / 3)
            .unwrap()
    }

    /// Checks whether all the components are valid
    pub fn is_valid(&self) -> bool {
        self.row().is_valid() && self.column().is_valid() && self.house().is_valid()
    }
}
