use std::env;
fn main() {
    let mut filename = String::new();
    if parse_command_line(&mut filename) {
        println!("Parsing file: {}", filename);
    } else {
        return;
    }
}


fn parse_command_line(filename: &mut String) -> bool {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { 
        println!("Please provide spr file path via  /path/to/file.spr");
        return false;
    }

    *filename = args[1].clone();
    true
}
