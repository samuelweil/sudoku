mod load;

use std::ops::Index;

use load::{InvalidBoardError, load};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Static(u8),
    User(u8),
    Error(u8),
}

pub struct Row([Cell; 9]);

impl Index<usize> for Row {
    type Output = Cell;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Row {
    pub fn col(&self, n_col: u8) -> Cell {
        self.0[(n_col - 1) as usize]
    }
}

impl IntoIterator for &Row {
    type Item = Cell;
    type IntoIter = std::array::IntoIter<Cell, 9>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Board {
    pub cells: [Cell; 81],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::Empty; 81],
        }
    }

    pub fn from_file(file_name: &str) -> Result<Board, InvalidBoardError> {
        let defined_values = load(file_name)?;
        let mut result = Board::new();

        for cell_value in defined_values {
            result.set_index(cell_value.index, Cell::Static(cell_value.value));
        }

        Ok(result)
    }

    pub fn rows(&self) -> [Row; 9] {
        [
            self.row(1),
            self.row(2),
            self.row(3),
            self.row(4),
            self.row(5),
            self.row(6),
            self.row(7),
            self.row(8),
            self.row(9),
        ]
    }

    pub fn row(&self, row: usize) -> Row {
        let start_index = (row - 1) * 9;
        let stop_index = row * 9;
        let mut result = [Cell::Empty; 9];
        result.copy_from_slice(&self.cells[start_index..stop_index]);
        Row(result)
    }

    pub fn set(&mut self, row: u8, col: u8, val: u8) {
        let index = index_of(row, col);
        self.set_index(index, Cell::User(val))
    }

    pub fn set_index(&mut self, index: usize, val: Cell) {
        self.cells[index as usize] = val;
    }
}

pub fn index_of(row: u8, col: u8) -> usize {
    ((row - 1) * 9 + col - 1) as usize
}
