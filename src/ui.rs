use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
};

use crate::{
    board::{Board, Cell, Row},
    cmd::Cmd,
};
pub trait Ui {
    fn draw(&mut self, board: &Board);
    fn get_input(&mut self) -> Cmd;
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

    fn get_input(&mut self) -> Cmd {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            dbg!(&buffer);

            match parse_cmd(&buffer) {
                Ok(cmd) => return cmd,
                Err(e) => {
                    eprintln!("{}", e)
                }
            };
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

fn non_empty(inp: &str) -> Option<String> {
    let result = inp.trim();
    if result.is_empty() {
        return None;
    }

    Some(String::from(result))
}

fn parse_cmd(input: &String) -> Result<Cmd, InputError> {
    let tokens: Vec<String> = input
        .split_ascii_whitespace()
        .filter_map(non_empty)
        .collect();

    if tokens.len() == 0 {
        return Err(InputError::NoInput);
    }

    match &tokens[0][..] {
        "set" | "s" => {
            if tokens.len() < 4 {
                return Err(InputError::InsufficientArgs("<row> <col> <val>"));
            }

            return Ok(Cmd::Set {
                row: tokens[1].parse::<u8>().unwrap(),
                col: tokens[2].parse::<u8>().unwrap(),
                val: tokens[3].parse::<u8>().unwrap(),
            });
        }
        "exit" | "e" | "quit" | "q" => Ok(Cmd::Exit),
        _ => Err(InputError::NoInput),
    }
}

#[derive(Debug)]
enum InputError {
    InsufficientArgs(&'static str),
    NoInput,
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InsufficientArgs(exp) => write!(f, "Insufficient args, expected {}", exp),
            Self::NoInput => write!(f, "Expected <cmd> <...args>"),
        }
    }
}

impl Error for InputError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::Cmd;

    #[test]
    fn test_parsing_valid_set() {
        fn test(input: &'static str, exp_row: u8, exp_col: u8, exp_val: u8) {
            if let Ok(Cmd::Set { row, col, val }) = parse_cmd(&String::from(input)) {
                assert_eq!(row, exp_row);
                assert_eq!(col, exp_col);
                assert_eq!(val, exp_val);
            } else {
                panic!("Parsing {} should return valid Set command", input);
            }
        }

        test("set 1 2 3", 1, 2, 3);
        test("set 4 5 6", 4, 5, 6);
        test("s 1 2 3", 1, 2, 3);
        test("s 4 5 6", 4, 5, 6);
    }

    #[test]
    fn test_parse_set_not_enough_args() {
        fn test(input: &'static str) {
            match parse_cmd(&String::from(input)) {
                Err(InputError::InsufficientArgs(_)) => { /* Ok */ }
                _ => panic!("Parsing {} should return InsufficientArgs error", input),
            }
        }

        test("set");
        test("set 1");
        test("set 1 2");
    }

    #[test]
    fn test_parse_empty_cmd_returns_error() {
        fn test(input: &'static str) {
            match parse_cmd(&String::from(input)) {
                Err(InputError::NoInput) => { /* Ok */ }
                _ => panic!("An empty command should return NoInput error"),
            }
        }

        test("");
        test("  ");
    }

    #[test]
    fn test_parsing_exit() {
        fn test(input: &'static str) {
            if let Ok(Cmd::Exit) = parse_cmd(&String::from(input)) {
                // Ok
            } else {
                panic!("Parsing {} should return exit command", input);
            }
        }

        test("exit");
        test("e");
        test("q");
        test("quit");
    }
}
