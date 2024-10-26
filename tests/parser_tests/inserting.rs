use maybe_sql::parser::{self, query::QueryType};

#[test]
fn test_basic_insert() {
    let mut p = parser::Parser::default();
    p.set_query(
        "INSERT STRUCTURED ('my_data')\
        ON my_table STRUCTURED (field1)\
        ON my_database;"
            .to_string(),
    )
    .parse();

    assert_eq!(p.query_data._type, QueryType::Insert);
    assert_eq!(p.query_data.table_name, "my_table");
    assert_eq!(p.query_data.db_name, "my_database");
    assert_eq!(p.query_data.inserted_value, vec!["my_data"]);
    assert_eq!(p.query_data.inserted_field, vec!["field1"]);
}

#[test]
fn test_multiple_fields() {
    let mut p = parser::Parser::default();
    p.set_query(
        "INSERT STRUCTURED (\
            'Tathya',\
            15,\
            ['Rust', 'Python']\
        )\
        ON my_table STRUCTURED (\
            name,\
            age,\
            known_languages\
        )\
        ON my_database;"
            .to_string(),
    )
    .parse();

    println!("{:?}", p.query_data);

    assert_eq!(
        p.query_data.inserted_value,
        vec!["Tathya", "15", "'Rust', 'Python'"]
    );
    assert_eq!(
        p.query_data.inserted_field,
        vec!["name", "age", "known_languages"]
    );
}
