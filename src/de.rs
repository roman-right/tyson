use pest::iterators::Pair;
use pest::Parser;

use crate::errors::TySONError;
use crate::value::{Primitive, TySONDocument, TySONValue};

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;


pub trait Desereilize<C, P> {
    fn deserialize_primitive(pair: Pair<Rule>) -> Result<P, TySONError> {
        return match pair.as_rule() {
            Rule::primitive => {
                let mut data: String = String::new();
                let mut prefix: String = String::new();
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.as_str().to_string();
                        }
                        _ => {
                            data = pair.as_str().to_string();
                        }
                    }
                };
                let val = Primitive(prefix, data);
                Ok(Self::from_primitive(val))
            }
            _ => unreachable!()
        };
    }

    fn deserialize_container(pair: Pair<Rule>) -> Result<C, TySONError> {
        return match pair.as_rule() {
            Rule::map => {
                let mut data: Vec<(P, C)> = vec![];
                let mut prefix: String = String::new();
                for pair in pair.into_inner()
                {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.as_str().to_string();
                        }
                        _ => {
                            let mut inner_rules = pair.into_inner();
                            if let left = Self::deserialize_primitive(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)? {
                                data.push((left, Self::deserialize_container(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?))
                            }
                        }
                    }
                };
                Ok(Self::from_map(prefix, data))
            }
            Rule::vector => {
                {
                    let mut data: Vec<C> = vec![];
                    let mut prefix: String = String::new();
                    for pair in pair.into_inner()
                    {
                        match pair.as_rule() {
                            Rule::prefix => {
                                prefix = pair.as_str().to_string();
                            }
                            _ => {
                                data.push(Self::deserialize_container(pair)?);
                            }
                        }
                    };
                    Ok(Self::from_vector(prefix, data))
                }
            }
            Rule::primitive => {
                Ok(Self::wrap_primitive(Self::deserialize_primitive(pair)?))
            }

            _ => unreachable!()
        };
    }

    fn deserialize(data: String) -> Result<Self, TySONError> where Self: Sized {
        let pair = TySONParser::parse(Rule::document, data.as_str())?.next().ok_or(TySONError::unexpected_parsing())?;

        let mut result = Self::new();

        match pair.as_rule() {
            Rule::document => {
                for pair in pair.into_inner() {
                    let mut inner_rules = pair.into_inner();
                    match inner_rules.next() {
                        Some(v) => {
                            if let key = Self::deserialize_primitive(v)? {
                                match inner_rules.next() {
                                    Some(v) => {
                                        result.push((key, Self::deserialize_container(v)?));
                                    }
                                    _ => unreachable!()
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(result)
            }
            _ => unreachable!()
        }
    }

    fn from_map(prefix: String, data: Vec<(P, C)>) -> C;

    fn from_vector(prefix: String, data: Vec<C>) -> C;

    fn from_primitive(val: Primitive) -> P;

    fn wrap_primitive(p: P) -> C;

    fn new() -> Self;

    fn push(&mut self, pair: (P, C));
}