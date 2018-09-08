use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Word(usize, String),
    Number(usize, u8),
    EOL(usize),
}

#[derive(Debug, Clone)]
pub struct TokenizerError {
    pattern: String,
    position: usize,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unexpected character {} at position {}",
            self.pattern, self.position
        )
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, TokenizerError> {
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
                return Err(TokenizerError {
                    position: i,
                    pattern: "\\n".to_string(),
                });
            } else {
                return Err(TokenizerError {
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
pub struct ParserError<'a> {
    token: &'a Token,
    expected: Token,
}

impl<'a> fmt::Display for ParserError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unexpected token {:?}, expected token {:?}",
            self.token, self.expected
        )
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<u8>, ParserError> {
    let mut i = 0;
    let mut mem: Vec<u8> = Vec::new();
    let mut name_table: HashMap<String, u8> = HashMap::new();
    let mut instruction = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        match token {
            Token::Word(_, word) => match word.to_lowercase().as_ref() {
                "dw" => match &tokens[i + 1] {
                    Token::Word(_, name) => match &tokens[i + 2] {
                        Token::Number(_, number) => match tokens[i + 3] {
                            Token::EOL(_) => {
                                i += 4;
                                name_table.insert(name.to_string(), instruction);
                                instruction += 1;
                                mem.push(*number);
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        _ => {
                            return Err(ParserError {
                                token: &tokens[i + 1],
                                expected: Token::Number(0, 0),
                            })
                        }
                    },
                    Token::Number(_, number) => match tokens[i + 2] {
                        Token::EOL(_) => {
                            i += 3;
                            instruction += 1;
                            mem.push(*number);
                        }
                        _ => {
                            return Err(ParserError {
                                token: &tokens[i + 2],
                                expected: Token::EOL(0),
                            })
                        }
                    },
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::Number(0, 0),
                        })
                    }
                },
                "halt" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x00);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "load" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x01);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "store" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x02);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "push" => match &tokens[i + 1] {
                    Token::Word(_, name) => match tokens[i + 2] {
                        Token::EOL(_) => {
                            i += 3;
                            if name_table.contains_key(name) {
                                instruction += 2;
                                mem.push(0x03);
                                mem.push(name_table[name]);
                            } else {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::Word(0, "unknown label".to_string()),
                                });
                            }
                        }
                        _ => {
                            return Err(ParserError {
                                token: &tokens[i + 2],
                                expected: Token::EOL(0),
                            })
                        }
                    },
                    Token::Number(_, number) => match tokens[i + 2] {
                        Token::EOL(_) => {
                            i += 3;
                            instruction += 2;
                            mem.push(0x03);
                            mem.push(*number);
                        }
                        _ => {
                            return Err(ParserError {
                                token: &tokens[i + 2],
                                expected: Token::EOL(0),
                            })
                        }
                    },
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::Number(0, 0),
                        })
                    }
                },
                "pop" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x04);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "add" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x05);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "sub" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x06);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "and" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x07);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "or" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x08);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "xor" => match tokens[i + 1] {
                    Token::EOL(_) => {
                        i += 2;
                        instruction += 1;
                        mem.push(0x09);
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::EOL(0),
                        })
                    }
                },
                "jp" => match &tokens[i + 1] {
                    Token::Word(_, condition) => match condition.to_lowercase().as_ref() {
                        "gt" => match tokens[i + 2] {
                            Token::EOL(_) => {
                                i += 3;
                                mem.push(0x0A);
                                mem.push(0x01);
                                instruction += 2;
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        "lt" => match tokens[i + 2] {
                            Token::EOL(_) => {
                                i += 3;
                                mem.push(0x0A);
                                mem.push(0x02);
                                instruction += 2;
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        "geq" => match tokens[i + 2] {
                            Token::EOL(_) => {
                                i += 3;
                                mem.push(0x0A);
                                mem.push(0x03);
                                instruction += 2;
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        "leq" => match tokens[i + 2] {
                            Token::EOL(_) => {
                                i += 3;
                                mem.push(0x0A);
                                mem.push(0x04);
                                instruction += 2;
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        "eq" => match tokens[i + 2] {
                            Token::EOL(_) => {
                                i += 3;
                                mem.push(0x0A);
                                mem.push(0x05);
                                instruction += 2;
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        "neq" => match tokens[i + 2] {
                            Token::EOL(_) => {
                                i += 3;
                                mem.push(0x0A);
                                mem.push(0x06);
                                instruction += 2;
                            }
                            _ => {
                                return Err(ParserError {
                                    token: &tokens[i + 2],
                                    expected: Token::EOL(0),
                                })
                            }
                        },
                        _ => {
                            return Err(ParserError {
                                token: &tokens[i + 1],
                                expected: Token::Word(
                                    0,
                                    "EOL or gt, lt, geq, leq, eq, neq".to_string(),
                                ),
                            })
                        }
                    },
                    Token::EOL(_) => {
                        i += 2;
                        mem.push(0x0A);
                        mem.push(0x00);
                        instruction += 2;
                    }
                    _ => {
                        return Err(ParserError {
                            token: &tokens[i + 1],
                            expected: Token::Number(0, 0),
                        })
                    }
                },
                _ => {
                    return Err(ParserError {
                        token,
                        expected: Token::EOL(0),
                    })
                }
            },
            _ => {
                return Err(ParserError {
                    token,
                    expected: Token::Word(0, "PUSH, POP, ADD".to_string()),
                })
            }
        }
    }

    Ok(mem)
}
