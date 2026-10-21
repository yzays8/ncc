use crate::error::{NccError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Constant(i128),
    Keyword(Keyword),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Minus,
    MinusMinus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

pub fn lex(input: &str) -> Result<Vec<Token>> {
    let mut input = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(c) = input.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                while let Some(c) = input.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                            identifier.push(*c);
                            input.next();
                        }
                        _ => break,
                    }
                }

                let token = match identifier.as_str() {
                    "int" => Token::Keyword(Keyword::Int),
                    "void" => Token::Keyword(Keyword::Void),
                    "return" => Token::Keyword(Keyword::Return),
                    _ => Token::Identifier(identifier),
                };
                tokens.push(token);
            }
            '0'..='9' => {
                let mut constant = String::new();
                while let Some(c) = input.peek() {
                    match c {
                        '0'..='9' => {
                            constant.push(*c);
                            input.next();
                        }
                        'a'..='z' | 'A'..='Z' | '_' => {
                            return Err(NccError::Lexing(format!(
                                "Unexpected character in constant: {}",
                                c
                            )));
                        }
                        _ => break,
                    }
                }

                let value = constant.parse::<i128>()?;
                tokens.push(Token::Constant(value));
            }
            '(' => {
                tokens.push(Token::OpenParen);
                input.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                input.next();
            }
            '{' => {
                tokens.push(Token::OpenBrace);
                input.next();
            }
            '}' => {
                tokens.push(Token::CloseBrace);
                input.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                input.next();
            }
            '~' => {
                tokens.push(Token::Tilde);
                input.next();
            }
            '-' => {
                input.next();
                if let Some('-') = input.peek() {
                    tokens.push(Token::MinusMinus);
                    input.next();
                } else {
                    tokens.push(Token::Minus);
                }
            }
            ' ' | '\t' | '\n' => {
                input.next();
            }
            _ => {
                return Err(NccError::Lexing(format!("Unexpected character: {}", c)));
            }
        }
    }

    Ok(tokens)
}
