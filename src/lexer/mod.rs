use regex::Regex;

#[derive(Debug)]
pub enum Final {
    Integer(i32),
}

#[derive(Debug)]
pub enum Word {
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Semicolon,
    Int,
}

#[derive(Debug)]
pub enum Token {
    Symbol(Word, u8),
    Value(Final),
}

pub enum Symbol<'a> {
    Value(&'a str),
}



fn lex_helper(helper: &mut String, v: &mut Vec<Token>) {
    let mut helper_match = true;
    match helper.as_str() {
        "print" => {
            v.push(Token::Symbol(Word::Print, 0));
        }
        "int" => {
            v.push(Token::Symbol(Word::Int, 0));
        }
        _ => {
            helper_match = false;
        }
    }

    if helper_match {
        helper.clear();
        return;
    }

    let re_integer = Regex::new(r"[0-9]+").unwrap();
    let integer = re_integer.is_match(helper);
    if integer {
        v.push(Token::Value(Final::Integer(helper.parse().unwrap())));
    }

    helper.clear();
}

pub fn lex(source: &String) -> Vec<Token> {
    let mut v: Vec<Token> = Vec::new();
    let mut helper = String::new();
    for c in source.chars() {
        match c {
            '+' => {
                lex_helper(&mut helper, &mut v);
                v.push(Token::Symbol(Word::Add, 1));
            }
            '-' => {
                lex_helper(&mut helper, &mut v);
                v.push(Token::Symbol(Word::Sub, 1));
            }
            '*' => {
                lex_helper(&mut helper, &mut v);
                v.push(Token::Symbol(Word::Mul, 2));
            }
            '/' => {
                lex_helper(&mut helper, &mut v);
                v.push(Token::Symbol(Word::Div, 2));
            }
            ';' => {
                lex_helper(&mut helper, &mut v);
                v.push(Token::Symbol(Word::Semicolon, 0));
            }
            ' ' => {
                lex_helper(&mut helper, &mut v);
            }
            '\n' => (),
            _ => {
                helper.push(c);
            }
        }
    }
    lex_helper(&mut helper, &mut v);
    println!("{}", helper);

    v
}
