use std::collections::BTreeMap;
use std::hash::Hash;
use crate::de::Desereilize;
use crate::document::TySONDocument;
use crate::item::{BaseTySONItemInterface, TySONItem};
use crate::map::TySONMap;
use crate::primitive::TySONPrimitive;
use crate::se::Serialize;
use crate::vector::TySONVector;



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


impl TySONPrimitive for IntPrimitive {
    fn new(value: String) -> Self {
        let num: i64 = value.parse().unwrap();
        Self {
            value: num
        }
    }

    fn get_string_value(&self) -> String {
        self.value.to_string()
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

    fn get_string_value(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct TestMap {
    values: BTreeMap<String, TySONItem>,
}

impl BaseTySONItemInterface for TestMap {
    fn get_prefix(&self) -> String {
        "h".to_string()
    }
}

impl TySONMap for TestMap {
    fn new() -> Self {
        Self {
            values: BTreeMap::new()
        }
    }

    fn insert(&mut self, k: Box<dyn TySONPrimitive>, v: TySONItem) {
        let _ = &self.values.insert(k.get_string_value(), v);
    }

    fn get_items(&self) -> Vec<(Box<dyn TySONPrimitive>, &TySONItem)> {
        let mut res: Vec<(Box<dyn TySONPrimitive>, &TySONItem)> = vec![];
        for (k, v) in &self.values {
            res.push((Box::new(StrPrimitive::new(k.to_string())), v))
        }
        res
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

    fn get_items(&self) -> Vec<&TySONItem> {
        let mut res: Vec<&TySONItem> = vec![];
        for value in &self.values {
            res.push(value)
        }
        res
    }
}

#[derive(Debug)]
pub struct Doc {
    items: Vec<(StrPrimitive, TySONItem)>,
}

impl Desereilize for Doc {
    fn new_document() -> Self {
        Self {
            items: vec![]
        }
    }

    fn add_to_document(&mut self, data: (Box<dyn TySONPrimitive>, TySONItem)) {
        self.items.push((StrPrimitive::new(data.0.get_string_value()), data.1))
    }

    fn new_vector(_prefix: String) -> Box<dyn TySONVector> {
        Box::new(Vector::new())
    }

    fn new_map(_prefix: String) -> Box<dyn TySONMap> {
        Box::new(TestMap::new())
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

impl Serialize for Doc {
    fn items(&self) -> Vec<(&dyn TySONPrimitive, &TySONItem)> {
        let mut res: Vec<(&dyn TySONPrimitive, &TySONItem)> = vec![];
        for (k, v) in &self.items {
            let key = k as &dyn TySONPrimitive;
            res.push((key, v))
        }
        res
    }
}
