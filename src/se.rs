use crate::item::TySONItem;
use crate::primitive::TySONPrimitive;

pub trait Serialize {
    fn items(&self) -> Vec<(&dyn TySONPrimitive, &TySONItem)>;

    fn serialize(&self) -> String {
        let mut contents: Vec<String> = vec![];

        for (k, v) in self.items() {
            contents.push(format!("{}:{}", k.serialize(), v.serialize()));
        };
        format!("{}", contents.join(";"))
    }
}