use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TysonParser;

#[derive(Debug)]
pub struct Primitive(String, String);

#[derive(Debug)]
pub enum TysonValue {
    Primitive(Primitive),
    Map(String, Vec<(Primitive, TysonValue)>),
    Array(String, Vec<TysonValue>),
}

pub fn parse_doc(data: String) -> Result<Vec<(Primitive, TysonValue)>, Error<Rule>> {
    let pair = TysonParser::parse(Rule::doc, data.as_str())?.next().unwrap(); // TODO catch this
    let mut result: Vec<(Primitive, TysonValue)> = vec![];

    fn parse_value(pair: Pair<Rule>) -> Result<TysonValue, Error<Rule>> {
        return match pair.as_rule() {
            Rule::map => {
                let mut pairs: Vec<(Primitive, TysonValue)> = vec![];
                let mut prefix: String = String::new();
                for pair in pair.into_inner()
                {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.into_inner().as_str().to_string();
                        }
                        _ => {
                            let mut inner_rules = pair.into_inner();
                            if let TysonValue::Primitive(left) = parse_value(inner_rules.next().unwrap())? {
                                pairs.push((left, parse_value(inner_rules.next().unwrap())?))
                            }
                        }
                    }
                };
                Ok(TysonValue::Map(prefix, pairs))
            }
            Rule::array => {
                {
                    let mut values: Vec<TysonValue> = vec![];
                    let mut prefix: String = String::new();
                    for pair in pair.into_inner()
                    {
                        match pair.as_rule() {
                            Rule::prefix => {
                                prefix = pair.into_inner().as_str().to_string();
                            }
                            _ => {
                                let mut inner_rules = pair.into_inner();
                                values.push(parse_value(inner_rules.next().unwrap())?)
                            }
                        }
                    };
                    Ok(TysonValue::Array(prefix, values))
                }
            }
            Rule::primitive => {
                let mut value: String = String::new();
                let mut prefix: String = String::new();
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::prefix => {
                            prefix = pair.as_str().to_string();
                        }
                        _ => {
                            value = pair.as_str().to_string();
                        }
                    }
                };
                Ok(TysonValue::Primitive(Primitive(prefix, value)))
            }
            _ => unreachable!()
        }
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