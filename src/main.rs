use std::env;

mod lib;
mod startup;

fn main() {
    println!("cli-spreadsheet");

    match parse_args(){
        Settings::Load   => ,
        Settings::Create => ,
        _                => println!("Argument error"),
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
            Ok(Settings::Load(args[2].clone())
        }else if args[1] == "create"{
            Ok(Settings::Create(args[2].clone())
        }else{
            Err(())
        }
    }
}