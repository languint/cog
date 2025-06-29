use crate::parser::{
    Parser,
    core::{expr::Expr, nodes::Nodes, token::Token},
    errors::ParserError,
};

impl Parser {
    pub fn primary(&mut self) -> Result<Expr, ParserError> {
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
                        return Err(ParserError::MalformedExpression(
                            "expected `)` after expression".into(),
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
                        return Err(ParserError::MalformedBlock(
                            "expected `}` after block".into(),
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
