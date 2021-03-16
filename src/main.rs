use std::env;

mod lib;
mod startup;
mod spreadsheet;
mod cell;

fn main() {
    println!("cli-spreadsheet");

    if let Ok(mode) = parse_args(){

        let mut spreadsheet: spreadsheet::Spreadsheet;

        match mode{
            Settings::Load(filename) => {
                spreadsheet = startup::loader(filename);
            },
            Settings::Create(filename) => {
                spreadsheet = startup::creator(filename);
            },
        }


        lib::run(&mut spreadsheet);
    }else{
        println!("Argument error");
    }
}

enum Settings{
    Load(String),   //Load <filename>
    Create(String), //Create <filename>
}


//TODO: improve parser
fn parse_args() -> Result<Settings, ()>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        Err(())
    }else{

        if args[1] == "load"{
            return Ok(Settings::Load(args[2].clone()))
        }else if args[1] == "create"{
            return Ok(Settings::Create(args[2].clone()))
        }

        Err(())
    }
}
