use std::fmt::{self};

#[derive(PartialEq, Debug, Clone)]
pub enum CheckError {}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CheckError {}
