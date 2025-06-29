use logos::Logos;

use crate::parser::{core::token::Token, errors::ParserError};

pub mod core;
pub mod errors;
pub mod impls;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    source: String,
}

impl Parser {
    pub fn new(input: String) -> Result<Self, ParserError> {
        let mut lexer = Token::lexer(&input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            match token {
                Ok(t) => tokens.push(t),
                Err(_) => {
                    let slice = lexer.slice();
                    if let Ok(num) = slice.parse::<i64>() {
                        tokens.push(Token::Integer(num));
                    } else if let Ok(num) = slice.parse::<f64>() {
                        tokens.push(Token::Float(num));
                    } else if slice.starts_with('"') && slice.ends_with('"') {
                        let string_content = slice[1..slice.len() - 1].into();
                        tokens.push(Token::String(string_content));
                    } else if slice == "true" || slice == "false" {
                        tokens.push(Token::Boolean(slice == "true"));
                    } else if slice.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        tokens.push(Token::Identifier(slice.into()));
                    } else {
                        return Err(ParserError::UnknownCharInInput(
                            slice.chars().next().unwrap(),
                        ));
                    }
                }
            }
        }

        Ok(Parser {
            tokens,
            current_index: 0,
            source: input,
        })
    }
}

impl Parser {
    /// Peeks into the next token, if the token matches it is consumed.
    fn match_token(&mut self, expected: &Token) -> bool {
        if let Some(t) = self.peek() {
            if std::mem::discriminant(t) == std::mem::discriminant(expected) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current_index)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current_index += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.tokens.len()
    }

    fn previous(&self) -> Option<&Token> {
        if self.current_index > 0 {
            self.tokens.get(self.current_index - 1)
        } else {
            None
        }
    }
}
