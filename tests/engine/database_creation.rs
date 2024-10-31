use crate::common;
use arraysql::{engine, parser};

#[test]
pub fn test_database_creation() {
    const DB_NAME: &str = "my_database";
    common::soft_remove_db(DB_NAME).unwrap();

    let mut p = parser::Parser::new();
    p.set_query(format!("DATABASE {};", DB_NAME)).parse();

    engine::engine(&p).unwrap();
    assert!(common::ensure_db_existance(DB_NAME));

    //  Post test
    common::remove_db(DB_NAME).unwrap();
}

#[test]
pub fn test_database_already_exists() {
    const DB_NAME: &str = "my_database";
    common::soft_remove_db(DB_NAME).unwrap();

    let mut p = parser::Parser::new();
    p.set_query(format!("DATABASE {};", DB_NAME)).parse();

    engine::engine(&p).unwrap();
    assert!(common::ensure_db_existance(DB_NAME));

    //  Trying to create the database again
    engine::engine(&p).unwrap_err();

    //  Post test
    common::remove_db(DB_NAME).unwrap();
}
