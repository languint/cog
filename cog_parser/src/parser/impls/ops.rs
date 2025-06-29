use crate::parser::{core::{expr::Expr, ops::{BinaryOp, UnaryOp}, token::Token}, errors::ParserError, Parser};

impl Parser {
    pub fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&Token::NotEqual) {
            Some(BinaryOp::NotEqual)
        } else if self.match_token(&Token::EqualEqual) {
            Some(BinaryOp::Equal)
        } else {
            None
        }
    }

    pub fn match_comparison_op(&mut self) -> Option<BinaryOp> {
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

    pub fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&Token::Minus) {
            Some(BinaryOp::Subtract)
        } else if self.match_token(&Token::Plus) {
            Some(BinaryOp::Add)
        } else {
            None
        }
    }

    pub fn match_factor_op(&mut self) -> Option<BinaryOp> {
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

    pub fn match_unary_op(&mut self) -> Option<UnaryOp> {
        if self.match_token(&Token::Minus) {
            Some(UnaryOp::Minus)
        } else if self.match_token(&Token::Bang) {
            Some(UnaryOp::Not)
        } else {
            None
        }
    }
}

impl Parser {
    pub fn term(&mut self) -> Result<Expr, ParserError> {
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

    pub fn factor(&mut self) -> Result<Expr, ParserError> {
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

    pub fn unary(&mut self) -> Result<Expr, ParserError> {
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
