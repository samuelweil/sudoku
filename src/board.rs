pub type Cell = Option<u8>;
pub type Row = [Cell; 9];

#[derive(Copy, Clone)]
pub struct Board {
    pub cells: [Cell; 81],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Option::None; 81],
        }
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
        result
    }
}