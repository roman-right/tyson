use pest::iterators::Pair;
use pest::Parser;
use crate::de::Desereilize;

use crate::errors::TySONError;
use crate::primitive::TySONPrimitive;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;

pub trait TySONDocument: Desereilize {}