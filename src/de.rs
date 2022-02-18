use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use crate::value::{Primitive, TysonValue};


#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TysonParser;


pub fn deserialize(data: String) -> Result<Vec<(Primitive, TysonValue)>, Error<Rule>> {
    let pair = TysonParser::parse(Rule::doc, data.as_str())?.next().unwrap(); // TODO catch this
    let mut result: Vec<(Primitive, TysonValue)> = vec![];

    fn parse_value(pair: Pair<Rule>) -> Result<TysonValue, Error<Rule>> {
        return match pair.as_rule() {
            Rule::map => {
                let mut data: Vec<(Primitive, TysonValue)> = vec![];
                let mut prefix: String = String::new();
                for pair in pair.into_inner()
                {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.as_str().to_string();
                        }
                        _ => {
                            let mut inner_rules = pair.into_inner();
                            if let TysonValue::Primitive(left) = parse_value(inner_rules.next().unwrap())? {
                                data.push((left, parse_value(inner_rules.next().unwrap())?))
                            }
                        }
                    }
                };
                Ok(TysonValue::Map(prefix, data))
            }
            Rule::array => {
                {
                    let mut data: Vec<TysonValue> = vec![];
                    let mut prefix: String = String::new();
                    for pair in pair.into_inner()
                    {
                        match pair.as_rule() {
                            Rule::prefix => {
                                prefix = pair.as_str().to_string();
                            }
                            _ => {
                                data.push(parse_value(pair)?);
                            }
                        }
                    };
                    Ok(TysonValue::Array(prefix, data))
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
                Ok(TysonValue::Primitive(Primitive(prefix, data)))
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
                        if let TysonValue::Primitive(key) = parse_value(v)? {
                            match inner_rules.next() {
                                Some(v) => {
                                    result.push((key, parse_value(v)?));
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