extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use unicode_segmentation::Graphemes;

use std::fs::File;
use std::io::Read;
use std::env;
use std::path::Path;
use std::string::String;
use std::fmt;

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


#[derive(Debug)]
enum LexElement {
    OpenParen,
    CloseParen,
    String(String),
    Token(String),
    Integer(i64),
    Float(f64),
}

enum LexState {
    Normal,
    Token(String),
    String(String),
    Number(String, bool),
}

fn lex(s: &str) -> Vec<LexElement> {
    let mut lex_stack = Vec::new();
    let mut state = LexState::Normal;
    for c in UnicodeSegmentation::graphemes(s,true) {
        match state {
            LexState::Normal => {
                match c {
                    "(" => {
                        lex_stack.push(LexElement::OpenParen);
                        state = LexState::Normal;
                    }
                    ")" => {
                        lex_stack.push(LexElement::CloseParen);
                    }
                    "\"" => {
                        state = LexState::String(String::new());
                    }
                    _ if is_number(c) => {
                        state = LexState::Number(String::from(c), false);
                    }
                    _ if is_whitespace(c) => {
                        state = LexState::Normal;
                    }
                    _ => {
                        state = LexState::Token(String::from(c));
                    }
                }
            }
            LexState::String(mut string) => {
                match c {
                    "\"" => {
                        lex_stack.push(LexElement::String(string));
                        state = LexState::Normal;
                    }
                    _ => {
                        string.push_str(c);
                        state = LexState::String(string);
                    }
                }
            }
            LexState::Number(mut string, float) => {
                match c {
                    _ if is_number(c) => {
                        string.push_str(c);
                        state = LexState::Number(string, float);
                    }
                    "." if !float => {
                        string.push_str(c);
                        state = LexState::Number(string, true);
                    }
                    c => {
                        lex_stack.push(match float {
                            true => {
                                let val = string.parse().unwrap();
                                LexElement::Float(val)
                            }
                            false => {
                                let val = string.parse().unwrap();
                                LexElement::Integer(val)
                            }
                        });
                        match c {
                            ")" => {
                                // lex_stack.
                                lex_stack.push(LexElement::CloseParen);
                                state = LexState::Normal;
                            }
                            _ if is_whitespace(c) => {
                                state = LexState::Normal;
                            }
                            _ => {
                                panic!("Parse Error, invalid token following number");
                            }
                        }
                    }
                }
            }
            LexState::Token(mut string) => {
                match c {
                    ")" => {
                        lex_stack.push(LexElement::Token(string));
                        lex_stack.push(LexElement::CloseParen);
                        state = LexState::Normal;
                    }
                    "\"" => {
                        panic!("\" Not allowed in tokens");
                    }
                    _ if is_whitespace(c) => {
                        lex_stack.push(LexElement::Token(string));
                        state = LexState::Normal;
                    }
                    _ => {
                        string.push_str(c);
                        state = LexState::Token(string);
                    }
                }
            }
        }
    }
    return lex_stack;
}

fn is_number(c: &str) -> bool {
    match c {
        "0" => true,
        "1" => true,
        "2" => true,
        "3" => true,
        "4" => true,
        "5" => true,
        "6" => true,
        "7" => true,
        "8" => true,
        "9" => true,
        _ => false
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
