use super::{coord, Cell};
use std::fmt;

#[derive(Debug)]
pub struct InvalidValueError {
    pub cell_idx_1: usize,
    pub cell_idx_2: usize,
}

impl fmt::Display for InvalidValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} and {} cannot be the same value",
            coord(self.cell_idx_1),
            coord(self.cell_idx_2)
        )
    }
}

pub fn check_for_duplicate(cell_group: [Cell; 9], value: u8) -> Option<(u8, Cell)> {
    cell_group
        .into_iter()
        .enumerate()
        .find(|(_, cell)| {
            if let Some(cell_val) = Option::<u8>::from(cell) {
                cell_val == value
            } else {
                false
            }
        })
        // Since we've fixed the input size to 9 we know this cast is always safe since i < 9
        .map(|(i, c)| (i as u8, c))
}
