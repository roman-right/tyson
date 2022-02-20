use pest::error::Error;
use crate::de::Rule;

#[derive(Debug)]
pub struct TySONError {
    msg: String,
}

impl TySONError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string()
        }
    }
}

impl From<Error<Rule>> for TySONError {
    fn from(error: Error<Rule>) -> Self {
        Self {
            msg: error.to_string()
        }
    }
}