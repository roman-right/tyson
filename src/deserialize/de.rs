use pest::iterators::Pair;
use pest::Parser;

use crate::errors::TySONError;
use crate::deserialize::item::{BaseTySONItemInterface, TySONItem};
use crate::deserialize::map::TySONMap;
use crate::deserialize::primitive::TySONPrimitive;
use crate::deserialize::vector::TySONVector;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;


pub trait Desereilize {
    fn deserialize_primitive(pair: Pair<Rule>) -> Result<Box<dyn TySONPrimitive>, TySONError> {
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

    fn deserialize_vector(pair: Pair<Rule>) -> Result<Box<dyn TySONVector>, TySONError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.into_inner().as_str().to_string();
        let mut vector = Self::new_vector(prefix);
        for pair in inner_rules
        {
            vector.push(Self::route_deserialization(pair)?);
        }
        Ok(vector)
    }

    fn deserialize_map(pair: Pair<Rule>) -> Result<Box<dyn TySONMap>, TySONError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.into_inner().as_str().to_string();
        let mut map = Self::new_map(prefix);
        for pair in inner_rules
        {
            let mut inner_rules = pair.into_inner();
            let left = Self::deserialize_primitive(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?;
            map.insert(left, Self::route_deserialization(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?)
        };
        Ok(map)
    }

    fn route_deserialization(pair: Pair<Rule>) -> Result<TySONItem, TySONError> {
        return match pair.as_rule() {
            Rule::map => {
                let res = Self::deserialize_map(pair)?;
                Ok(TySONItem::Map(res))
            }
            Rule::vector => {
                let res = Self::deserialize_vector(pair)?;
                Ok(TySONItem::Vector(res))
            }
            Rule::primitive => {
                let res = Self::deserialize_primitive(pair)?;
                Ok(TySONItem::Primitive(res))
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
                            let key = Self::deserialize_primitive(v)?;
                            result.add_to_document((key, Self::route_deserialization(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?));
                        }
                        _ => {}
                    }
                }
                Ok(result)
            }
            _ => unreachable!()
        }
    }

    fn new_document() -> Self;

    fn add_to_document(&mut self, data: (Box<dyn TySONPrimitive>, TySONItem));

    fn new_vector(prefix: String) -> Box<dyn TySONVector>;

    fn new_map(prefix: String) -> Box<dyn TySONMap>;

    fn new_primitive(prefix: String, data: String) -> Box<dyn TySONPrimitive>;
}
