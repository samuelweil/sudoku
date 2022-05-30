use std::{error, fmt, fs, io};

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

#[derive(PartialEq, Debug)]
pub struct BoardValue {
    pub index: usize,
    pub value: u8,
}

pub fn load(filename: &str) -> Result<Vec<BoardValue>, InvalidBoardError> {
    let mut contents = fs::read_to_string(filename)
        .map_err(|e| InvalidBoardError::for_file(filename).io_error(e))?;
    if !contents.is_ascii() {
        return Err(InvalidBoardError::for_file(filename).not_ascii());
    }
    contents.retain(is_valid_char);

    if contents.len() != 81 {
        return Err(InvalidBoardError::for_file(filename).invalid_size(contents.len()));
    }

    let mut result = Vec::new();

    for (index, c) in contents.chars().enumerate() {
        if c != EMPTY_CHAR {
            // These unwraps are safe since we've already checked that the characters are 1-9
            let value: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            result.push(BoardValue { index, value });
        }
    }

    Ok(result)
}

fn is_valid_char(c: char) -> bool {
    [EMPTY_CHAR, '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)
}

#[cfg(test)]
mod tests {
    use crate::board::index_of;

    use super::*;

    fn to_board_value(inp: (u8, u8, u8)) -> BoardValue {
        let (row, col, value) = inp;
        BoardValue {
            index: index_of(row, col),
            value,
        }
    }

    #[test]
    fn test_load_file() {
        let expected: Vec<BoardValue> = vec![
            (1, 2, 3),
            (1, 6, 8),
            (1, 7, 1),
            (1, 8, 6),
            (2, 3, 5),
            (2, 4, 1),
            (2, 6, 2),
            (2, 9, 9),
            (3, 4, 7),
            (3, 6, 4),
            (4, 5, 2),
            (4, 8, 1),
            (4, 9, 3),
            (5, 3, 2),
            (5, 4, 9),
            (5, 6, 5),
            (5, 7, 6),
            (5, 8, 8),
            (5, 9, 7),
            (6, 3, 7),
            (6, 6, 3),
            (7, 2, 2),
            (7, 3, 6),
            (7, 4, 8),
            (7, 5, 7),
            (7, 8, 4),
            (8, 2, 5),
            (8, 6, 9),
            (9, 1, 4),
            (9, 6, 6),
        ]
        .into_iter()
        .map(to_board_value)
        .collect();

        load("data/med.board")
            .unwrap()
            .into_iter()
            .enumerate()
            .for_each(|(i, inp)| assert_eq!(inp, expected[i]));
    }
}
