mod load;

use load::load;

use self::load::InvalidBoardError;

pub type Cell = Option<u8>;
//pub type Row = [Cell; 9];

pub struct Row([Cell; 9]);

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
            cells: [Option::None; 81],
        }
    }

    pub fn from_file(file_name: &str) -> Result<Board, InvalidBoardError> {
        load(file_name)
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
        let mut result = [Option::None; 9];
        result.copy_from_slice(&self.cells[start_index..stop_index]);
        Row(result)
    }

    pub fn set(&mut self, row: u8, col: u8, val: u8) {
        let index = (row - 1) * 9 + col - 1;
        self.set_index(index, val)
    }

    pub fn set_index(&mut self, index: u8, val: u8) {
        self.cells[index as usize] = Some(val);
    }
}
