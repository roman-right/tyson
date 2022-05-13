use crate::item::BaseTySONItemInterface;

pub trait TySONPrimitive: BaseTySONItemInterface {
    fn new(value: String) -> Self
        where
            Self: Sized;

    fn get_string_value(&self) -> String;

    fn serialize(&self) -> String {
        let prefix = self.get_prefix();
        if prefix == "".to_string() {
            format!("{}", self.get_string_value())
        } else {
            format!("{}|{}|", prefix, self.get_string_value()) // TODO escape
        }
    }
}
