use std::collections::HashMap;
use std::hash::Hash;
use crate::deserialize::de::Desereilize;
use crate::deserialize::document::TySONDocument;
use crate::deserialize::item::{BaseTySONItemInterface, TySONItem};
use crate::deserialize::map::TySONMap;
use crate::deserialize::primitive::TySONPrimitive;
use crate::deserialize::vector::TySONVector;
use crate::serialize::se::{SerializeDoc, SerializeItem};


pub trait HashablePrimitive: Hash + PartialEq + TySONPrimitive {}

#[derive(Debug, Hash, PartialEq)]
pub struct IntPrimitive {
    value: i64,
}

#[derive(Debug, Hash, PartialEq)]
pub struct StrPrimitive {
    value: String,
}

impl BaseTySONItemInterface for IntPrimitive {
    fn get_prefix(&self) -> String {
        "n".to_string()
    }
}


impl HashablePrimitive for IntPrimitive {}

impl HashablePrimitive for StrPrimitive {}

impl TySONPrimitive for IntPrimitive {
    fn new(value: String) -> Self {
        let num: i64 = value.parse().unwrap();
        Self {
            value: num
        }
    }
}

impl BaseTySONItemInterface for StrPrimitive {
    fn get_prefix(&self) -> String {
        "s".to_string()
    }
}

impl TySONPrimitive for StrPrimitive {
    fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

#[derive(Debug)]
pub struct HMap {
    values: HashMap<String, TySONItem>,
}

impl BaseTySONItemInterface for HMap {
    fn get_prefix(&self) -> String {
        "h".to_string()
    }
}

impl TySONMap for HMap {
    fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    fn insert(&mut self, k: Box<dyn TySONPrimitive>, v: TySONItem) {
       let _ =  &self.values.insert(k.get_prefix(), v);
    }
}

#[derive(Debug)]
pub struct Vector {
    values: Vec<TySONItem>,
}

impl BaseTySONItemInterface for Vector {
    fn get_prefix(&self) -> String {
        "v".to_string()
    }
}

impl TySONVector for Vector {
    fn new() -> Self {
        Self {
            values: vec![]
        }
    }

    fn push(&mut self, item: TySONItem) {
        let _ = &self.values.push(item);
    }
}

#[derive(Debug)]
pub struct Doc {
    items: Vec<(Box<dyn TySONPrimitive>, TySONItem)>,
}

impl Desereilize for Doc {
    fn new_document() -> Self {
        Self {
            items: vec![]
        }
    }

    fn add_to_document(&mut self, data: (Box<dyn TySONPrimitive>, TySONItem)) {
        self.items.push(data)
    }

    fn new_vector(_prefix: String) -> Box<dyn TySONVector> {
        Box::new(Vector::new())
    }

    fn new_map(_prefix: String) -> Box<dyn TySONMap> {
        Box::new(HMap::new())
    }

    fn new_primitive(prefix: String, data: String) -> Box<dyn TySONPrimitive> {
        return match prefix.as_str() {
            "s" => { Box::new(StrPrimitive::new(data)) }
            "n" => { Box::new(IntPrimitive::new(data)) }
            _ => unreachable!()
        };
    }
}

impl TySONDocument for Doc {}

// impl SerializeDoc for Doc{
//     fn items(&self) -> Vec<(Box<dyn SerializeItem>, Box<dyn SerializeItem>)> {
//         let mut res: Vec<(Box<dyn SerializeItem>, Box<dyn SerializeItem>)> = vec![];
//         for i in self.items{
//             // let j: Box<dyn SerializeItem> = i.0;
//             // let k: Box<dyn SerializeItem> = i.1;
//             res.push(i)
//         }
//         res
//     }
// }