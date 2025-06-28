use logos::Logos;

use crate::parser::{
    core::expr::Expr,
    core::nodes::Nodes,
    core::ops::{BinaryOp, UnaryOp},
    core::token::Token,
    core::types::Types,
    errors::ParserError,
};

pub mod core;
pub mod errors;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    source: String,
}

/* constructor */
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
                        return Err(ParserError::UnknownChar(slice.chars().next().unwrap()));
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

/* state manipulation */
impl Parser {
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

/* parse */
impl Parser {
    pub fn parse(&mut self) -> Result<Vec<Expr>, ParserError> {
        let mut statements = Vec::new();

        loop {
            if self.is_at_end() {
                break;
            }
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Expr, ParserError> {
        let expr = self.expression()?;

        // consume `;`
        self.match_token(&Token::Semicolon);

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        if let Some(Token::KeywordIf) = self.peek() {
            return self.if_else();
        }
        self.assignment()
    }
}

/* types */
impl Parser {
    fn parse_type(&mut self) -> Result<Types, ParserError> {
        if let Some(token) = self.peek().cloned() {
            match token {
                Token::Identifier(type_name) => {
                    self.advance();
                    match type_name.as_str() {
                        "i32" => Ok(Types::I32),
                        "i64" => Ok(Types::I64),
                        "u32" => Ok(Types::U32),
                        "u64" => Ok(Types::U64),
                        "f32" => Ok(Types::F32),
                        "f64" => Ok(Types::F64),
                        "bool" => Ok(Types::Bool),
                        "String" => Ok(Types::String),
                        _ => Err(ParserError::UnexpectedToken(format!(
                            "unknown type: {}",
                            type_name
                        ))),
                    }
                }
                Token::KeywordTypeI32 => {
                    self.advance();
                    Ok(Types::I32)
                }
                Token::KeywordTypeI64 => {
                    self.advance();
                    Ok(Types::I64)
                }
                Token::KeywordTypeU32 => {
                    self.advance();
                    Ok(Types::U32)
                }
                Token::KeywordTypeU64 => {
                    self.advance();
                    Ok(Types::U64)
                }
                Token::KeywordTypeF32 => {
                    self.advance();
                    Ok(Types::F32)
                }
                Token::KeywordTypeF64 => {
                    self.advance();
                    Ok(Types::F64)
                }
                Token::KeywordTypeBool => {
                    self.advance();
                    Ok(Types::Bool)
                }
                Token::KeywordTypeString => {
                    self.advance();
                    Ok(Types::String)
                }
                _ => {
                    dbg!(self.peek().cloned());
                    Err(ParserError::ExpectedToken("type".into()))
                }
            }
        } else {
            Err(ParserError::ExpectedToken("type".into()))
        }
    }
}

/* variables */
impl Parser {
    fn assignment(&mut self) -> Result<Expr, ParserError> {
        // if `let` exists, it's consumed here.
        if self.match_token(&Token::KeywordLet) {
            if let Some(Token::Identifier(name)) = self.peek().cloned() {
                self.advance(); // consume identifier

                let var_type = if self.match_token(&Token::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                if !self.match_token(&Token::Equal) {
                    return Err(ParserError::ExpectedAfterCustom(
                        "=".into(),
                        "".into(),
                        "identifier".into(),
                        self.source[self.current_index..].to_string(),
                    ));
                }

                let value = self.assignment()?;

                return Ok(Expr::Declaration {
                    identifier: name,
                    var_type,
                    value: Box::new(value),
                });
            } else {
                return Err(ParserError::ExpectedAfter(
                    "identifier".into(),
                    "let".into(),
                    self.source[self.current_index..].to_string(),
                ));
            }
        }

        let expr = self.or()?;

        if self.match_token(&Token::Equal) {
            if let Expr::Literal(Nodes::Identifier(name)) = expr {
                let value = self.assignment()?;
                return Ok(Expr::Assignment {
                    identifier: name,
                    value: Box::new(value),
                });
            }
            return Err(ParserError::InvalidAssignment(
                "target must be an identifier".into(),
            ));
        }

        Ok(expr)
    }
}

/* binary operators */
impl Parser {
    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;

        while self.match_token(&Token::Or) {
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(BinaryOp::Or),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;

        while self.match_token(&Token::And) {
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(BinaryOp::And),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.match_equality_op() {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while let Some(op) = self.match_comparison_op() {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
}

/* unary operators */
impl Parser {
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while let Some(op) = self.match_term_op() {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while let Some(op) = self.match_factor_op() {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if let Some(op) = self.match_unary_op() {
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                operator: Box::new(op),
                operand: Box::new(expr),
            });
        }

        self.primary()
    }
}

/* operator matches */
impl Parser {
    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&Token::NotEqual) {
            Some(BinaryOp::NotEqual)
        } else if self.match_token(&Token::EqualEqual) {
            Some(BinaryOp::Equal)
        } else {
            None
        }
    }

    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&Token::Greater) {
            Some(BinaryOp::Greater)
        } else if self.match_token(&Token::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else if self.match_token(&Token::Less) {
            Some(BinaryOp::Less)
        } else if self.match_token(&Token::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else {
            None
        }
    }

    fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&Token::Minus) {
            Some(BinaryOp::Subtract)
        } else if self.match_token(&Token::Plus) {
            Some(BinaryOp::Add)
        } else {
            None
        }
    }

    fn match_factor_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&Token::Slash) {
            Some(BinaryOp::Divide)
        } else if self.match_token(&Token::Star) {
            Some(BinaryOp::Multiply)
        } else if self.match_token(&Token::Percent) {
            Some(BinaryOp::Modulo)
        } else {
            None
        }
    }

