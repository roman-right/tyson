use crate::deserialize::item::{TySONItem, BaseTySONItemInterface};
use crate::serialize::se::SerializeItem;

pub trait TySONVector: BaseTySONItemInterface {
    fn new() -> Self
        where
            Self: Sized;

    fn push(&mut self, item: TySONItem);

    // fn get_items(&self) -> Vec<Box<dyn TySONItem>>;
}

// impl SerializeItem for Box<dyn TySONVector> {
//     fn serialize(&self) -> String {
//         let prefix = self.get_prefix();
//         let mut contents: Vec<String> = vec![];
//         // for i in self.get_items() {
//         //     contents.push(i.serialize())
//         // }
//         format!("{}[{}]", prefix, contents.join(","))
//     }
// }