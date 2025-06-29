use std::fmt::{self};

#[derive(PartialEq, Debug, Clone)]
pub enum ParserError {
    UnknownType(String),
    UnknownCharInInput(char),
    MalformedBinaryOperator(String),
    MalformedFuncDecl(String),
    MalformedReturn(String),
    MalformedVarDecl(String),
    MalformedIfElse(String),
    MalformedExpression(String),
    MalformedBlock(String),
    InvalidAssignment(String),
    ExpectedToken(String),
    UnexpectedToken(String),
    UnexpectedEndOfInput,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParserError {}
