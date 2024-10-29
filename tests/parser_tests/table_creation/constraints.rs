use std::collections::HashMap;

use arraysql::parser::{
    self,
    query::{BinaryOperation, Constraint, ConstraintType, Expression, Identifier},
};

fn make_assertions(
    expected: HashMap<Identifier, Vec<Constraint>>,
    found: HashMap<Identifier, Vec<Constraint>>,
) {
    for (field, constraints) in &found {
        let expected_contraints = &expected[field];
        assert_eq!(constraints, expected_contraints);
    }
    /*
    for (i, (field, constraints)) in found.iter().enumerate() {
        let (expected_field, expected_contraints) = &expected[i];
        assert_eq!(field, expected_field);
        for (j, (constraint, options)) in constraints.iter().enumerate() {
            let (expected_contraint, expected_options) = &expected_contraints[j];
            assert_eq!(constraint, expected_contraint);
            assert_eq!(options, expected_options);
        }
    }*/
}

#[test]
fn test_basic_contraints() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject) CONSTRAINED (ON id EXISTS PKEY UNIQUE INC, ON name EXISTS, ON favorite_subject DEFAULT('math')) MODE FREAD FADD;"
            .to_string(),
    )
    .parse();

    let expected = HashMap::from([
        (
            Identifier::Field("id".to_string()),
            vec![
                Constraint {
                    constraint_type: ConstraintType::Exists,
                    options: vec![],
                },
                Constraint {
                    constraint_type: ConstraintType::PKey,
                    options: vec![],
                },
                Constraint {
                    constraint_type: ConstraintType::Unique,
                    options: vec![],
                },
                Constraint {
                    constraint_type: ConstraintType::Inc,
                    options: vec![],
                },
            ],
        ),
        (
            Identifier::Field("name".to_string()),
            vec![Constraint {
                constraint_type: ConstraintType::Exists,
                options: vec![],
            }],
        ),
        (
            Identifier::Field("favorite_subject".to_string()),
            vec![Constraint {
                constraint_type: ConstraintType::Default,
                options: vec![Expression::Identifier(Identifier::StringLiteral(
                    "'math'".to_string(),
                ))],
            }],
        ),
    ]);

    make_assertions(expected, p.query_data.constraints);
}

#[test]
fn test_multiple_constraint_options() {
    let mut p = parser::Parser::new();

    p.set_query(
        "TABLE my_table ON my_database STRUCTURED (UINT() id, STRING(64) name, OPTIONS(math, english) favorite_subject, UINT() max_marks) CONSTRAINED (ON id EXISTS PKEY UNIQUE INC, ON name EXISTS, ON favorite_subject DEFAULT('math'), ON max_marks SUCHTHAT(max_marks <= 80) DEFAULT(0)) MODE FREAD FADD;"
            .to_string(),
    )
    .parse();

    let expected = HashMap::from([
        (
            Identifier::Field("id".to_string()),
            vec![
                Constraint {
                    constraint_type: ConstraintType::Exists,
                    options: vec![],
                },
                Constraint {
                    constraint_type: ConstraintType::PKey,
                    options: vec![],
                },
                Constraint {
                    constraint_type: ConstraintType::Unique,
                    options: vec![],
                },
                Constraint {
                    constraint_type: ConstraintType::Inc,
                    options: vec![],
                },
            ],
        ),
        (
            Identifier::Field("name".to_string()),
            vec![Constraint {
                constraint_type: ConstraintType::Exists,
                options: vec![],
            }],
        ),
        (
            Identifier::Field("favorite_subject".to_string()),
            vec![Constraint {
                constraint_type: ConstraintType::Default,
                options: vec![Expression::Identifier(Identifier::StringLiteral(
                    "'math'".to_string(),
                ))],
            }],
        ),
        (
            Identifier::Field("max_marks".to_string()),
            vec![
                Constraint {
                    constraint_type: ConstraintType::Suchthat,
                    options: vec![Expression::Binary(
                        BinaryOperation::LesserThanEqualTo,
                        Box::new((
                            Expression::Identifier(Identifier::Field("max_marks".to_string())),
                            Expression::Identifier(Identifier::IntLiteral(80)),
                        )),
                    )],
                },
                Constraint {
                    constraint_type: ConstraintType::Default,
                    options: vec![Expression::Identifier(Identifier::IntLiteral(0))],
                },
            ],
        ),
    ]);

    make_assertions(expected, p.query_data.constraints);
}
