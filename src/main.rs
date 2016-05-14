use std::fs::File;
use std::io::Read;
use std::env;
use std::path::Path;
use std::string::String;

mod lex;

use lex::lex;

fn main() {
    println!("Reading File");
    let file_name = match env::args_os().nth(1) {
        Some(a) => {a}
        None => panic!("No file passed")
    };
    let file_name = Path::new(&file_name);
    let mut program_file = match File::open(file_name) {
        Err(e) => panic!("Couldn't open file: {}", e),
        Ok(f) => f
    };
    let mut file_string = String::new();
    match program_file.read_to_string(&mut file_string) {
        Err(e) => panic!("Error reading file: {}", e),
        Ok(_) => {}
    }
    print!("File: \n{}", file_string);
    print!("Stack: {:?}\n", lex(&file_string));
}
