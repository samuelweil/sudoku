use std::{error, fmt, fs, io};

use crate::board::Board;

const EMPTY_CHAR: char = '-';

#[derive(Debug)]
pub struct InvalidBoardError {
    msg: String,
    filename: String,
}

struct InvalidBoardErrorBuilder {
    filename: String,
}

impl error::Error for InvalidBoardError {}

impl fmt::Display for InvalidBoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid board {}: {}", self.filename, self.msg)
    }
}

impl InvalidBoardError {
    fn for_file(fname: &str) -> InvalidBoardErrorBuilder {
        InvalidBoardErrorBuilder {
            filename: fname.to_owned(),
        }
    }
}

impl InvalidBoardErrorBuilder {
    fn io_error(self, err: io::Error) -> InvalidBoardError {
        InvalidBoardError {
            filename: self.filename,
            msg: format!("{}", err),
        }
    }

    fn not_ascii(self) -> InvalidBoardError {
        InvalidBoardError {
            filename: self.filename,
            msg: String::from("Board file must be valid ascii"),
        }
    }

    fn invalid_size(self, size: usize) -> InvalidBoardError {
        InvalidBoardError {
            filename: self.filename,
            msg: format!("Board file must have 81 valid characters, got {}", size),
        }
    }
}

pub fn load(filename: &str) -> Result<Board, InvalidBoardError> {
    let mut contents = fs::read_to_string(filename)
        .map_err(|e| InvalidBoardError::for_file(filename).io_error(e))?;
    if !contents.is_ascii() {
        return Err(InvalidBoardError::for_file(filename).not_ascii());
    }
    contents.retain(is_valid_char);

    if contents.len() != 81 {
        return Err(InvalidBoardError::for_file(filename).invalid_size(contents.len()));
    }

    let mut result = Board::new();

    for (i, c) in contents.chars().enumerate() {
        if c != EMPTY_CHAR {
            // These unwraps are safe since we've already checked that the characters are 1-9
            let value: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            result.set_index(i.try_into().unwrap(), value);
        }
    }

    Ok(result)
}

fn is_valid_char(c: char) -> bool {
    [EMPTY_CHAR, '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)
}

#[cfg(test)]
mod tests {
    use crate::board::Cell;

    use super::*;

    #[test]
    fn test_load_file() {
        let board = Board::from_file("data/med.board").unwrap(); 
        assert_eq!(board.row(1).col(2), Cell::User(3u8));
    }
}
