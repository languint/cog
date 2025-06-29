use crate::parser::{core::{expr::Expr, token::Token}, errors::ParserError, Parser};

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

    pub fn statement(&mut self) -> Result<Expr, ParserError> {
        let expr = self.expression()?;

        // consume `;`
        self.match_token(&Token::Semicolon);

        Ok(expr)
    }

    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        if let Some(Token::KeywordIf) = self.peek() {
            return self.if_else();
        }
        self.assignment()
    }
}