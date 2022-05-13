use crate::deserialize::de::Desereilize;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;

pub trait TySONDocument: Desereilize {}