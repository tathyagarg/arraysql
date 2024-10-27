#![allow(dead_code)]

mod parser;

fn main() {
    let mut p = parser::Parser::new();
    p.set_query(String::from(
        "READ STRUCTURED (my_name, my_age) ON my_table ON my_database WHERE my_age >= 13;",
    ));
    p.parse();

    println!("{:?}", p);
}
