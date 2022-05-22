mod board;
mod ui;

use std::io;

use board::Board;
use ui::{ConsoleUi, Ui};

fn set_board(board: &mut Board, args: &Vec<String>) -> io::Result<()> {
    let (row, col, val) = parse_args(args)?;
    board.set(row, col, val);
    Ok(())
}

fn main() {
    let mut board = Board::from_file("data/med.board").unwrap();
    let mut renderer = ConsoleUi::new();
    
    renderer.draw(&board);

    loop {
        let cmd = renderer.get_input().unwrap();

        let result: io::Result<()> = match &cmd.name[..] {
            "set" => set_board(&mut board, &cmd.args),
            _ => {
                println!("Unrecognized command {}", cmd.name);
                Ok(())
            }
        };

        renderer.draw(&board);

        if let Err(e) = result {
            renderer.display_err(e);
        }
    }
}

fn parse_args(args: &Vec<String>) -> io::Result<(u8, u8, u8)> {
    if args.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Set requires 3 args <row> <col> <val>",
        ));
    }
    Ok((
        parse_arg(&args[0])?,
        parse_arg(&args[1])?,
        parse_arg(&args[2])?,
    ))
}

fn parse_arg(arg: &str) -> io::Result<u8> {
    fn invalid_arg(arg: &str) -> io::Error {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Argument must be integer 1 - 9, got {}", arg),
        )
    }
    arg.parse::<u8>().map_err(|_| invalid_arg(arg))
}
