use std::fmt;
use crate::deserialize::item::{TySONItem, BaseTySONItemInterface};
use crate::serialize::se::SerializeItem;

pub trait TySONPrimitive: BaseTySONItemInterface {
    fn new(value: String) -> Self
        where
            Self: Sized;

    // fn get_type(&self) -> TySONItem {
    //     TySONItem::Primitive
    // }

    // fn get_string_value(&self) -> String;
}

// impl <T: TySONPrimitive> SerializeItem for T {
//     fn serialize(&self) -> String {
//         let prefix = self.get_prefix();
//         if prefix == "".to_string() {
//             format!("{}", "1234".to_string())
//         } else {
//             format!("{}|{}|", prefix, "1234".to_string()) // TODO escape
//         }
//     }
// }

