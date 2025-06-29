use std::fmt::{self};

#[derive(PartialEq, Debug, Clone)]
pub enum CodeGenError {

}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CodeGenError {}
