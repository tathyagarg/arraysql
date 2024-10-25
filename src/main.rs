#![allow(dead_code)]

mod parser;

fn main() {
    let mut p = parser::Parser::default();
    p.set_query(String::from(
        "TABLE sigma ON dbb STRUCTURED (UINT(skibidi) skibidi) CONSTRAINED (ON skibidi UNIQUE, ON rizz UNIQUE PKEY INC DEFAULT(AUTO)) MODE FREAD FDELETE;",
    ));
    p.parse();

    println!("{:?}", p);
}
