use std::collections::HashMap;
use std::hash::Hash;
use crate::de::Desereilize;
use crate::document::TySONDocument;
use crate::item::TySONItem;
use crate::map::TySONMap;
use crate::primitive::TySONPrimitive;
use crate::vector::TySONVector;


pub trait HashablePrimitive: Hash + PartialEq + TySONPrimitive {}

#[derive(Debug, Hash, PartialEq)]
pub struct IntPrimitive {
    value: i64,
}

#[derive(Debug, Hash, PartialEq)]
pub struct StrPrimitive {
    value: String,
}

impl TySONItem for IntPrimitive {
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

impl TySONItem for StrPrimitive {
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
    values: HashMap<String, Box<dyn TySONItem>>,
}

impl TySONItem for HMap {
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

    fn insert(&mut self, k: Box<dyn TySONPrimitive>, v: Box<dyn TySONItem>) {
       let _ =  &self.values.insert(k.get_prefix(), v);
    }
}

#[derive(Debug)]
pub struct Vector {
    values: Vec<Box<dyn TySONItem>>,
}

impl TySONItem for Vector {
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

    fn push(&mut self, item: Box<dyn TySONItem>) {
        let _ = &self.values.push(item);
    }
}

#[derive(Debug)]
pub struct Doc {
    items: Vec<(Box<dyn TySONPrimitive>, Box<dyn TySONItem>)>,
}

impl Desereilize for Doc {
    fn new_document() -> Self {
        Self {
            items: vec![]
        }
    }

    fn add_to_document(&mut self, data: (Box<dyn TySONPrimitive>, Box<dyn TySONItem>)) {
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


// use std::fmt::Debug;
// use crate::upcaster::Upcaster;
//
// pub trait Super: AsDynSuper {}
//
//
//
// impl<T: Super + Sized> AsDynSuper for T {
//     fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
//         where
//             Self: 'a,
//     {
//         self
//     }
// }
//
// pub trait Sub: Super {}
//
// pub fn upcast(obj: Box<dyn Sub>) -> Box<dyn Super> {
//     obj.as_dyn_super()
// }

// #[derive(Debug)]
// struct TestStruct {
//     s: String,
// }
//
// impl Super for TestStruct {}
//
// // impl AsDynSuper for TestStruct {
// //     fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a> where Self: 'a {
// //         todo!()
// //     }
// // }
//
// impl Sub for TestStruct {}
//
// pub fn create_test_struct() -> Box<dyn Sub>{
//     let s = "test".to_string();
//     let t = TestStruct{
//         s
//     };
//     Box::new(t)
// }
//
// pub fn print_s(t: Box<dyn Super>){
//     print!("{:?}", t);
// }