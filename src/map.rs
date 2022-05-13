use crate::item::{BaseTySONItemInterface, TySONItem};
use crate::primitive::TySONPrimitive;

pub trait TySONMap: BaseTySONItemInterface {
    fn new() -> Self
        where
            Self: Sized;

    fn insert(&mut self, k: Box<dyn TySONPrimitive>, v: TySONItem);

    fn get_items(&self) -> Vec<(Box<dyn TySONPrimitive>, &TySONItem)>;

    fn serialize(&self) -> String {
        let mut contents: Vec<String> = vec![];
        for (k, v) in self.get_items() {
            let s = format!("{}:{}", k.serialize(), v.serialize());
            contents.push(s);
        }
        format!("{}{{{}}}", self.get_prefix(), contents.join(","))
    }
}