    fn match_unary_op(&mut self) -> Option<UnaryOp> {
        if self.match_token(&Token::Minus) {
            Some(UnaryOp::Minus)
        } else if self.match_token(&Token::Bang) {
            Some(UnaryOp::Not)
        } else {
            None
        }
    }
}

/* primary */
impl Parser {
    fn primary(&mut self) -> Result<Expr, ParserError> {
        if let Some(token) = self.peek().cloned() {
            match token {
                Token::Integer(value) => {
                    self.advance();
                    Ok(Expr::Literal(Nodes::Integer(value)))
                }
                Token::Float(value) => {
                    self.advance();
                    Ok(Expr::Literal(Nodes::Float(value)))
                }
                Token::String(value) => {
                    self.advance();
                    Ok(Expr::Literal(Nodes::String(value)))
                }
                Token::Boolean(value) => {
                    self.advance();
                    Ok(Expr::Literal(Nodes::Boolean(value)))
                }
                Token::Identifier(name) => {
                    self.advance();
                    Ok(Expr::Literal(Nodes::Identifier(name)))
                }
                Token::LeftParen => {
                    self.advance(); // consume `(`
                    let expr = self.expression()?;
                    if !self.match_token(&Token::RightParen) {
                        return Err(ParserError::ExpectedAfter(
                            ")".into(),
                            "expression".into(),
                            self.source[self.current_index..].to_string(),
                        ));
                    }
                    Ok(expr)
                }
                Token::LeftBrace => {
                    self.advance(); // consume `{`
                    let mut statements = Vec::new();

                    while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
                        statements.push(self.statement()?);
                    }

                    if self.previous() != Some(&Token::RightBrace) {
                        return Err(ParserError::ExpectedAfter(
                            "}".into(),
                            "block".into(),
                            self.source[self.current_index..].to_string(),
                        ));
                    }

                    Ok(Expr::Block(statements))
                }

                _ => Err(ParserError::UnexpectedToken(format!("{:?}", token))),
            }
        } else {
            Err(ParserError::UnexpectedEndOfInput)
        }
    }
}

/* if else */
impl Parser {
    fn if_else(&mut self) -> Result<Expr, ParserError> {
        if !self.match_token(&Token::KeywordIf) {
            return Err(ParserError::ExpectedToken("if".into()));
        }

        let condition_expr = self.expression()?;
        let condition = Box::new(condition_expr);

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParserError::ExpectedAfter(
                "{".into(),
                "if-condition".into(),
                self.source[self.current_index..].to_string(),
            ));
        }

        let mut then_statements = Vec::new();
        while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
            then_statements.push(self.statement()?);
        }

        if self.previous() != Some(&Token::RightBrace) {
            return Err(ParserError::ExpectedAfter(
                "}".into(),
                "if-block".into(),
                self.source[self.current_index..].to_string(),
            ));
        }

        let then_branch = Expr::Block(then_statements);

        let else_branch = if self.match_token(&Token::KeywordElse) {
            if !self.match_token(&Token::LeftBrace) {
                return Err(ParserError::ExpectedAfter(
                    "{".into(),
                    "else".into(),
                    self.source[self.current_index..].to_string(),
                ));
            }

            let mut else_statements = Vec::new();
            while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
                else_statements.push(self.statement()?);
            }

            if self.previous() != Some(&Token::RightBrace) {
                return Err(ParserError::ExpectedAfter(
                    "}".into(),
                    "else-block".into(),
                    self.source[self.current_index..].to_string(),
                ));
            }

            Some(Box::new(Expr::Block(else_statements)))
        } else {
            None
        };

        Ok(Expr::IfElse {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }
}
