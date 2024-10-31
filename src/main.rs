#![allow(dead_code)]

use arraysql::{engine, parser};

fn main() {
    let mut p = parser::Parser::new();
    p.set_query("DATABASE my_database;".to_string()).parse();
    let _ = engine::engine(&p);

    p.reset();
    p.set_query("TABLE users ON my_database STRUCTURED (UINT(1) id);".to_string())
        .parse();
    let _ = engine::engine(&p);
}
