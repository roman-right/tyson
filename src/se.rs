use crate::value::{TySONDocument, TySONValue};


pub fn serialize(doc: &TySONDocument) -> String {
    let mut contents: Vec<String> = vec![];

    fn serialize_value(val: &TySONValue) -> String {
        match val {
            TySONValue::Map(prefix, data) => {
                let contents: Vec<_> = data
                    .iter()
                    .map(|(left, value)|
                        format!("{}:{}", left, serialize_value(value)))
                    .collect();
                format!("{}{{{}}}", prefix, contents.join(","))
            }
            TySONValue::Vector(prefix, data) => {
                let contents: Vec<_> = data.iter().map(serialize_value).collect();
                format!("{}[{}]", prefix, contents.join(","))
            }
            TySONValue::Primitive(data) => {
                format!("{}", data)
            }
        }
    }

    for (key, val) in doc.items() {
        contents.push(format!("{}:{}", key, serialize_value(val)));
    }
    format!("{}", contents.join(";"))
}