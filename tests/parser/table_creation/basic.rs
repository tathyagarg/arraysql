use arraysql::parser::{self, query::Identifier};

#[test]
fn test_table_creation() {
    let mut p = parser::Parser::new();

    p.set_query("TABLE my_table ON my_database STRUCTURED (UINT(1) id);".to_string())
        .parse();

    assert_eq!(p.query_data._type, parser::query::QueryType::TableCreation);
    assert_eq!(
        p.query_data.db_name,
        Identifier::StringLiteral("my_database".to_string())
    );
    assert_eq!(
        p.query_data.table_name,
        Identifier::StringLiteral("my_table".to_string())
    );
}

#[test]
fn test_table_field_data() {
    let mut p = parser::Parser::new();

    p.set_query("TABLE my_table ON my_database STRUCTURED (UINT(1) id);".to_string())
        .parse();

    let field_data = &p.query_data.fields[0];
    let (datatype, options, identifier) = field_data;

    assert_eq!(datatype, &Identifier::Datatype("UINT".to_string()));
    assert_eq!(options, &vec![Identifier::IntLiteral(1)]);
    assert_eq!(identifier, &Identifier::Field("id".to_string()));
}

#[test]
fn test_multiple_fields() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT(1) id, STRING(64) name);".to_string(),
    )
    .parse();

    // let expected = [("UINT", vec!["1"], "id"), ("STRING", vec!["64"], "name")];
    let expected = [
        (
            Identifier::Datatype("UINT".to_string()),
            vec![Identifier::IntLiteral(1)],
            Identifier::Field("id".to_string()),
        ),
        (
            Identifier::Datatype("STRING".to_string()),
            vec![Identifier::IntLiteral(64)],
            Identifier::Field("name".to_string()),
        ),
    ];

    for (i, (datatype, options, identifier)) in p.query_data.fields.iter().enumerate() {
        let (expected_datatype, expected_options, expected_identifier) = &expected[i];

        assert_eq!(datatype, expected_datatype);

        for (i, option) in options.iter().enumerate() {
            assert_eq!(option, &expected_options[i]);
        }

        assert_eq!(identifier, expected_identifier);
    }
}

#[test]
#[should_panic]
fn test_no_fields() {
    let mut p = parser::Parser::new();

    p.set_query("TABLE my_table ON my_database STRUCTURED ();".to_string())
        .parse();
}

#[test]
#[should_panic]
fn test_no_field_identifier() {
    let mut p = parser::Parser::new();

    p.set_query("TABLE my_table ON my_database STRUCTURED (UINT(1), STRING(64) name);".to_string())
        .parse();
}

#[test]
fn test_empty_options() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name);".to_string(),
    )
    .parse();
}

#[test]
fn test_multiple_options() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject);"
        .to_string(),
    )
    .parse();

    let expected = [
        (
            Identifier::Datatype("UINT".to_string()),
            vec![],
            Identifier::Field("id".to_string()),
        ),
        (
            Identifier::Datatype("STRING".to_string()),
            vec![Identifier::IntLiteral(64)],
            Identifier::Field("name".to_string()),
        ),
        (
            Identifier::Datatype("OPTIONS".to_string()),
            vec![
                Identifier::StringLiteral("math".to_string()),
                Identifier::StringLiteral("english".to_string()),
            ],
            Identifier::Field("favorite_subject".to_string()),
        ),
    ];

    for (i, (datatype, options, identifier)) in p.query_data.fields.iter().enumerate() {
        let (expected_datatype, expected_options, expected_identifier) = &expected[i];

        assert_eq!(datatype, expected_datatype);

        for (i, option) in options.iter().enumerate() {
            assert_eq!(option, &expected_options[i]);
        }

        assert_eq!(identifier, expected_identifier);
    }
}
