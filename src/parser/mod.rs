use crate::lexer::{Token, Word};

fn _parse_helper(tokens: &Vec<Token>, index: &mut usize, len: &usize) {
    if *index == *len {
        if *index == 0 {
            panic!("syntax error");
        }
        return;
    } else {
        if let Some(current) = tokens.get(*index) {
            match current {
                Token::Value(..) => {
                    if let Some(next) = tokens.get(*index + 1) {
                        match next {
                            Token::Value(..) => {
                                panic!("syntax error")
                            }
                            _ => {
                                *index += 2;
                                _parse_helper(&tokens, index, len)
                            }
                        }
                    }
                }
                _ => {
                    panic!("syntax error")
                }
            }
        } else {
            panic!("syntax error")
        }
    }
}

pub fn parse(tokens: &Vec<Token>) -> Vec<&Token> {
    let mut _index = 0;
    let _len = tokens.len() - 1;
    // parse_helper(&tokens, &mut index, &len);
    reverse_polish(&tokens)
}

fn reverse_polish_helper<'a>(
    expression: &mut Vec<&'a Token>,
    operands: &mut Vec<&'a Token>,
    token: &Token,
) {
    let mut finished = false;
    if let Token::Symbol(_, v) = token {
        while !finished {
            if operands.is_empty() {
                finished = true;
                continue;
            }
            if let Token::Symbol(_, vv) = operands.last().unwrap() {
                if v <= vv {
                    let last = operands.pop().unwrap();
                    expression.push(last);
                } else {
                    finished = true;
                }
            }
        }
    }
}

fn reverse_polish(tokens: &Vec<Token>) -> Vec<&Token> {
    let mut expression: Vec<&Token> = Vec::new();
    let mut operands: Vec<&Token> = Vec::new();

    for token in tokens {
        if let Token::Symbol(s, v) = token {
            if operands.is_empty() {
                operands.push(token);
            } else {
                if let Token::Symbol(_, vv) = operands.last().unwrap() {
                    if v > vv {
                        operands.push(token);
                    } else {
                        reverse_polish_helper(&mut expression, &mut operands, token);
                        match s {
                            Word::Semicolon => (),
                            _ => operands.push(token),
                        }
                    }
                }
            }
        } else {
            expression.push(token);
        }
    }

    while !operands.is_empty() {
        let last = operands.pop().unwrap();
        expression.push(last);
    }

    expression
}
