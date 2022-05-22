#[derive(Debug)]
pub enum Cmd {
    Set { row: u8, col: u8, val: u8 },
}
