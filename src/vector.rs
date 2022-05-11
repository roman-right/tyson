use crate::item::TySONItem;

pub trait TySONVector:TySONItem {
    fn new() -> Self
    where
        Self: Sized;

    fn push(&mut self, item: Box<dyn TySONItem>);

}