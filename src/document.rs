use crate::item::TySONItem;
use crate::primitive::TySONPrimitive;

pub struct TySONDocument(Vec<(Box<dyn TySONPrimitive>, Box<dyn TySONItem>)>);