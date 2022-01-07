mod board;
mod renderer;

use board::Board;
use renderer::{Renderer, ConsoleRenderer};

fn main() {
    let board = Board::new();
    let mut renderer = ConsoleRenderer::new();

    renderer.render(&board);
}
