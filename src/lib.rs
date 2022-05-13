extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod errors;
mod example;
pub mod primitive;
pub mod map;
pub mod item;
pub mod vector;
pub mod document;
pub mod de;
pub mod se;

#[cfg(test)]
mod tests {

    use crate::de::Desereilize;
    use crate::example::Doc;
    use crate::se::Serialize;

    #[test]
    fn de_se() {
        let data = r#"
        s|123|: h{s|s|: s|100|; s|p|:s|100|};
        s|124|: l[s|100|; s|p|];
        s|125|: h{s|s|: s|100|, s|p|:s|100|};
        s|123|: h{s|s|: s|100|; s|p|:s|100|};
        "#;

        let doc = Doc::deserialize(data.to_string()).unwrap();

        assert_eq!(doc.serialize(), "s|123|:h{s|p|:s|100|,s|s|:s|100|};s|124|:v[s|100|,s|p|];s|125|:h{s|p|:s|100|,s|s|:s|100|};s|123|:h{s|p|:s|100|,s|s|:s|100|}".to_string())
    }
}
