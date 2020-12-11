use crate::spreadsheet::*;

use std::io::{stdin,stdout,Write};

pub fn run(sheet: Spreadsheet){

    let mut is_looping = true;

    while is_looping{

        let input_result = command_input();

        if let Ok(command) = input_result{

            //TODO: ad hoc function to split the input in different parts

            let mut arguments = command.split(' '); //Temporary
            
            match arguments.next(){
                Some("")            => {},
                Some("exit")        => is_looping = false,
                Some("view")        => sheet.view(arguments.next()),
                Some("set")         => sheet.set(arguments.next(), arguments.next()),
                Some(wrong_command) => println!("Command {:?} not recognised", wrong_command),
                _                   => {},
            }


            println!("");

        }else if let Err(error) = input_result{

            //It must be a big input error, since it can only be invoked by 'readline'
            println!("Big input error");
            println!("Error: {:?}", error);
            let _ = stdout().flush();

        }
    }
}

fn command_input() -> Result<String, Box<dyn std::error::Error>>{

    print!("> ");

    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input)?;

    //Get "clean" input
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    Ok(input)
}