use crate::parser::{core::{expr::Expr, ops::BinaryOp, token::Token}, errors::ParserError, Parser};

impl Parser {
    pub fn or(&mut self) -> Result<Expr, ParserError> {
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

    pub fn and(&mut self) -> Result<Expr, ParserError> {
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

    pub fn equality(&mut self) -> Result<Expr, ParserError> {
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

    pub fn comparison(&mut self) -> Result<Expr, ParserError> {
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
