use std::fmt;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TysonValue {
    Primitive(Primitive),
    Map(String, Vec<(Primitive, TysonValue)>),
    Array(String, Vec<TysonValue>),
}
