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
    reverse_polish(&tokens)
}

fn reverse_polish_helper<'a>(
    expression: &mut Vec<&'a Token>,
    operands: &mut Vec<&'a Token>,
    token: &'a Token,
) {
    let mut finished = false;
    while !finished {
        if operands.is_empty() {
            finished = true;
            continue;
        }
        let top = operands.last().unwrap();
        match top {
            Token::Symbol(_, vv) => {
                match token {
                    Token::Symbol(_, v) => {
                        println!("sym {:?}", token);
                        if vv >= v {
                            let last = operands.pop().unwrap();
                            expression.push(last);
                        } else {
                            finished = true;
                        }
                    }
                    Token::ObligatorySymbol(_, _, _) => {
                        let last = operands.pop().unwrap();
                        expression.push(last);
                    }
                    _ => (),
                };
            }
            Token::ObligatorySymbol(_, opening, _) => match token {
                Token::Symbol(..) => {
                    finished = true;
                }
                Token::ObligatorySymbol(_, closing, _) => {
                    if (*opening == '(' && *closing == ')') || (*opening == '{' && *closing == '}') {
                        operands.pop();
                        finished = true;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn reverse_polish(tokens: &Vec<Token>) -> Vec<&Token> {
    let mut expression: Vec<&Token> = Vec::new();
    let mut operands: Vec<&Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Symbol(s, v) => {
                if operands.is_empty() {
                    operands.push(token);
                } else {
                    let top = operands.last().unwrap();
                    match top {
                        Token::Symbol(_, vv) => {
                            if vv < v {
                                operands.push(token);
                            } else {
                                reverse_polish_helper(&mut expression, &mut operands, token);
                                match s {
                                    Word::Semicolon => (),
                                    _ => {
                                        operands.push(token);
                                    }
                                }
                            }
                        }
                        Token::ObligatorySymbol(..) => {
                            operands.push(token);
                        }
                        _ => {
                            operands.push(token);
                        }
                    }
                }
            }
            Token::ObligatorySymbol(_, _, close) => {
                if *close {
                    reverse_polish_helper(&mut expression, &mut operands, token);
                } else {
                    operands.push(token);
                }
            }
            Token::Value(..) => {
                expression.push(token);
            }
        }
    }

    while !operands.is_empty() {
        let last = operands.pop().unwrap();
        expression.push(last);
    }

    expression
}
