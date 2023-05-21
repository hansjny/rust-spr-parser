use std::{env, io};
use std::fs::File;
use std::process;

fn main() -> io::Result<()> {

    let filename: String = match parse_command_line() {
        Ok(filename) => filename,
        Err(e) => {
            println!("Error parsing command line: {}", e);
            process::exit(1);
        },
    };

    let file: File = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            process::exit(1);
        },
    };
    print!("File opened successfully");
    Ok(())
}


fn parse_command_line() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { 
        Err("Please provide spr file path via  /path/to/file.spr")
    } else
    {
        Ok(args[1].clone())
    }
}
