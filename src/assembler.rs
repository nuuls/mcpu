use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Word(usize, String),
    Number(usize, u8),
    EOL(usize),
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pattern: String,
    position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unexpected character {} at position {}",
            self.pattern, self.position
        )
    }
}

pub fn parse(source: &str) -> Result<Vec<Token>, ParseError> {
    let mut expect_letter = true;
    let mut expect_newline = true;
    let mut expect_number = false;
    let mut expect_space = true;
    let mut expect_hex = true;

    let mut tokens: Vec<Token> = Vec::new();

    let mut curr_token = String::new();

    let chars: Vec<char> = source.chars().collect();

    let mut i = 0usize;

    while i < chars.len() {
        let el = chars[i];

        let res = match el {
            ' ' => if expect_space {
                if expect_letter {
                    tokens.push(Token::Word(i - curr_token.len(), curr_token.clone()));
                    curr_token = String::new();
                } else if expect_number {
                    tokens.push(Token::Number(
                        i - curr_token.len() - 2,
                        u8::from_str_radix(&curr_token, 16).unwrap(),
                    ));
                    curr_token = String::new();
                }

                expect_letter = true;
                expect_newline = true;
                expect_number = false;
                expect_space = true;
                expect_hex = true;

                true
            } else {
                false
            },
            ';' => if expect_newline {
                if expect_letter {
                    tokens.push(Token::Word(i - curr_token.len(), curr_token.clone()));
                    curr_token = String::new();
                } else if expect_number {
                    tokens.push(Token::Number(
                        i - curr_token.len() - 2,
                        u8::from_str_radix(&curr_token, 16).unwrap(),
                    ));
                    curr_token = String::new();
                }

                expect_letter = true;
                expect_newline = true;
                expect_number = false;
                expect_space = true;
                expect_hex = true;

                tokens.push(Token::EOL(i));

                true
            } else {
                false
            },
            '0'...'9' | 'A'...'Z' | 'a'...'z' => {
                if expect_hex && el == '0' && chars[i + 1] == 'x' {
                    expect_number = true;
                    expect_letter = false;
                    expect_newline = false;
                    expect_space = false;
                    expect_hex = false;
                    i += 1;
                    true
                } else if expect_number {
                    curr_token.push(el);
                    expect_newline = true;
                    expect_space = true;
                    true
                } else if expect_letter {
                    curr_token.push(el);
                    expect_number = false;
                    expect_hex = false;
                    expect_newline = true;
                    expect_space = true;
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        if !res {
            if el == '\n' {
                return Err(ParseError {
                    position: i,
                    pattern: "\\n".to_string(),
                });
            } else {
                return Err(ParseError {
                    position: i,
                    pattern: el.to_string(),
                });
            }
        }

        i += 1;
    }

    Ok(tokens)
}

#[derive(Debug, Clone)]
pub struct AssembleError<'a> {
    token: &'a Token,
    expected: Token,
}

impl<'a> fmt::Display for AssembleError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unexpected token {:?}, expected token {:?}",
            self.token, self.expected
        )
    }
}

pub fn assemble(tokens: &Vec<Token>) -> Result<Vec<u8>, AssembleError> {
    let mut i = 0;
    let mut mem = Vec::new();

    while i < tokens.len() {
        let token = &tokens[i];

        match token {
            Token::Word(_, word) => match word.to_lowercase().as_ref() {
                "push" => match tokens[i + 1] {
                    Token::Number(_, number) => match tokens[i + 2] {
                        Token::EOL(_) => {
                            i += 3;
                            mem.push(0x01);
                            mem.push(number);
                        }
                        _ => {
                            return Err(AssembleError {
                                token: &tokens[i + 2],
                                expected: Token::EOL(0),
                            })
                        }
                    },
                    _ => {
                        return Err(AssembleError {
                            token: &tokens[i + 1],
                            expected: Token::Number(0, 0),
                        })
                    }
                },
                "pop" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        mem.push(0x02);
                    }
                    _ => {
                        return Err(AssembleError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "add" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        mem.push(0x03);
                    }
                    _ => {
                        return Err(AssembleError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                _ => {
                    return Err(AssembleError {
                        token,
                        expected: Token::EOL(0),
                    })
                }
            },
            _ => {
                return Err(AssembleError {
                    token,
                    expected: Token::Word(0, "PUSH, POP, ADD".to_string()),
                })
            }
        }
    }

    Ok(mem)
}
