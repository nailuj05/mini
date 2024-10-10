mod lexer;
use lexer::{tokenize, Token};
use std::fs;

fn main() {
    let src = std::env::args().nth(1).expect("no path given");
    let file = fs::read_to_string(src).expect("file does not exist");

    let tokens: Vec<Token> = match tokenize(file.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => panic!("Lexing Error: {}", e),
    };

    tokens.iter().for_each(|t| println!("{:?}", t));
}

// TODO;
// 1. Lexer
// 2. Parser
// 3. QBE IR CodeGen
