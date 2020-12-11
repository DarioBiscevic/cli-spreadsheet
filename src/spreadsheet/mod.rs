mod cell;

use cell::*;

pub struct Spreadsheet{
    cells: Vec< Vec<Cell> >,
}

impl Spreadsheet{

    pub fn new(n_cells: usize) -> Self{

        let mut cells = Vec::new();

        for _ in 0..n_cells{
            cells.push(Vec::new());
        }

        Spreadsheet{
            cells,
        }
    }


    pub fn view(&self, cell_arg: Option<&str>){
        if let Some(cell) = cell_arg{

            use std::convert::TryInto;

            //x_coord is a letter
            //y_coord is a digit
            let (x_coord, y_coord): (String, String) = cell.chars().partition(|c| !c.is_digit(10));

            println!("{:?}", (x_coord.clone(), y_coord.clone()));

            /*
            NOTE: this snippet converts all the letters in a number --> a = column 0, b = column 1, .. , aa = column 26
            */
            //TODO: improve this with error handling
            let x_coord: usize = x_coord
                .chars()
                .rev()
                .enumerate()
                .fold(0, |acc, (index, elem)| {
                    //'index' is improbable to be larger than 2^30 - 1 (max i32)
                    acc + (elem.to_uppercase().last().unwrap() as usize - 64) * (26 as usize).pow(index.try_into().unwrap())
                }) - 1;

            //TODO: improve this with error handling
            let y_coord: usize = y_coord.parse().unwrap();

            //println!("{:?}", (x_coord, y_coord));
        }else{
            println!("No argument given");
        }
    }

    pub fn set(&self, cell_arg: Option<&str>, expression_arg: Option<&str>){

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