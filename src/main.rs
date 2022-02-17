mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::parse_doc;

fn main() {
    let data = r#"
    l|123|: hash{|s|: s|100|; n|2|:s|100|};
    l|123|: hash{n|1|: s|100|, n|2|:s|100|},
    l|123|: ll[n|1|, n|2|],
    l|123|: ll[n|1|; n|2|],
    l|123|: true;
    "#;

    let res = parse_doc(data.to_string());
    println!("{:?}", res)
}