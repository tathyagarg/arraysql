#![allow(dead_code)]

mod parser;

fn main() {
    let mut p = parser::Parser::default();
    p.set_query(String::from(
        "INSERT STRUCTURED ('I love turtles', 3, 12, 'skibidi sigma')",
    ));
    p.parse();

    println!("{:?}", p);
}
