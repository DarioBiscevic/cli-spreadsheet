//!For now, the functions defined below are identical, as the features are not yet implemented.
use crate::spreadsheet::*;

const CELLS: usize = 26;

///When called, loads the data of the specified file into a new spreadsheet object.
pub fn loader(filename: String) -> Spreadsheet{
    Spreadsheet::new(filename, CELLS)
}

///When called, creates a new, empty spreadsheet object.
pub fn creator(filename: String) -> Spreadsheet{
    Spreadsheet::new(filename, CELLS)
}
