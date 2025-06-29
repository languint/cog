use crate::parser::{
    Parser,
    core::{token::Token, types::Types},
    errors::ParserError,
};

impl Parser {
    pub fn parse_type(&mut self) -> Result<Types, ParserError> {
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
                        _ => Err(ParserError::UnknownType(type_name)),
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
