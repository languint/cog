use crate::parser::{
    Parser,
    core::{expr::Expr, nodes::Nodes, token::Token},
    errors::ParserError,
};

impl Parser {
    pub fn assignment(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&Token::KeywordLet) {
            if let Some(Token::Identifier(name)) = self.peek().cloned() {
                self.advance(); // consume identifier

                let var_type = if self.match_token(&Token::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                if !self.match_token(&Token::Equal) {
                    return Err(ParserError::MalformedFuncDecl(
                        "expected `=` afer identifier".into(),
                    ));
                }

                let value = self.assignment()?;

                return Ok(Expr::Declaration {
                    identifier: name,
                    var_type,
                    value: Box::new(value),
                });
            } else {
                return Err(ParserError::MalformedFuncDecl(
                    "expected identifier after `let`".into(),
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
                "assignment target must be an identifier".into(),
            ));
        }

        Ok(expr)
    }
}
