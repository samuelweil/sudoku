mod board;
mod ui;

use core::fmt;
use std::{error, io};

use board::Board;
use ui::{ConsoleUi, Ui};

fn main() -> io::Result<()> {
    let mut board = Board::from_file("data/med.board").unwrap();
    let mut renderer = ConsoleUi::new();

    loop {
        renderer.draw(&board);

        let mut input = String::new();
        println!("{}", "Enter next command");
        io::stdin().read_line(&mut input)?;

        clear_screen();

        match parse_tokens(input) {
            Ok((row, col, val)) => board.set(row, col, val),
            Err(e) => println!("{}", e),
        }
    }
}

fn parse_tokens(s: String) -> ParseResult<(u8, u8, u8)> {
    let tokens = s.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
    if tokens.len() != 3 {
        return InputError::new("3 input arguments required: row, col, val");
    }

    let row = parse_arg(tokens[0])?;
    let col = parse_arg(tokens[1])?;
    let val = parse_arg(tokens[2])?;

    Ok((row, col, val))
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Clone, Debug)]
struct InputError {
    msg: String,
}

impl InputError {
    fn new<T, M>(msg: M) -> ParseResult<T>
    where
        String: From<M>,
    {
        Err(InputError {
            msg: String::from(msg),
        })
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid inputs {}", self.msg)
    }
}

impl error::Error for InputError {}

type ParseResult<T> = Result<T, InputError>;

fn parse_error(val: &str) -> ParseResult<u8> {
    InputError::new(format!("{} must be a number between 1-9", val))
}

fn parse_arg(input: &str) -> ParseResult<u8> {
    match str::parse::<u8>(input) {
        Err(_) => parse_error(input),
        Ok(u) => {
            if u > 0 && u < 10 {
                Ok(u)
            } else {
                parse_error(input)
            }
        }
    }
}
