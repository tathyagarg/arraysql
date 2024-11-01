use crate::common;
use arraysql::{engine, parser};
use serial_test::serial;

const DB_NAME: &str = "my_database";

#[test]
#[serial]
pub fn test_database_creation() {
    common::soft_remove_db(DB_NAME).unwrap();

    let mut p = parser::Parser::new();
    p.set_query(format!("DATABASE {};", DB_NAME)).parse();

    let res = engine::engine(&p);
    if let Err(err) = res {
        panic!("Err: {}", err);
    }
    assert!(res.is_ok());

    assert!(common::ensure_db_existance(DB_NAME));

    //  Post test
    let res = common::remove_db(DB_NAME);
    assert!(res.is_ok());
}

#[test]
#[serial]
pub fn test_database_already_exists() {
    common::soft_remove_db(DB_NAME).unwrap();

    let mut p = parser::Parser::new();
    p.set_query(format!("DATABASE {};", DB_NAME)).parse();

    let res = engine::engine(&p);
    assert!(res.is_ok());

    assert!(common::ensure_db_existance(DB_NAME));

    //  Trying to create the database again
    let res = engine::engine(&p);
    assert!(res.is_err());

    //  Post test
    let res = common::remove_db(DB_NAME);
    assert!(res.is_ok());
}
