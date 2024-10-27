use maybe_sql::parser::{
    self,
    query::{BinaryOperation, Expression, Identifier, QueryType},
};

#[test]
fn test_basic_read() {
    let mut p = parser::Parser::new();
    p.set_query("READ STRUCTURED (name) ON users ON my_database;".to_string())
        .parse();

    assert_eq!(p.query_data._type, QueryType::Read);
    assert_eq!(
        p.query_data.read_fields,
        vec![Identifier::Field("name".to_string())]
    );
    assert_eq!(
        p.query_data.table_name,
        Identifier::StringLiteral("users".to_string())
    );
    assert_eq!(
        p.query_data.db_name,
        Identifier::StringLiteral("my_database".to_string())
    );
}

#[test]
fn test_multiple_field() {
    let mut p = parser::Parser::new();
    p.set_query(
        "READ STRUCTURED (name, age, known_languages) ON users ON my_database;".to_string(),
    )
    .parse();

    assert_eq!(
        p.query_data.read_fields,
        vec![
            Identifier::Field("name".to_string()),
            Identifier::Field("age".to_string()),
            Identifier::Field("known_languages".to_string())
        ]
    );
}

#[test]
fn test_where() {
    let mut p = parser::Parser::new();
    p.set_query(
        "READ STRUCTURED (name, age, known_languages) ON users ON my_database WHERE age >= 13;"
            .to_string(),
    )
    .parse();

    assert_eq!(
        p.query_data.conditions,
        Expression::Binary(
            BinaryOperation::GreaterThanEqualTo,
            Box::new((
                Expression::Identifier(Identifier::Field("age".to_string())),
                Expression::Identifier(Identifier::IntLiteral(13))
            ))
        )
    );
}
