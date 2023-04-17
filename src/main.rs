pub mod lexer;
pub mod parser;
pub mod interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_path = &args[1];
    let source = fs::read_to_string(source_path).unwrap(); 

    let tokens = lexer::lex(&source);

    let parsed = parser::parse(&tokens);

    println!("{:?}", parsed);

    let _l = interpreter::interpret(&parsed);
}
