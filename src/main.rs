#![allow(dead_code)]

mod parser;

fn main() {
    let mut p = parser::Parser::default();
    p.set_query(String::from(
        "INSERT STRUCTURED ('I love turtles', 3, 12, 'skibidi sigma') ON skibidi_table STRUCTURED (f1, f2, f3, f4) ON my_database;",
    ));
    p.parse();

    println!("{:?}", p);
}
