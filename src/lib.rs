pub mod de;
pub mod se;
pub mod value;
pub mod errors;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
mod tests {
    use pest::iterators::Pair;
    use crate::de::Desereilize;
    use crate::errors::TySONError;
    use crate::se::serialize;
    use crate::value::{Primitive, TySONDocument};
    use crate::value::TySONValue;

    #[test]
    fn de_se() {
        let data = r#"
        l|123|: hash{|s|: s|100|; a:s|100|};
        l|124|: hash{c|1|: s|100|, b|2|:s|100|},
        l|123|: ll[d|1|, e|2|],
        "#;



        #[derive(Debug)]
        struct Doc{
            items: Vec<(Primitive, TySONValue)>
        }

        impl Desereilize<TySONValue, Primitive> for Doc{


            fn from_map(prefix: String, pairs: Vec<(Primitive, TySONValue)>) -> TySONValue {
                TySONValue::Map(prefix, pairs)
            }

            fn from_vector(prefix: String, data: Vec<TySONValue>) -> TySONValue {
                TySONValue::Vector(prefix, data)
            }

            fn from_primitive(val: Primitive) -> Primitive {
                val
            }

            fn wrap_primitive(p: Primitive) -> TySONValue {
                TySONValue::Primitive(p)
            }

            fn new() -> Self {
                Self {
                    items: vec![]
                }
            }

            fn push(&mut self, pair: (Primitive, TySONValue)) {
                self.items.push(pair);
            }
        }

        let doc = Doc::deserialize(data.to_string()).unwrap();
        print!("{:?}", doc);

        // let des = deserialize(data.to_string()).unwrap();
        // println!("{:?}", des.items());
        // assert_eq!(*des.items(), vec![(Primitive("l".to_string(), "123".to_string()),
        //                                TySONValue::Map("hash".to_string(),
        //                                                vec![(Primitive("".to_string(), "s".to_string()),
        //                                                      TySONValue::Primitive(Primitive("s".to_string(), "100".to_string()))),
        //                                                     (Primitive("a".to_string(), "".to_string()),
        //                                                      TySONValue::Primitive(Primitive("s".to_string(), "100".to_string())))])),
        //                               (Primitive("l".to_string(), "124".to_string()),
        //                                TySONValue::Map("hash".to_string(), vec![(Primitive("c".to_string(), "1".to_string()),
        //                                                                          TySONValue::Primitive(Primitive("s".to_string(), "100".to_string()))),
        //                                                                         (Primitive("b".to_string(), "2".to_string()),
        //                                                                          TySONValue::Primitive(Primitive("s".to_string(), "100".to_string())))])),
        //                               (Primitive("l".to_string(), "123".to_string()),
        //                                TySONValue::Vector("ll".to_string(), vec![TySONValue::Primitive(Primitive("d".to_string(), "1".to_string())),
        //                                                                          TySONValue::Primitive(Primitive("e".to_string(), "2".to_string()))]))]);
        // assert_eq!(serialize(&des), "l|123|:hash{|s|:s|100|,a:s|100|};l|124|:hash{c|1|:s|100|,b|2|:s|100|};l|123|:ll[d|1|,e|2|]".to_string())
    }
}
