use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Primitive(pub String, pub String);


impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1.is_empty() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}|{}|", self.0, self.1) // TODO escape
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TYSONValue {
    Primitive(Primitive),
    Map(String, Vec<(Primitive, TYSONValue)>),
    Vector(String, Vec<TYSONValue>),
}

#[derive(Debug)]
pub struct TYSONDocument(Vec<(Primitive, TYSONValue)>);

impl TYSONDocument {
    pub fn new() -> Self{
        Self(vec![])
    }

    pub fn push(&mut self, item: (Primitive, TYSONValue)){
        self.0.push(item);
    }

    pub fn items(&self) -> &Vec<(Primitive, TYSONValue)>{
        &self.0
    }
}