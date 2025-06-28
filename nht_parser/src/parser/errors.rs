use std::fmt;

use owo_colors::OwoColorize;

const ERROR_PREFIX: &str = "P";

#[derive(PartialEq)]
pub enum ParserError {
    UnknownChar(char),
    UnexpectedToken(String),
    UnexpectedEndOfInput,
    ExpectedToken(String),
    ExpectedAfter(
        /* expected */ String,
        /* after */ String,
        /* slice */ String,
    ),
    ExpectedAfterCustom(
        /* msg */ String,
        /* expected */ String,
        /* after */ String,
        /* slice */ String,
    ),
    InvalidAssignment(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", get_error_message(self))
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", get_error_message(self))
    }
}

impl std::error::Error for ParserError {}

pub fn get_error_message(err: &ParserError) -> String {
    let msg: (i32, String) = match err {
        ParserError::UnknownChar(c) => (0, format!("Unknown character `{}`", c)),
        ParserError::UnexpectedToken(token) => (1, format!("Unexpected token `{}`", token)),
        ParserError::UnexpectedEndOfInput => (2, "Unexpected end of input".to_string()),
        ParserError::ExpectedToken(token) => (3, format!("Expected token `{}`", token)),
        ParserError::ExpectedAfter(expected, after, slice) => (
            4,
            format!("Expected token `{}` after `{}`\n{}", expected, after, slice),
        ),
        ParserError::ExpectedAfterCustom(msg, expected, after, slice) => (
            4,
            format!("Expected {} after {} {}\n{}", msg, expected, after, slice),
        ),
        ParserError::InvalidAssignment(msg) => (5, format!("Invalid assignment {}", msg)),
    };

    format!(
        "{}[{}{:04}]{} {}",
        "error".red(),
        ERROR_PREFIX.red(),
        msg.0.red(),
        ":".bold(),
        msg.1.bold()
    )
}
