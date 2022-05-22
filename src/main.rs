mod board;
mod cmd;
mod ui;

use board::Board;
use cmd::Cmd;
use ui::{ConsoleUi, Ui};

fn main() {
    let mut board = Board::from_file("data/med.board").unwrap();
    let mut renderer = ConsoleUi::new();

    renderer.draw(&board);

    loop {
        match renderer.get_input() {
            Cmd::Set { row, col, val } => board.set(row, col, val),
            Cmd::Exit => break,
        }

        renderer.draw(&board);
    }
}
