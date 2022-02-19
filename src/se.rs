use crate::value::{Primitive, TYSONDocument, TYSONValue};


pub fn serialize_doc(doc: &TYSONDocument) -> String {
    let mut contents: Vec<String> = vec![];

    fn serialize_value(val: &TYSONValue) -> String {
        match val {
            TYSONValue::Map(prefix, data) => {
                let contents: Vec<_> = data
                    .iter()
                    .map(|(left, value)|
                        format!("{}:{}", left, serialize_value(value)))
                    .collect();
                format!("{}{{{}}}", prefix, contents.join(","))
            }
            TYSONValue::Vector(prefix, data) => {
                let contents: Vec<_> = data.iter().map(serialize_value).collect();
                format!("{}[{}]", prefix, contents.join(","))
            }
            TYSONValue::Primitive(data) => {
                format!("{}", data)
            }
        }
    }

    for (key, val) in doc.items() {
        contents.push(format!("{}:{}", key, serialize_value(val)));
    }
    format!("{}", contents.join(";"))
}