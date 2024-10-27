use maybe_sql::parser::{self, query::QueryType};

#[test]
fn test_basic_read() {
    let mut p = parser::Parser::default();
    p.set_query("READ STRUCTURED (name) ON users ON my_database;".to_string())
        .parse();

    assert_eq!(p.query_data._type, QueryType::Read);
    assert_eq!(p.query_data.read_fields, vec!["name"]);
    assert_eq!(p.query_data.table_name, "users");
    assert_eq!(p.query_data.db_name, "my_database");
}

#[test]
fn test_multiple_field() {
    let mut p = parser::Parser::default();
    p.set_query(
        "READ STRUCTURED (name, age, known_languages) ON users ON my_database;".to_string(),
    )
    .parse();

    assert_eq!(
        p.query_data.read_fields,
        vec!["name", "age", "known_languages"]
    );
}

#[test]
fn test_where() {
    let mut p = parser::Parser::default();
    p.set_query(
        "READ STRUCTURED (name, age, known_languages) ON users ON my_database WHERE age >= 13;"
            .to_string(),
    )
    .parse();

    assert_eq!(p.query_data.conditions, vec!["age", ">=", "13"]);
}
