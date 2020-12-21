use crate::spreadsheet::*;
use crate::cell::*;

use std::io::{stdin,stdout,Write};

pub fn run(sheet: &mut Spreadsheet){

    let mut is_looping = true;

    while is_looping{

        let input_result = command_input();

        if let Ok(command) = input_result{

            //TODO: ad hoc function to split the input in different parts
            let mut arguments = command.split(' '); //Temporary
            
            //"arguments" is used as a "stack"; with ".next()" a value is popped as an Option<T>
            match arguments.next(){
                Some("")            => {},
                Some("exit")        => is_looping = false,
                Some("view")        => sheet.view(arguments.next()),
                Some("set")         => sheet.set(arguments.next(), arguments.next()),
                Some(wrong_command) => println!("Command {:?} not recognised", wrong_command),
                _                   => {},
            }


            println!();

        }else if let Err(error) = input_result{

            //It must be a big input error, since it can only be invoked by 'readline'
            println!("Big input error");
            println!("Error: {:?}", error);
            let _ = stdout().flush();

        }
    }

    //Save question
    //TODO: add "cancel" option
    println!("Save? [y(es) / n(o)]");
    let _ = stdout().flush();

    loop{
        let input = command_input();

        if let Ok(c) = input{

            let letters: Vec<_> = c.chars().collect();

            if letters[0].to_ascii_lowercase() == 'y'{

                save(sheet);

                break
            }else if letters[0].to_ascii_lowercase() == 'n'{break}
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

fn save(sheet: &Spreadsheet){

    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new(sheet.filename.as_str());
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    //Combine into a variable all the data that must be written to the save file;
    //Add all the "defined" cells --> all the cells which have "cell.name" as a Some
    let contents = sheet.cells
        .iter()
        .flatten()
        .filter(|cell| cell.name.is_some())
        .fold(String::new(), |acc, elem|{
            format!("{}{}: {}\n", acc, elem.name.clone().unwrap(), elem.expression)
        });

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Failed to save in {}: {}", display, why),
        Ok(_) => println!("Successfully saved in {}", display),
    }
}