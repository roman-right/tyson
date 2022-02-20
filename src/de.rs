use pest::iterators::Pair;
use pest::Parser;
use crate::errors::TySONError;

use crate::value::{Primitive, TySONDocument, TySONValue};

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TysonParser;


pub fn deserialize(data: String) -> Result<TySONDocument, TySONError> {
    let pair = TysonParser::parse(Rule::document, data.as_str())?.next().ok_or(TySONError::unexpected_parsing())?;

    let mut result: TySONDocument = TySONDocument::new();

    fn deserialize_value(pair: Pair<Rule>) -> Result<TySONValue, TySONError> {
        return match pair.as_rule() {
            Rule::map => {
                let mut data: Vec<(Primitive, TySONValue)> = vec![];
                let mut prefix: String = String::new();
                for pair in pair.into_inner()
                {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.as_str().to_string();
                        }
                        _ => {
                            let mut inner_rules = pair.into_inner();
                            if let TySONValue::Primitive(left) = deserialize_value(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)? {
                                data.push((left, deserialize_value(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?))
                            }
                        }
                    }
                };
                Ok(TySONValue::Map(prefix, data))
            }
            Rule::vector => {
                {
                    let mut data: Vec<TySONValue> = vec![];
                    let mut prefix: String = String::new();
                    for pair in pair.into_inner()
                    {
                        match pair.as_rule() {
                            Rule::prefix => {
                                prefix = pair.as_str().to_string();
                            }
                            _ => {
                                data.push(deserialize_value(pair)?);
                            }
                        }
                    };
                    Ok(TySONValue::Vector(prefix, data))
                }
            }
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
                Ok(TySONValue::Primitive(Primitive(prefix, data)))
            }
            _ => unreachable!()
        };
    }

    match pair.as_rule() {
        Rule::document => {
            for pair in pair.into_inner() {
                let mut inner_rules = pair.into_inner();
                match inner_rules.next() {
                    Some(v) => {
                        if let TySONValue::Primitive(key) = deserialize_value(v)? {
                            match inner_rules.next() {
                                Some(v) => {
                                    result.push((key, deserialize_value(v)?));
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
