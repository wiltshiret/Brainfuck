use std::{env, fs, io};
use lexer::{Tokens, tokenize};
use parser::parse;

mod lexer;
mod parser;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Unknown file path");
    }

    let file_path: &String = &args[1];
    let code: String = fs::read_to_string(file_path).expect("Failed to reading file");

    let tokens: Vec<Tokens> = tokenize(&code);
    let result: io::Result<()> = parse(&tokens);

    println!(""); // for move to next line
    result
}