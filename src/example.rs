use crate::de::Desereilize;
use crate::primitive::Primitive;

#[derive(Debug)]
pub struct Doc {
    items: Vec<(Primitive, Value)>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    WrappedPrimitive(Primitive),
    Map(String, Vec<(Primitive, Value)>),
    Vector(String, Vec<Value>),
}

impl Desereilize<Value, Primitive> for Doc {
    fn new_document() -> Self {
        Self {
            items: vec![]
        }
    }

    fn add_to_document(&mut self, pair: (Primitive, Value)) {
        self.items.push(pair);
    }

    fn new_map(prefix: String) -> Value {
        Value::Map(prefix, vec![])
    }

    fn add_to_map(map: &mut Value, data: (Primitive, Value)) {
        if let Value::Map(_, container) = map {
            container.push(data)
        }
    }


    fn new_vector(prefix: String) -> Value {
        Value::Vector(prefix, vec![])
    }

    fn add_to_vector(vector: &mut Value, data: Value) {
        if let Value::Vector(_, container) = vector {
            container.push(data)
        }
    }


    fn new_primitive(prefix: String, val: String) -> Primitive {
        Primitive(prefix, val)
    }

    fn wrap_primitive(p: Primitive) -> Value {
        Value::WrappedPrimitive(p)
    }
}