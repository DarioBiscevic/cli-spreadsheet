mod cell;

use cell::*;

pub struct Spreadsheet{
    cells: Vec<Cell>,
}

impl Spreadsheet{

    pub fn new(n_cells: usize) -> Self{

        let mut cells = Vec::new();

        for _ in 0..n_cells{
            cells.push(Cell::new());
        }

        Spreadsheet{
            cells,
        }
    }

}