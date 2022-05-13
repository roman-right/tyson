use crate::deserialize::item::{TySONItem, BaseTySONItemInterface};
use crate::deserialize::primitive::TySONPrimitive;
use crate::serialize::se::SerializeItem;

pub trait TySONMap: BaseTySONItemInterface {
    fn new() -> Self
        where
            Self: Sized;

    fn insert(&mut self, k: Box<dyn TySONPrimitive>, v: TySONItem);

    // fn get_type(&self) -> TySONItem {
    //     TySONItem::Map
    // }

    // fn get_items(&self) -> Vec<(Box<dyn TySONPrimitive>, Box<dyn TySONItem>)>;
}

// impl SerializeItem for Box<dyn TySONMap> {
//     fn serialize(&self) -> String {
//         let mut contents: Vec<String> = vec![];
//         // for (k,v) in self.get_items(){
//         //     let s = format!("{}:{}", k.serialize(), v.serialize());
//         //     contents.push(s);
//         // }
//         format!("{}{{{}}}", self.get_prefix(), contents.join(","))
//     }
// }