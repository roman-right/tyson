use crate::de::Desereilize;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;

pub trait TySONDocument: Desereilize {}