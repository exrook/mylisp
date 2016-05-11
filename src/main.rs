extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

use std::fs::File;
use std::io::Read;
use std::env;
use std::path::Path;
use std::string::String;

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
    evaluate(&file_string);

}

enum ParseState {
    WhitespaceOrParen,
    FunctionName(String),
    FunctionArg(String, String),
    WhitespaceOrArgOrParen(String),
}

enum ParseElement {
    Function(String),
    Argument(String)
}

fn evaluate(s: &str) {
    let stack = Vec::new();
    let mut state = ParseState::WhitespaceOrParen;
    for c in UnicodeSegmentation::graphemes(s, true) {
        match state {
            ParseState::WhitespaceOrParen => {
                match c {
                    "(" => { //start of function
                        state = ParseState::FunctionName(String::new())
                    }
                    _ if is_whitespace(c) => { //

                    }
                    _ => {
                        panic!("Parse Error on character [{}]", c)
                    }
                }
            }
            ParseState::FunctionName(mut name) => {
                match c {
                    ")" => {
                        state = ParseState::WhitespaceOrParen;
                    }
                    _ if is_whitespace(c) => {
                        state = ParseState::WhitespaceOrArgOrParen(name);
                    }
                    c => {
                        name.push_str(c);
                        state = ParseState::FunctionName(name);
                    }
                }
            }
            ParseState::FunctionArg(name, mut arg) => {
                match c {
                    ")" => { // func is over, next func
                        state = ParseState::WhitespaceOrParen;
                    }
                    _ if is_whitespace(c) => { //arg is over, next arg
                        state = ParseState::WhitespaceOrArgOrParen(name)
                    }
                    _ => { // next glyph
                        arg.push_str(c);
                        state = ParseState::FunctionArg(name, arg);
                    }
                }
            }
            ParseState::WhitespaceOrArgOrParen(name) => {
                match c {
                    ")" => {
                        state = ParseState::WhitespaceOrParen
                    }
                    _ if is_whitespace(c) => {

                    }
                    _ => {
                        state = ParseState::FunctionArg(name, String::new());
                    }
                }
            }
        }
    }
}

fn is_whitespace(c: &str) -> bool {
    match c {
        " " => true,
        "\t" => true,
        "\n" => true,
        "\r\n" => true,
        _ => false
    }
}
