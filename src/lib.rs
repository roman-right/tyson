mod de;
mod se;
mod value;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
mod tests {
    use crate::de::deserialize_doc;
    use crate::se::serialize_doc;
    use crate::value::Primitive;
    use crate::value::TYSONValue;

    #[test]
    fn de_se() {
        let data = r#"
        l|123|: hash{|s|: s|100|; a:s|100|};
        l|124|: hash{c|1|: s|100|, b|2|:s|100|},
        l|123|: ll[d|1|, e|2|],
        "#;

        let des = deserialize_doc(data.to_string()).unwrap();
        println!("{:?}", des.items());
        assert_eq!(*des.items(), vec![(Primitive("l".to_string(), "123".to_string()),
                                       TYSONValue::Map("hash".to_string(),
                                                       vec![(Primitive("".to_string(), "s".to_string()),
                                                             TYSONValue::Primitive(Primitive("s".to_string(), "100".to_string()))),
                                                            (Primitive("a".to_string(), "".to_string()),
                                                             TYSONValue::Primitive(Primitive("s".to_string(), "100".to_string())))])),
                                      (Primitive("l".to_string(), "124".to_string()),
                                       TYSONValue::Map("hash".to_string(), vec![(Primitive("c".to_string(), "1".to_string()),
                                                                                 TYSONValue::Primitive(Primitive("s".to_string(), "100".to_string()))),
                                                                                (Primitive("b".to_string(), "2".to_string()),
                                                                                 TYSONValue::Primitive(Primitive("s".to_string(), "100".to_string())))])),
                                      (Primitive("l".to_string(), "123".to_string()),
                                       TYSONValue::Vector("ll".to_string(), vec![TYSONValue::Primitive(Primitive("d".to_string(), "1".to_string())),
                                                                                 TYSONValue::Primitive(Primitive("e".to_string(), "2".to_string()))]))]);
        assert_eq!(serialize_doc(&des), "l|123|:hash{|s|:s|100|,a:s|100|};l|124|:hash{c|1|:s|100|,b|2|:s|100|};l|123|:ll[d|1|,e|2|]".to_string())
    }
}
