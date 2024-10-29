#![allow(dead_code)]

use arraysql::{engine, parser};

fn main() {
    let mut p = parser::Parser::new();
    p.set_query("DATABASE my_database;".to_string()).parse();

    engine::engine(&p);
}
