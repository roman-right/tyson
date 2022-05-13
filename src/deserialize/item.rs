use std::fmt::{Debug, Display};
use crate::deserialize::map::TySONMap;
use crate::deserialize::primitive::TySONPrimitive;
use crate::serialize::se::SerializeItem;
use crate::deserialize::upcaster::Upcaster;
use crate::deserialize::vector::TySONVector;

#[derive(Debug)]
pub enum TySONItem {
    Primitive(Box<dyn TySONPrimitive>),
    Vector(Box<dyn TySONVector>),
    Map(Box<dyn TySONMap>),
}

pub trait BaseTySONItemInterface:Debug {
    fn get_prefix(&self) -> String;
}
