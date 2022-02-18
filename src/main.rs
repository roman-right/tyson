mod de;
mod se;
mod value;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::de::deserialize;
use crate::se::serialize;

fn main() {
    let data = r#"
    l|123|: hash{|s|: s|100|; a|2|:s|100|};
    l|123|: hash{c|1|: s|100|, b|2|:s|100|},
    l|123|: ll[d|1|, e|2|],
    "#;

    let res = deserialize(data.to_string()).unwrap();
    println!("{:?}", &res);
    for i in &res{
        println!("{:?}", serialize(&i.1));
    }

}