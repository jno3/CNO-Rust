use regex::Regex;

#[derive(Debug)]
pub enum Final {
    Integer(u32),
    Symbol(usize),
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
    Equals,
    EqualsEquals,
    DiffEquals,
    GreaterThan,
    LowerThan,
    LowerEquals,
    GreaterEquals,
}

#[derive(Debug)]
pub enum Token {
    Symbol(Word, u8),
    Value(Final),
}

#[derive(Debug)]
pub struct Table {
    pub symbols: Vec<String>,
    pub status: Vec<bool>,
    pub registers: Vec<usize>,
}

fn lex_helper(helper: &mut String, tokens: &mut Vec<Token>, table: &mut Table) {
    let mut helper_match = true;
    match helper.as_str() {
        "print" => {
            tokens.push(Token::Symbol(Word::Print, 0));
        }
        "int" => {
            tokens.push(Token::Symbol(Word::Int, 0));
        }
        "==" => {
            tokens.push(Token::Symbol(Word::EqualsEquals, 1));
        }
        "!=" => {
            tokens.push(Token::Symbol(Word::DiffEquals, 1));
        }
        "<=" => {
            tokens.push(Token::Symbol(Word::LowerEquals, 1));
        }
        ">=" => {
            tokens.push(Token::Symbol(Word::GreaterEquals, 1));
        }
        "=" => {
            tokens.push(Token::Symbol(Word::Equals, 0));
        }
        "<" => {
            tokens.push(Token::Symbol(Word::LowerThan, 1));            
        }
        ">" => {
            tokens.push(Token::Symbol(Word::GreaterThan, 1))
        }
        _ => {
            helper_match = false;
        }
    }

    if helper_match {
        helper.clear();
        return;
    }

    let re_name = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*").unwrap();
    let name_match = re_name.is_match(helper);
    if name_match {
        let i;
        if !table.symbols.contains(&helper) {
            println!("{}", helper);
            i = table.symbols.len();
            table.symbols.push(helper.clone());
            table.status.push(false);
        } else {
            i = table.symbols.iter().position(|s| s.eq(helper)).unwrap();
        }

        tokens.push(Token::Value(Final::Symbol(i.try_into().unwrap())));
    }

    let re_integer = Regex::new(r"[0-9]+").unwrap();
    let integer_match = re_integer.is_match(helper);
    if integer_match {
        tokens.push(Token::Value(Final::Integer(helper.parse().unwrap())));
        helper.clear();
        return;
    }

    helper.clear();
}

pub fn lex(source: &String) -> (Vec<Token>, Table) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut sign_helper = '-';
    let mut table = Table {
        symbols: Vec::new(),
        status: Vec::new(),
        registers: Vec::new(),
    };

    let mut helper = String::new();
    for c in source.chars() {
        match c {
            '+' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::Add, 1));
            }
            '-' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::Sub, 1));
            }
            '*' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::Mul, 2));
            }
            '/' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::Div, 2));
            }
            ';' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::Semicolon, 0));
            }
            '=' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::Equals, 0));
            }
            '<' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::LowerThan, 1));
            }
            '>' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
                tokens.push(Token::Symbol(Word::GreaterThan, 1));
            }
            '!' => {

            }
            ' ' => {
                lex_helper(&mut helper, &mut tokens, &mut table);
            }
            '\n' => (),
            _ => {
                helper.push(c);
            }
        }
    }
    lex_helper(&mut helper, &mut tokens, &mut table);
    println!("{}", helper);

    (tokens, table)
}
