use arraysql::parser::{self, query::Mode};

#[test]
fn test_mode() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject) MODE FREAD FADD;"
            .to_string(),
    )
    .parse();

    assert_eq!(p.query_data.modes, vec![Mode::Fread, Mode::Fadd]);
}

#[test]
fn test_single_mode() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject) MODE FREAD;"
            .to_string(),
    )
    .parse();

    assert_eq!(p.query_data.modes, vec![Mode::Fread,]);
}

#[test]
#[should_panic]
fn test_no_mode_with_kw() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject) MODE;"
            .to_string(),
    )
    .parse();
}
