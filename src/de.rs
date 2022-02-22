use pest::iterators::Pair;
use pest::Parser;

use crate::errors::TySONError;
use crate::primitive::Primitive as TySONPrimitive;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;


pub trait Desereilize<Container, Primitive> {
    fn deserialize_primitive(pair: Pair<Rule>) -> Result<Primitive, TySONError> {
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
                Ok(Self::new_primitive(prefix, data))
            }
            _ => unreachable!()
        };
    }

    fn deserialize_container(pair: Pair<Rule>) -> Result<Container, TySONError> {
        return match pair.as_rule() {
            Rule::map => {
                let mut inner_rules = pair.into_inner();
                let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.into_inner().as_str().to_string();
                let mut map = Self::new_map(prefix);
                for pair in inner_rules
                {
                    let mut inner_rules = pair.into_inner();
                    if let left = Self::deserialize_primitive(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)? {
                        Self::add_to_map(&mut map, (left, Self::deserialize_container(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?))
                    }
                };
                Ok(map)
            }
            Rule::vector => {
                {
                    let mut inner_rules = pair.into_inner();
                    let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.into_inner().as_str().to_string();
                    let mut vector = Self::new_vector(prefix);
                    for pair in inner_rules
                    {
                        Self::add_to_vector(&mut vector, Self::deserialize_container(pair)?);
                    };
                    Ok(vector)
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

        let mut result = Self::new_document();

        match pair.as_rule() {
            Rule::document => {
                for pair in pair.into_inner() {
                    let mut inner_rules = pair.into_inner();
                    match inner_rules.next() {
                        Some(v) => {
                            if let key = Self::deserialize_primitive(v)? {
                                match inner_rules.next() {
                                    Some(v) => {
                                        result.add_to_document((key, Self::deserialize_container(v)?));
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

    fn new_map(prefix: String) -> Container;

    fn add_to_map(map: &mut Container, data: (Primitive, Container));

    fn new_vector(prefix: String) -> Container;

    fn add_to_vector(vector: &mut Container, data: Container);

    fn new_primitive(prefix: String, value: String) -> Primitive;

    fn wrap_primitive(p: Primitive) -> Container;

    fn new_document() -> Self;

    fn add_to_document(&mut self, pair: (Primitive, Container));
}
