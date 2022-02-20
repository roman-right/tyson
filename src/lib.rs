pub mod de;
pub mod se;
pub mod value;
pub mod errors;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::de::deserialize;
use crate::se::serialize;

#[cfg(test)]
mod tests {
    use crate::de::deserialize;
    use crate::se::serialize;
    use crate::value::Primitive;
    use crate::value::TySONValue;

    #[test]
    fn de_se() {
        let data = r#"
        l|123|: hash{|s|: s|100|; a:s|100|};
        l|124|: hash{c|1|: s|100|, b|2|:s|100|},
        l|123|: ll[d|1|, e|2|],
        "#;

        let des = deserialize(data.to_string()).unwrap();
        println!("{:?}", des.items());
        assert_eq!(*des.items(), vec![(Primitive("l".to_string(), "123".to_string()),
                                       TySONValue::Map("hash".to_string(),
                                                       vec![(Primitive("".to_string(), "s".to_string()),
                                                             TySONValue::Primitive(Primitive("s".to_string(), "100".to_string()))),
                                                            (Primitive("a".to_string(), "".to_string()),
                                                             TySONValue::Primitive(Primitive("s".to_string(), "100".to_string())))])),
                                      (Primitive("l".to_string(), "124".to_string()),
                                       TySONValue::Map("hash".to_string(), vec![(Primitive("c".to_string(), "1".to_string()),
                                                                                 TySONValue::Primitive(Primitive("s".to_string(), "100".to_string()))),
                                                                                (Primitive("b".to_string(), "2".to_string()),
                                                                                 TySONValue::Primitive(Primitive("s".to_string(), "100".to_string())))])),
                                      (Primitive("l".to_string(), "123".to_string()),
                                       TySONValue::Vector("ll".to_string(), vec![TySONValue::Primitive(Primitive("d".to_string(), "1".to_string())),
                                                                                 TySONValue::Primitive(Primitive("e".to_string(), "2".to_string()))]))]);
        assert_eq!(serialize(&des), "l|123|:hash{|s|:s|100|,a:s|100|};l|124|:hash{c|1|:s|100|,b|2|:s|100|};l|123|:ll[d|1|,e|2|]".to_string())
    }
}
