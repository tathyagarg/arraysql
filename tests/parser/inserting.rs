use arraysql::parser::{
    self,
    query::{Identifier, QueryType},
};

#[test]
fn test_basic_insert() {
    let mut p = parser::Parser::new();
    p.set_query(
        "INSERT STRUCTURED ('my_data') ON my_table STRUCTURED (field1) ON my_database;".to_string(),
    )
    .parse();

    assert_eq!(p.query_data._type, QueryType::Insert);
    assert_eq!(
        p.query_data.table_name,
        Identifier::StringLiteral("my_table".to_string())
    );
    assert_eq!(
        p.query_data.db_name,
        Identifier::StringLiteral("my_database".to_string())
    );
    assert_eq!(
        p.query_data.inserted_value,
        vec![Identifier::StringLiteral("'my_data'".to_string())]
    );
    assert_eq!(
        p.query_data.inserted_field,
        vec![Identifier::Field("field1".to_string())]
    );
}

#[test]
fn test_multiple_fields() {
    let mut p = parser::Parser::new();
    p.set_query(
        "INSERT STRUCTURED ('Tathya', 15, ['Rust', 'Python']) ON my_table STRUCTURED (name, age, known_languages) ON my_database;"
            .to_string(),
    )
    .parse();

    println!("{:?}", p.query_data);

    assert_eq!(
        p.query_data.inserted_value,
        vec![
            Identifier::StringLiteral("'Tathya'".to_string()),
            Identifier::IntLiteral(15),
            Identifier::Array(vec![
                Identifier::StringLiteral("'Rust'".to_string()),
                Identifier::StringLiteral("'Python'".to_string())
            ])
        ]
    );
}
