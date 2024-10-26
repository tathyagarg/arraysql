use maybe_sql::parser;

type Constraints<'a> = Vec<(&'a str, Vec<(&'a str, Vec<&'a str>)>)>;
type ConstraintsString = Vec<(String, Vec<(String, Vec<String>)>)>;

fn make_assertions(expected: Constraints, found: ConstraintsString) {
    for (i, (field, constraints)) in found.iter().enumerate() {
        let (expected_field, expected_contraints) = &expected[i];
        assert_eq!(field, expected_field);
        for (j, (constraint, options)) in constraints.iter().enumerate() {
            let (expected_contraint, expected_options) = &expected_contraints[j];
            assert_eq!(constraint, expected_contraint);
            assert_eq!(options, expected_options);
        }
    }
}

#[test]
fn test_basic_contraints() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table \
            ON my_database \
            STRUCTURED (\
                UINT() id,\
                STRING(64) name,\
                OPTIONS(math, english) favorite_subject\
            )\
            CONSTRAINED (\
                ON id EXISTS PKEY UNIQUE INC,\
                ON name EXISTS,\
                ON favorite_subject DEFAULT(math)\
            )\
            MODE FREAD FADD;"
            .to_string(),
    )
    .parse();

    let expected: Constraints = vec![
        (
            "id",
            vec![
                ("EXISTS", vec![]),
                ("PKEY", vec![]),
                ("UNIQUE", vec![]),
                ("INC", vec![]),
            ],
        ),
        ("name", vec![("EXISTS", vec![])]),
        ("favorite_subject", vec![("DEFAULT", vec!["math"])]),
    ];

    make_assertions(expected, p.query_data.constraints);
}

#[test]
fn test_multiple_constraint_options() {
    let mut p = parser::Parser::default();

    p.set_query(
        "TABLE my_table \
            ON my_database \
            STRUCTURED (\
                UINT() id,\
                STRING(64) name,\
                OPTIONS(math, english) favorite_subject,\
                UINT() max_marks\
            )\
            CONSTRAINED (\
                ON id EXISTS PKEY UNIQUE INC,\
                ON name EXISTS,\
                ON favorite_subject DEFAULT(math),\
                ON max_marks SUCHTHAT(max_marks <= 80) DEFAULT(0)\
            )\
            MODE FREAD FADD;"
            .to_string(),
    )
    .parse();

    let expected: Constraints = vec![
        (
            "id",
            vec![
                ("EXISTS", vec![]),
                ("PKEY", vec![]),
                ("UNIQUE", vec![]),
                ("INC", vec![]),
            ],
        ),
        ("name", vec![("EXISTS", vec![])]),
        ("favorite_subject", vec![("DEFAULT", vec!["math"])]),
        (
            "max_marks",
            vec![
                ("SUCHTHAT", vec!["max_marks", "<=", "80"]),
                ("DEFAULT", vec!["0"]),
            ],
        ),
    ];

    make_assertions(expected, p.query_data.constraints);
}
