use crate::item::TySONItem;
use crate::primitive::TySONPrimitive;

pub trait TySONMap:TySONItem {
    fn new() -> Self
    where
        Self: Sized;

    fn insert(&mut self, k: Box<dyn TySONPrimitive>, v: Box<dyn TySONItem>);

}