use crate::common;
use arraysql::{engine, parser};

#[test]
pub fn test_database_creation() {
    let mut p = parser::Parser::new();
    p.set_query("DATABASE my_database;".to_string()).parse();

    engine::engine(&p).unwrap();
    assert!(common::ensure_db_existance("my_database"));

    //  Post test
    common::remove_db("my_database").unwrap();
}

#[test]
pub fn test_database_already_exists() {
    let mut p = parser::Parser::new();
    p.set_query("DATABASE my_database;".to_string()).parse();

    engine::engine(&p).unwrap();
    assert!(common::ensure_db_existance("my_database"));

    //  Trying to create the database again
    engine::engine(&p).unwrap_err();

    //  Post test
    common::remove_db("my_database").unwrap();
}
