use std::fmt::Debug;

pub trait TySONItem: Debug {
    fn get_prefix(&self) -> String;

    fn as_item(&self) -> Box<dyn TySONItem>{
        Box::new(self)
    }
}