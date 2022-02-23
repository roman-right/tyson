use crate::item::TySONItem;
use crate::primitive::TySONPrimitive;

pub trait TySONMap:TySONItem{
    fn new(prefix: String) -> Self;

    fn add(&mut self, data: (Box<&dyn TySONPrimitive>, Box<&dyn TySONItem>));
}