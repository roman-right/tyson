use std::fmt::Debug;
use crate::upcaster::Upcaster;

pub trait TySONItem: Debug+Upcaster {
    fn get_prefix(&self) -> String;
}
