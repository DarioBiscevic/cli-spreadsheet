mod cell;

use cell::*;

pub struct Spreadsheet{
    max_x: usize,
    max_y: usize,
    
    //First vector is "horizontal" --> holds the vectors for the columns
    cells: Vec< Vec<Cell> >,
}

impl Spreadsheet{

    pub fn new(n_cells: usize) -> Self{

        let mut cells = Vec::new();

        for _ in 0..n_cells{
            cells.push(Vec::new());
        }

        Spreadsheet{
            max_x: n_cells,
            max_y: 0,
            cells,
        }
    }


    pub fn view(&self, cell_arg: Option<&str>){
        if let Some(cell) = cell_arg{

            if let Some((x_coord, y_coord)) = Spreadsheet::parse_cell_name(cell){
                if x_coord >= self.max_x || y_coord >= self.max_y{
                    println!("{}: Empty", cell);
                    println!("0");
                }else{
                    println!("{}: {}", cell, self.cells[x_coord][y_coord].expression);
                    println!("{}", self.cells[x_coord][y_coord].value)
                }
            }

            //println!("{:?}", (x_coord, y_coord));
        }else{
            println!("No argument given");
        }
    }

    pub fn set(&mut self, cell_arg: Option<&str>, expression_arg: Option<&str>){

        if let Some(cell) = cell_arg{
            //println!("{:?}", cell);

            if let Some(expression) = expression_arg{
                //println!("{:?}", expression);
            

                if let Some((x_coord, y_coord)) = Spreadsheet::parse_cell_name(cell){

                    println!("{}, {}    {}, {}", x_coord, y_coord, self.max_x, self.max_y);


                    if x_coord < self.max_x && y_coord < self.max_y{
                        self.cells[x_coord][y_coord].set(expression);
                    }else{
                        //Calculate the number of additional columns and rows needed and add them
                        let add_cols = (x_coord as i64) - (self.max_x as i64) + 1;
                        let add_rows = (y_coord as i64) - (self.max_y as i64) + 1;

                        if add_cols > 0 { self.max_x += add_cols as usize; }
                        if add_rows > 0 { self.max_y += add_rows as usize; }

                        println!("{}, {}", add_cols, add_rows);

                        for _ in 0..add_cols{
                            self.cells.push(Vec::new());
                        }

                        println!("{}", self.cells.len());

                        for _ in 0..add_rows{
                            self.cells[x_coord].push(Cell::new((x_coord, y_coord)));
                        }

                        println!("{}", self.cells[x_coord].len());

                        self.cells[x_coord][y_coord].set(expression);

                        //println!("Cell: {}", self.cells[x_coord][y_coord].expression);
                    }
                }

            }else{
                println!("Second argument not given");
            }

        }else{
            println!("First argument not given");
        }
    }


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