use crate::parser::{
    Parser,
    core::{expr::Expr, token::Token},
    errors::ParserError,
};

impl Parser {
    pub fn func_declaration(&mut self) -> Result<Expr, ParserError> {
        if !self.match_token(&Token::KeywordFn) {
            return Err(ParserError::ExpectedToken("expected `fn`".into()));
        }

        let identifier = if let Some(Token::Identifier(name)) = self.peek().cloned() {
            self.advance();
            name
        } else {
            return Err(ParserError::MalformedFuncDecl(
                "expected identifier after `fn`".into(),
            ));
        };

        if !self.match_token(&Token::LeftParen) {
            return Err(ParserError::MalformedFuncDecl(
                "expected `(` after function name".into(),
            ));
        }

        let mut parameters = Vec::new();
        if !self.match_token(&Token::RightParen) {
            loop {
                if let Some(Token::Identifier(name)) = self.peek().cloned() {
                    self.advance();
                    if !self.match_token(&Token::Colon) {
                        return Err(ParserError::MalformedFuncDecl(
                            "expected `:` after parameter name".into(),
                        ));
                    }
                    let param_type = self.parse_type()?;
                    parameters.push(Expr::Declaration {
                        identifier: name,
                        var_type: Some(param_type),
                        value: Box::new(Expr::Literal(
                            crate::parser::core::nodes::Nodes::Identifier(
                                "placeholder".to_string(),
                            ),
                        )),
                    });
                } else {
                    return Err(ParserError::MalformedFuncDecl(
                        "expected identifier in parameter list".into(),
                    ));
                }

                if self.match_token(&Token::RightParen) {
                    break;
                }

                if !self.match_token(&Token::Comma) {
                    return Err(ParserError::MalformedFuncDecl(
                        "expected `,` or `)` after parameter".into(),
                    ));
                }
            }
        }

        let return_type = if self.match_token(&Token::ArrowSmall) {
            Some(self.parse_type()?)
        } else {
            None
        };

        if !self.match_token(&Token::LeftBrace) {
            return Err(ParserError::MalformedFuncDecl(
                "expected `{` before function body".into(),
            ));
        }

        let mut body = Vec::new();
        while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }

        if self.previous() != Some(&Token::RightBrace) {
            return Err(ParserError::MalformedFuncDecl(
                "expected `}` after function body".into(),
            ));
        }

        Ok(Expr::FunctionDeclaration {
            identifier,
            parameters,
            body: Box::new(Expr::Block(body)),
            return_type,
        })
    }

    pub fn parse_return(&mut self) -> Result<Expr, ParserError> {
        if !self.match_token(&Token::KeywordReturn) {
            return Err(ParserError::MalformedReturn("expected `return`".into()));
        }

        let value = self.expression()?;

        Ok(Expr::Return {
            value: Box::new(value),
        })
    }
}
