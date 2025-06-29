use std::fmt::{self};

use cog_parser::parser::core::types::Types;

#[derive(PartialEq, Debug, Clone)]
pub enum CodeGenError {
    UnknownType(Types),
    NotImplemented(String)
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CodeGenError {}
