pub mod lexer;
pub mod parser;
pub mod interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_path = &args[1];
    let source = fs::read_to_string(source_path).unwrap(); 

    let (tokens, mut table) = lexer::lex(&source);

    println!("{:?}\n",tokens);

    let parsed = parser::parse(&tokens);

    let _l = interpreter::interpret(&parsed, &mut table);
}
