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

            use std::convert::TryInto;

            //x_coord is a letter
            //y_coord is a digit

            //TODO: improve
            let (x_string, y_string): (String, String) = cell.chars().partition(|c| !c.is_digit(10));

            if x_string.is_empty() || y_string.is_empty(){
                println!("Incomplete cell name");
                return
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

            if x_coord >= self.max_x || y_coord >= self.max_y{
                println!("{}{}: Empty", x_string, y_string);
                println!("0");
            }

            //println!("{:?}", (x_coord, y_coord));
        }else{
            println!("No argument given");
        }
    }

    pub fn set(&mut self, cell_arg: Option<&str>, expression_arg: Option<&str>){

        if let Some(cell) = cell_arg{
            println!("{:?}", cell);

            if let Some(expression) = expression_arg{
                println!("{:?}", expression);
            }else{
                println!("Second argument not given");
            }
        }else{
            println!("First argument not given");
        }
    }

}
