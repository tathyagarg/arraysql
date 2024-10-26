use maybe_sql::parser;

#[test]
fn test_database_creation() {
    let mut p = parser::Parser::default();

    p.set_query("DATABASE my_database;".to_string()).parse();

    assert_eq!(
        p.query_data._type,
        parser::query::QueryType::DatabaseCreation
    );
    assert_eq!(p.query_data.db_name, "my_database");
}

#[test]
fn test_lowercase_database_creation() {
    let mut p = parser::Parser::default();
    p.set_query("database my_lowercase_database;".to_string())
        .parse();

    assert_eq!(
        p.query_data._type,
        parser::query::QueryType::DatabaseCreation
    );
    assert_eq!(p.query_data.db_name, "my_lowercase_database");
}

#[test]
#[should_panic]
fn test_failed_database_creation() {
    let mut p = parser::Parser::default();
    p.set_query("database;".to_string()).parse();
}

#[test]
#[should_panic]
fn test_invalid_database_identifier() {
    let mut p = parser::Parser::default();
    p.set_query("database --;".to_string()).parse();
}

#[test]
fn test_database_with_database_in_name() {
    let mut p = parser::Parser::default();
    p.set_query("database database_is_cool;".to_string())
        .parse();

    assert_eq!(p.query_data.db_name, "database_is_cool");
}
