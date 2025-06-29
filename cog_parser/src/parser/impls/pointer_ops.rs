use crate::parser::{
    Parser,
    core::{expr::Expr, token::Token},
    errors::ParserError,
};

impl Parser {
    pub fn pointer_ops(&mut self) -> Result<Expr, ParserError> {
        if let Some(token) = self.peek().cloned() {
            match token {
                Token::Ampersand => {
                    self.advance();
                    let operand = self.primary()?;
                    Ok(Expr::AddressOf(Box::new(operand)))
                }
                Token::Star => {
                    self.advance();
                    let operand = self.primary()?;
                    Ok(Expr::Dereference(Box::new(operand)))
                }
                _ => self.primary(),
            }
        } else {
            Err(ParserError::UnexpectedEndOfInput)
        }
    }
}
