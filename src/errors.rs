use pest::error::Error;
use crate::de::Rule;

#[derive(Debug)]
pub struct TySONError {
    msg: String,
}

impl TySONError {
    pub(crate) fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string()
        }
    }

    pub fn unexpected_parsing() -> Self {
       Self::new("Unexpected parsing error")
    }
}

impl From<Error<Rule>> for TySONError {
    fn from(error: Error<Rule>) -> Self {
        Self {
            msg: error.to_string()
        }
    }
}