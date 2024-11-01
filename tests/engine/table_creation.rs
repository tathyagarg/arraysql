use crate::common::{self, remove_db};
use serial_test::serial;

const DATABASE_NAME: &str = "my_database";
const TABLE_NAME: &str = "users";

fn pretest(p: &mut arraysql::parser::Parser) -> Result<(), std::io::Error> {
    println!("Soft removing");
    common::soft_remove_db(DATABASE_NAME)?;
    println!("Soft removal successful");

    p.set_query(format!("DATABASE {};", DATABASE_NAME)).parse();
    println!("Parsing");
    let res = arraysql::engine::engine(p);
    if let Err(err) = res {
        panic!("Err: {}", err);
    }

    println!("Parsing successful");

    p.reset();

    Ok(())
}

#[test]
#[serial]
pub fn test_table_creation() {
    let mut p = arraysql::parser::Parser::new();
    let res = pretest(&mut p);
    assert!(res.is_ok());

    p.set_query(format!(
        "TABLE {} ON {} STRUCTURED (UINT(1) id);",
        TABLE_NAME, DATABASE_NAME
    ))
    .parse();

    let res = arraysql::engine::engine(&p);
    if let Err(err) = res {
        panic!("Err: {}", err);
    }
    assert!(res.is_ok());

    assert!(common::ensure_table_existance(DATABASE_NAME, TABLE_NAME));

    let res = remove_db(DATABASE_NAME);
    assert!(res.is_ok());
}
