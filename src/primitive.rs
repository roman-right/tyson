use crate::item::TySONItem;

pub trait TySONPrimitive: TySONItem {
    fn new(value: String) -> Self
        where
            Self: Sized;
}
