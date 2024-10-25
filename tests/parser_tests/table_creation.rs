use maybe_sql::parser;

#[test]
fn test_table_creation() {
    let mut p = parser::Parser::default();

    p.set_query("TABLE my_table ON my_database STRUCTURED (UINT(1) id);".to_string())
        .parse();

    assert_eq!(p.query_data._type, parser::query::QueryType::TableCreation);
    assert_eq!(p.query_data.db_name, "my_database");
    assert_eq!(p.query_data.table_name, "my_table");
}

#[test]
fn test_table_field_data() {
    let mut p = parser::Parser::default();

    p.set_query("TABLE my_table ON my_database STRUCTURED (UINT(1) id);".to_string())
        .parse();

    let field_data = &p.query_data.fields[0];
    let (datatype, options, identifier) = field_data;

    assert_eq!(datatype, "UINT");
    assert_eq!(options, &vec!["1"]);
    assert_eq!(identifier, "id");
}

#[test]
fn test_multiple_fields() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT(1) id, STRING(64) name);".to_string(),
    )
    .parse();

    let expected = [("UINT", vec!["1"], "id"), ("STRING", vec!["64"], "name")];

    for (i, (datatype, options, identifier)) in p.query_data.fields.iter().enumerate() {
        let (expected_datatype, expected_options, expected_identifier) = &expected[i];

        assert_eq!(datatype, expected_datatype);

        for (i, option) in options.iter().enumerate() {
            assert_eq!(option, expected_options[i]);
        }

        assert_eq!(identifier, expected_identifier);
    }
}

#[test]
#[should_panic(expected = "Expected a Datatype")]
fn test_no_fields() {
    let mut p = parser::Parser::default();

    p.set_query("TABLE my_table ON my_database STRUCTURED ();".to_string())
        .parse();
}

#[test]
#[should_panic(expected = "Expected identifier for field name.")]
fn test_no_field_identifier() {
    let mut p = parser::Parser::default();

    p.set_query("TABLE my_table ON my_database STRUCTURED (UINT(1), STRING(64) name);".to_string())
        .parse();
}

#[test]
fn test_empty_options() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name);".to_string(),
    )
    .parse();
}

#[test]
fn test_multiple_options() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject);".to_string(),
    )
    .parse();

    let expected = [
        ("UINT", vec![], "id"),
        ("STRING", vec!["64"], "name"),
        ("OPTIONS", vec!["math", "english"], "favorite_subject"),
    ];

    for (i, (datatype, options, identifier)) in p.query_data.fields.iter().enumerate() {
        let (expected_datatype, expected_options, expected_identifier) = &expected[i];

        assert_eq!(datatype, expected_datatype);

        for (i, option) in options.iter().enumerate() {
            assert_eq!(option, expected_options[i]);
        }

        assert_eq!(identifier, expected_identifier);
    }
}

#[test]
fn test_mode() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table \
        ON my_database \
        STRUCTURED (\
            UINT() id,\
            STRING(64) name,\
            OPTIONS(math, english) favorite_subject\
        )\
        MODE FREAD FINSERT;"
            .to_string(),
    )
    .parse();

    assert_eq!(p.query_data.modes, vec!["FREAD", "FINSERT"]);
}

#[test]
fn test_single_mode() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table \
        ON my_database \
        STRUCTURED (\
            UINT() id,\
            STRING(64) name,\
            OPTIONS(math, english) favorite_subject\
        )\
        MODE FREAD;"
            .to_string(),
    )
    .parse();

    assert_eq!(p.query_data.modes, vec!["FREAD",]);
}
