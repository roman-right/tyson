use crate::item::TySONItem;

pub trait Upcaster {
    fn as_upcaster<'a>(self: Box<Self>) -> Box<dyn TySONItem + 'a>
        where
            Self: 'a;
}

impl<T: TySONItem + Sized> Upcaster for T {
    fn as_upcaster<'a>(self: Box<Self>) -> Box<dyn TySONItem + 'a>
        where
            Self: 'a,
    {
        self
    }
}