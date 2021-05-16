use clap::*;

mod lib;
mod startup;
mod spreadsheet;
mod cell;

fn main() {
    println!("cli-spreadsheet");

    /*
    if let Ok(settings) = parse_args(){

        let mut spreadsheet: spreadsheet::Spreadsheet;

        match settings{
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
*/
    let result = parse_args();

    if let Ok(settings) = result{

        let mut spreadsheet: spreadsheet::Spreadsheet;

        match settings{
            Settings::Load(filename) => {
                spreadsheet = startup::loader(filename);
            },
            Settings::Create(filename) => {
                spreadsheet = startup::creator(filename);
            },
        }

        lib::run(&mut spreadsheet);

    }else if let Err(error) = result{
        println!("Argument error: {}", error);
    }


}

enum Settings{
    Load(String),   //Load <filename>
    Create(String), //Create <filename>
}


//TODO: improve parser
fn parse_args() -> Result<Settings>{


    let matches_raw = App::new("cli-spreadsheet")
                          .version("1.0")
                          .author("Dario Biscevic <dario.biscevic03@gmail.com>")
                          .about("Spreadsheet program for terminal")
                          .arg(Arg::with_name("mode")
                               .short("m")
                               .long("mode")
                               .value_name("MODE")
                               .help("Define the output initial mode: create new file or load one")
                               .takes_value(true))
                          .arg(Arg::with_name("filename")
                               .short("f")
                               .long("file")
                               .value_name("FILENAME")
                               .takes_value(true))
                          .get_matches_safe();

    if let Ok(matches) = matches_raw{

        let mode = matches.value_of("mode").unwrap_or("create");

        let filename = matches.value_of("filename").unwrap_or("sheet");


        match mode{
            "load"   => Ok(Settings::Load(filename.to_string())),
            "create" => Ok(Settings::Create(filename.to_string())),
            &_       => Err(clap::Error{
                message: "Unknown flag".to_string(),
                info: None,
                kind: clap::ErrorKind::UnknownArgument,
            })
        }

    }else{

        Err(clap::Error{
            message: "Unknown flag".to_string(),
            info: None,
            kind: clap::ErrorKind::UnknownArgument,
        })

    }

}
