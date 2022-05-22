use crate::board::{Board, Cell, Row};

pub trait Renderer {
    fn render(&mut self, board: &Board);
}

pub struct ConsoleRenderer {
    buffers: [Vec<char>; 9],
}

fn mapping(index: usize) -> usize {
    let (group, offset) = if index > 5 {
        (2, 16)
    } else if index > 2 {
        (1, 8)
    } else {
        (0, 0)
    };

    2 * (index - (group * 3)) + offset + 2
}

const HEADER: &str = "   1 2 3   4 5 6   7 8 9";

static DIVIDER: &str = " |-----------------------|";

fn new_buffer() -> Vec<char> {
    String::from("|       |       |       |").chars().collect()
}

impl ConsoleRenderer {
    pub fn new() -> ConsoleRenderer {
        ConsoleRenderer {
            buffers: ['a'; 9].map(|_| new_buffer()),
        }
    }

    fn update_buffers(&mut self, board: &Board) {
        for (n_row, row) in board.rows().into_iter().enumerate() {
            self.update_buffer(n_row, &row);
        }
    }

    fn update_buffer(&mut self, n_buffer: usize, row: &Row) {
        for (n_cell, cell) in row.into_iter().enumerate() {
            let index = mapping(n_cell);
            self.buffers[n_buffer][index] = draw_cell(cell);
        }
    }

    fn draw(&self) {
        println!("{}", HEADER);
        for (n_buf, buf) in (&self.buffers).into_iter().enumerate() {
            println!("{}{}", n_buf + 1, buf.into_iter().collect::<String>());
            match n_buf {
                2 | 5 => println!("{}", DIVIDER),
                _ => {}
            };
        }
    }
}

impl Renderer for ConsoleRenderer {
    fn render(&mut self, board: &Board) {
        self.update_buffers(board);
        self.draw();
    }
}

fn draw_cell(cell: &Cell) -> char {
    match cell {
        None => '-',
        Some(u) => (*u + 48) as char,
    }
}
