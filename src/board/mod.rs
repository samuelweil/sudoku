mod check;
mod load;

use std::fmt::Display;

use check::{check_for_duplicate, InvalidValueError};
use load::load;
use load::InvalidBoardError;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Static(u8),
    User(u8),
    Error(u8),
}

impl From<&Cell> for Option<u8> {
    fn from(cell: &Cell) -> Self {
        match *cell {
            Cell::Empty => None,
            Cell::Static(val) => Some(val),
            Cell::User(val) => Some(val),
            Cell::Error(val) => Some(val),
        }
    }
}

pub type Row = [Cell; 9];
pub type Column = [Cell; 9];
type Block = [Cell; 9];

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

    pub fn row(&self, row: u8) -> Row {
        let cast_row = row as usize;
        let start_index = (cast_row - 1) * 9;
        let stop_index = cast_row * 9;
        let mut result = [Cell::Empty; 9];
        result.copy_from_slice(&self.cells[start_index..stop_index]);
        result
    }

    pub fn column(&self, col: u8) -> Column {
        let mut result = [Cell::Empty; 9];
        for i in 0..9 {
            result[i] = self.cells[index_of(i as u8 + 1, col)];
        }
        result
    }

    pub fn remaining(&self) -> u8 {
        self.cells
            .into_iter()
            .filter(|cell| match cell {
                &Cell::Empty | &Cell::Error(_) => true,
                _ => false,
            })
            .count() as u8
    }

    fn block_for(&self, row: u8, col: u8) -> Block {
        let mut result = [Cell::Empty; 9];
        let start_row = ((row - 1) / 3) * 3 + 1;
        let start_col = ((col - 1) / 3) * 3 + 1;

        for (ii, r) in (start_row..start_row + 3).enumerate() {
            for (jj, c) in (start_col..start_col + 3).enumerate() {
                result[(ii + jj) as usize] = self.cells[index_of(r, c)]
            }
        }
        result
    }

    pub fn set(&mut self, row: u8, col: u8, val: u8) -> Result<(), InvalidValueError> {
        let index = index_of(row, col);

        if let Cell::Static(_) = self.cells[index] {
            return Err(InvalidValueError::StaticValue(index));
        }

        if let Some((col_idx, _)) = check_for_duplicate(self.row(row), val) {
            self.set_index(index, Cell::Error(val));
            dbg!("Setting {} {} to Error({})", row, col, val);
            let err = InvalidValueError::DuplicateValue {
                new: index,
                conflicting: index_of(row, col_idx + 1),
            };
            return Err(err);
        }

        if let Some((row_idx, _)) = check_for_duplicate(self.column(col), val) {
            self.set_index(index, Cell::Error(val));
            dbg!("Setting {} {} to Error({})", row, col, val);
            self.set_index(index, Cell::Error(val));
            let err = InvalidValueError::DuplicateValue {
                new: index,
                conflicting: index_of(row_idx as u8 + 1, col),
            };
            return Err(err);
        }

        if let Some((block_idx, _)) = check_for_duplicate(self.block_for(row, col), val) {
            self.set_index(index, Cell::Error(val));
            dbg!("Setting {} {} to Error({})", row, col, val);
            self.set_index(index, Cell::Error(val));
            let err = InvalidValueError::DuplicateValue {
                new: index,
                conflicting: index_of(row + block_idx / 3, col + block_idx % 3),
            };
            return Err(err);
        }

        dbg!("Setting {} {} to User({})", row, col, val);
        self.set_index(index, Cell::User(val));
        Ok(())
    }

    fn set_index(&mut self, index: usize, val: Cell) {
        self.cells[index as usize] = val;
    }
}

pub fn index_of(row: u8, col: u8) -> usize {
    ((row - 1) * 9 + col - 1) as usize
}
#[derive(Debug, PartialEq)]
pub struct Coord {
    pub row: u8,
    pub col: u8,
}

fn coord(index: usize) -> Coord {
    let findex = index as f32;
    let row = ((findex / 9f32).floor() as u8) + 1;
    let col = ((index % 9) as u8) + 1;
    Coord { row, col }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_to_coord() {
        let tests = [
            (0, Coord { row: 1, col: 1 }),
            (80, Coord { row: 9, col: 9 }),
            (42, Coord { row: 5, col: 7 }),
        ];

        for (input, exp) in tests {
            assert_eq!(coord(input), exp);
        }
    }

    fn test_board() -> Board {
        let mut result = Board::new();
        result.set(1, 1, 4).unwrap();
        result
    }

    #[test]
    fn ok_when_no_clash() {
        let mut board = test_board();
        let result = board.set(1, 4, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn error_when_row_has_value() {
        let mut board = test_board();
        let result = board.set(1, 4, 4);
        assert!(result.is_err());
    }

    #[test]
    fn error_when_col_has_value() {
        let mut board = test_board();
        let result = board.set(8, 1, 4);
        assert!(result.is_err());
    }

    #[test]
    fn error_when_block_has_value() {
        let mut board = test_board();
        let result = board.set(3, 3, 4);
        assert!(result.is_err());
    }

    #[test]
    fn error_when_overriding_game_value() {
        let mut board = Board::new();
        board.set_index(4, Cell::Static(4));
        let result = board.set(1, 5, 7);
        assert!(result.is_err());
    }
}
