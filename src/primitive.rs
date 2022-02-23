// use std::fmt;
//
// #[derive(Debug, Eq, PartialEq)]
// pub struct Primitive(pub String, pub String);
//
//
// impl fmt::Display for Primitive {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.1.is_empty() {
//             write!(f, "{}", self.0)
//         } else {
//             write!(f, "{}|{}|", self.0, self.1) // TODO escape
//         }
//     }
// }
//
//
//
//


use crate::item::TySONItem;

pub trait TySONPrimitive:TySONItem{
    fn new(prefix: String, value: String) -> Self;
}