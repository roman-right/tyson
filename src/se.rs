use crate::value::{Primitive, TysonValue};

pub fn serialize(val: &TysonValue) -> String {
    match val {
        TysonValue::Map(prefix, data) => {
            let contents: Vec<_> = data
                .iter()
                .map(|(left, value)|
                    format!("{}:{}", left, serialize(value)))
                .collect();
            format!("{}{{{}}}", prefix, contents.join(","))
        }
        TysonValue::Array(prefix, data) => {
            let contents: Vec<_> = data.iter().map(serialize).collect();
            format!("{}[{}]", prefix, contents.join(","))
        }
        TysonValue::Primitive(data) => {
            format!("{}", data)
        }
    }
}