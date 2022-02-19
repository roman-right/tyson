use custom_error::custom_error;
use pest::error::Error as PestError;
use pest::error::InputLocation;
use pest::iterators::Pair;
use pest::Parser;

use crate::value::{Primitive, TYSONDocument, TYSONValue};

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TysonParser;

custom_error! {DeserialisationError
    PestError{location: InputLocation}            = "Parsing error: {location}",
    Unexpected{value: String}         = "Unexpected error: {value}"
}


pub fn deserialize_doc(data: String) -> Result<TYSONDocument, DeserialisationError> {
    let pair = TysonParser::parse(Rule::document, data.as_str())?.next().ok_or(Err(DeserialisationError::Unexpected { value: "SMTH went wrong".to_string() }))?;
    let mut result: TYSONDocument = TYSONDocument::new();

    fn deserialize_value(pair: Pair<Rule>) -> Result<TYSONValue, PestError<Rule>> {
        return match pair.as_rule() {
            Rule::map => {
                let mut data: Vec<(Primitive, TYSONValue)> = vec![];
                let mut prefix: String = String::new();
                for pair in pair.into_inner()
                {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.as_str().to_string();
                        }
                        _ => {
                            let mut inner_rules = pair.into_inner();
                            if let TYSONValue::Primitive(left) = deserialize_value(inner_rules.next().unwrap())? {
                                data.push((left, deserialize_value(inner_rules.next().unwrap())?))
                            }
                        }
                    }
                };
                Ok(TYSONValue::Map(prefix, data))
            }
            Rule::vector => {
                {
                    let mut data: Vec<TYSONValue> = vec![];
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
                    Ok(TYSONValue::Vector(prefix, data))
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
                Ok(TYSONValue::Primitive(Primitive(prefix, data)))
            }
            _ => unreachable!()
        };
    }

    match pair.as_rule() {
        Rule::doc => {
            for pair in pair.into_inner() {
                let mut inner_rules = pair.into_inner();
                match inner_rules.next() {
                    Some(v) => {
                        if let TYSONValue::Primitive(key) = deserialize_value(v)? {
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