use std::{io::{self, Write}, error::Error};

use crate::board::{Board, Cell, Row};

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    fn from_vec(mut vec: Vec<String>) -> Command {
        let args = if vec.len() > 1 {
            vec.drain(1..).collect()
        } else {
            Vec::new()
        };

        Command {
            name: vec.remove(0),
            args,
        }
    }
}

pub trait Ui {
    fn draw(&mut self, board: &Board);
    fn get_input(&mut self) -> io::Result<Command>;
    fn display_err<E: Error>(&mut self, text: E);
}

pub struct ConsoleUi {
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

impl ConsoleUi {
    pub fn new() -> ConsoleUi {
        ConsoleUi {
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
            self.buffers[n_buffer][index] = draw_cell(&cell);
        }
    }

    fn draw_buffer(&self) {
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

impl Ui for ConsoleUi {
    fn draw(&mut self, board: &Board) {
        clear_screen();
        self.update_buffers(board);
        self.draw_buffer();
    }

    fn get_input(&mut self) -> io::Result<Command> {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;

            dbg!(&buffer);

            let tokens: Vec<String> = buffer
                .split_ascii_whitespace()
                .map(|s| String::from(s.trim()))
                .collect();

            if tokens.len() > 1 {
                return Ok(Command::from_vec(tokens));
            }
        }
    }

    fn display_err<E: Error>(&mut self, e: E) {
        eprintln!("Error: {}", e)    
    }
}

fn draw_cell(cell: &Cell) -> char {
    match cell {
        None => '-',
        Some(u) => (*u + 48) as char,
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
