use crate::cell::*;

///Spreadsheet objects hold the two-dimensional array with all the cells.
pub struct Spreadsheet{

    pub filename: String,

    //Number of columns allocated
    max_x: usize,
    
    //First vector is "horizontal" --> holds the vectors for the columns
    pub cells: Vec< Vec<Cell> >,
}

impl Spreadsheet{

    ///Creates a new spreadsheet.
    pub fn new(filename: String, n_cells: usize) -> Self{

        let mut cells = Vec::new();

        for _ in 0..n_cells{
            cells.push(Vec::new());
        }

        Spreadsheet{
            filename,
            max_x: n_cells,
            cells,
        }
    }


    ///Prints a cell's expression and value:
    ///
    ///view [cell]
    ///e.g. view B4
    ///
    ///Will print:
    ///
    ///B4: <B4's expression>
    ///
    ///    <B4's value>

    pub fn view(&self, cell_arg: Option<&str>){
        if let Some(cell) = cell_arg{

            if let Some((x_coord, y_coord)) = Spreadsheet::parse_cell_name(cell){
                if x_coord >= self.max_x{
                    //If the x_coord-th column is not defined, the cell that must be viewed is definitely empty
                    println!("{}: Empty", cell);
                    println!("0");
                }else{
                    //If the y_coord-th row is not defined, the cell that must be viewed is definitely empty
                    if y_coord >= self.cells[x_coord].len(){
                        println!("{}: Empty", cell);
                        println!("0");
                    }else{
                        println!("{}: {}", cell, self.cells[x_coord][y_coord].expression);
                        println!("{}", self.cells[x_coord][y_coord].value);
                    }
                }
            }
        }else{
            println!("No argument given");
        }
    }


    ///Modifies a cell's expression:
    ///
    ///set [cell] [expression]
    ///e.g. set A2 "42"
    ///
    ///A2's expression becomes "42"
    ///
    pub fn set(&mut self, cell_arg: Option<&str>, expression_arg: Option<&str>){

        if let Some(cell) = cell_arg{

            if let Some(expression) = expression_arg{

                if let Some((x_coord, y_coord)) = Spreadsheet::parse_cell_name(cell){

                    //Calculate the number of additional columns and rows needed and add them
                    let add_cols = (x_coord as i64) - (self.max_x as i64) + 1;

                    if add_cols > 0 { self.max_x += add_cols as usize; }

                    for _ in 0..add_cols{
                        self.cells.push(Vec::new());
                    }

                    let add_rows = (y_coord as i64) - (self.cells[x_coord].len() as i64) + 1;

                    for _ in 0..add_rows{
                        self.cells[x_coord].push(Cell::new(None, (x_coord, y_coord)));
                    }

                    self.cells[x_coord][y_coord].set_expr(expression);
                    self.cells[x_coord][y_coord].set_name(cell);
                }

            }else{
                println!("Second argument not given");
            }

        }else{
            println!("First argument not given");
        }
    }


    ///Converts the string cell name into coordinates to be used in the spreadsheet array.
    fn parse_cell_name(cell: &str) -> Option<(usize, usize)>{

        use std::convert::TryInto;

        //TODO: improve
        let (x_string, y_string): (String, String) = cell.chars().partition(|c| !c.is_digit(10));

        if x_string.is_empty() || y_string.is_empty(){
            println!("Incomplete cell name");
            return None
        }

        /*
        NOTE: this snippet converts all the letters in a number --> a = column 0, b = column 1, .. , aa = column 26
        */
        //TODO: improve this with error handling
        let x_coord: usize = x_string
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, elem)| {
                //'index' is improbable to be larger than 2^31 - 1 (max i32)
                acc + (elem.to_uppercase().last().unwrap() as usize - 64) * (26 as usize).pow(index.try_into().unwrap())
            }) - 1;

        //TODO: improve this with error handling
        let y_coord: usize = y_string.parse().unwrap();

        Some((x_coord, y_coord))
    }
}
