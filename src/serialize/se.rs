use crate::deserialize::item::{TySONItem, BaseTySONItemInterface};
use crate::deserialize::map::TySONMap;
use crate::deserialize::primitive::TySONPrimitive;
use crate::deserialize::vector::TySONVector;

pub trait SerializeItem {
    fn serialize(&self) -> String;
}

pub trait SerializeDoc {
    fn items(&self) -> Vec<(Box<dyn SerializeItem>, Box<dyn SerializeItem>)>;

    fn serialize(&self) -> String {

        let mut contents: Vec<String> = vec![];

        for (k, v) in self.items() {
            contents.push(format!("{}:{}", k.serialize(), v.serialize()));
        };
        format!("{}", contents.join(";"))
    }
}