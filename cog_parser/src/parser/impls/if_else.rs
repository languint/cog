use crate::parser::{
    Parser,
    core::{expr::Expr, token::Token},
    errors::ParserError,
};

impl Parser {
    pub fn if_else(&mut self) -> Result<Expr, ParserError> {
        if !self.match_token(&Token::KeywordIf) {
            return Err(ParserError::MalformedIfElse("expected `if`".into()));
        }

        let condition_expr = self.expression()?;
        let condition = Box::new(condition_expr);

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParserError::MalformedIfElse(
                "expected `{` after condition".into(),
            ));
        }

        let mut then_statements = Vec::new();
        while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
            then_statements.push(self.statement()?);
        }

        if self.previous() != Some(&Token::RightBrace) {
            return Err(ParserError::MalformedIfElse(
                "expected `}` after if-block".into(),
            ));
        }

        let then_branch = Expr::Block(then_statements);

        let else_branch = if self.match_token(&Token::KeywordElse) {
            if !self.match_token(&Token::LeftBrace) {
                return Err(ParserError::MalformedIfElse(
                    "expected `{` after `else`".into(),
                ));
            }

            let mut else_statements = Vec::new();
            while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
                else_statements.push(self.statement()?);
            }

            if self.previous() != Some(&Token::RightBrace) {
                return Err(ParserError::MalformedIfElse("`}` else-block".into()));
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
