use crate::spreadsheet::*;

const CELLS: usize = 26;

pub fn loader(filename: String) -> Spreadsheet{
    Spreadsheet::new(filename, CELLS)
}

pub fn creator(filename: String) -> Spreadsheet{
    Spreadsheet::new(filename, CELLS)
}