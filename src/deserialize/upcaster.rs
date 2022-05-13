use crate::deserialize::item::BaseTySONItemInterface;

pub trait Upcaster {
    fn as_upcaster<'a>(self: Box<Self>) -> Box<dyn BaseTySONItemInterface + 'a>
        where
            Self: 'a;
}

impl<T: BaseTySONItemInterface + Sized> Upcaster for T {
    fn as_upcaster<'a>(self: Box<Self>) -> Box<dyn BaseTySONItemInterface + 'a>
        where
            Self: 'a,
    {
        self
    }
}