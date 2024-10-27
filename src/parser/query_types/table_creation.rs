use std::collections::HashMap;

use super::type_checker;
use crate::parser::constants::*;
use crate::parser::query::string_to_binop;
use crate::parser::query::string_to_unop;
use crate::parser::query::Constraint;
use crate::parser::query::ConstraintType;
use crate::parser::query::Expression;
use crate::parser::query::Identifier;
use crate::parser::query::Mode;
use crate::parser::Parser;
use crate::parser::Step;

fn push_to_options(latest: &mut Constraint, token: String) {
    latest.options.push(if type_checker::check_unop(&token) {
        Expression::Unary(string_to_unop(&token), Box::new(Expression::None))
    } else {
        Expression::Identifier(type_checker::as_identifier(&token))
    })
}

pub fn table_creation(parser: &mut Parser, step: Step) -> Step {
    match step {
        Step::DefineTableName => {
            let identifier = parser.pop_identifier();
            if identifier.is_empty() {
                panic!("Expected indentifier for table name");
            }

            parser.query_data.table_name = Identifier::StringLiteral(identifier);
            {
                // Ensure ON after table name
                let token = parser.pop();
                parser.ensure_token(token, ON);
                Step::DefineTableDatabase
            }
        }
        Step::DefineTableDatabase => {
            let identifier = parser.pop_identifier();
            if identifier.is_empty() {
                panic!("Expected identifier for database name.");
            }

            parser.query_data.db_name = Identifier::StringLiteral(identifier);
            {
                // Ensure 'STRUCTURED ('
                let token = parser.pop();
                parser.ensure_token(token, STRUCTURED);

                let token = parser.pop();
                parser.ensure_token(token, OPEN_PAREN);
                Step::DefineFieldDatatype
            }
        }
        Step::DefineFieldDatatype => {
            let token = parser.pop();
            if !DATATYPES.contains(&token.as_str()) {
                panic!("Expected a Datatype, found {:?}", token);
            };

            parser.query_data.fields.push((
                Identifier::Datatype(token),
                Vec::new(),
                Identifier::Field(String::new()),
            ));
            {
                let token = parser.pop();
                parser.ensure_token(token, OPEN_PAREN);

                match parser.peek().as_str() {
                    CLOSE_PAREN => {
                        parser.pop();
                        Step::DefineFieldIdentifier
                    }
                    _ => Step::DefineFieldDatatypeOption,
                }
            }
        }
        Step::DefineFieldDatatypeOption => {
            let token = parser.pop();
            let (dtype, ref mut options, _) = parser.query_data.fields.last_mut().unwrap();

            if let Identifier::Datatype(data) = dtype {
                if data == DT_OPTIONS {
                    options.push(Identifier::StringLiteral(token));
                } else {
                    options.push(Identifier::IntLiteral(token.parse::<i32>().unwrap()));
                }
            }

            let next_token = parser.pop();

            match next_token.as_str() {
                CLOSE_PAREN => Step::DefineFieldIdentifier,
                COMMA => Step::DefineFieldDatatypeOption,
                _ => panic!(
                    "Expected close paren ')' or comma ',', got {:?}",
                    next_token
                ),
            }
        }
        Step::DefineFieldIdentifier => {
            let token = parser.pop_identifier();
            if token.is_empty() {
                panic!("Expected identifier for field name.");
            }

            let mut non_digit_seen = false;
            for character in token.chars() {
                if character.is_alphabetic() {
                    non_digit_seen = true;
                }
            }

            if !non_digit_seen {
                panic!("Field name cannot be only digits.")
            }

            let (_, _, ref mut identifier) = parser.query_data.fields.last_mut().unwrap();
            *identifier = Identifier::Field(token);

            let next_token = parser.pop();
            match next_token.as_str() {
                CLOSE_PAREN => Step::DefineTableStructureCloseParen,
                COMMA => Step::DefineFieldDatatype,
                _ => panic!(
                    "Expected close paren ')' or comma ',', got {:?}",
                    next_token
                ),
            }
        }
        Step::DefineTableStructureCloseParen => match parser.pop().to_uppercase().as_str() {
            SEMICOLON => Step::End,
            MODE => Step::DefineTableMode,
            CONSTRAINED => {
                let token = parser.pop();
                parser.ensure_token(token, OPEN_PAREN);
                Step::DefineConstraintOn
            }
            found => panic!("Unexpected token {:?}", found),
        },
        Step::DefineTableMode => {
            let token = parser.pop();
            let mode = match token.as_str() {
                FADD => Mode::FADD,
                FREAD => Mode::FREAD,
                FDELETE => Mode::FDELETE,
                LMEM => Mode::LMEM,
                _ => panic!("Expected a mode, found {:?}", token),
            };

            parser.query_data.modes.push(mode);
            match parser.peek().as_str() {
                SEMICOLON => Step::End,
                FADD | FREAD | FDELETE | LMEM => Step::DefineTableMode,
                token => panic!("Expected another mode or semicolon, found {:?}", token),
            }
        }
        Step::DefineConstraintOn => {
            // This has a good number of steps referencing back to it, no need to delete
            let token = parser.pop();
            parser.ensure_token(token, ON);
            Step::DefineConstraintIdentifier
        }
        Step::DefineConstraintIdentifier => {
            let token = parser.pop_identifier();
            let mut initial: HashMap<Identifier, Vec<Constraint>> = HashMap::new();
            initial.insert(Identifier::Field(token.clone()), Vec::new());

            parser.query_data.curr_constraint = Identifier::Field(token);

            parser.query_data.constraints.extend(initial);
            Step::DefineConstraint
        }
        Step::DefineConstraint => {
            let token = parser.pop();
            let constraint = match token.as_str() {
                EXISTS => ConstraintType::Exists,
                UNIQUE => ConstraintType::Unique,
                PKEY => ConstraintType::PKey,
                FKEY => ConstraintType::FKey,
                SUCHTHAT => ConstraintType::Suchthat,
                DEFAULT => ConstraintType::Default,
                INC => ConstraintType::Inc,
                _ => panic!("Expected a constraint, found {:?}", token),
            };

            let last = parser
                .query_data
                .constraints
                .get_mut(&parser.query_data.curr_constraint)
                .unwrap();

            last.push(Constraint {
                constraint_type: constraint,
                options: Vec::new(),
            });

            match parser.peek().as_str() {
                OPEN_PAREN => {
                    parser.pop();
                    Step::DefineConstraintOption
                }
                EXISTS | UNIQUE | PKEY | FKEY | SUCHTHAT | DEFAULT | INC => Step::DefineConstraint,
                COMMA => {
                    parser.pop();
                    Step::DefineConstraintOn
                }
                CLOSE_PAREN => Step::DefineConstraintCloseParen,
                _ => panic!(),
            }
        }
        Step::DefineConstraintOption => {
            let token = parser.peek();
            match token.as_str() {
                CLOSE_PAREN => Step::DefineConstraintOptionCloseParen,
                _ => {
                    let token = parser.pop();
                    let curr_constraints = parser
                        .query_data
                        .constraints
                        .get_mut(&parser.query_data.curr_constraint)
                        .unwrap();
                    let latest: &mut Constraint = curr_constraints.last_mut().unwrap();

                    match latest.options.last_mut().unwrap_or(&mut Expression::None) {
                        Expression::Binary(_, operands) => match **operands {
                            (_, Expression::None) => {
                                operands.1 =
                                    Expression::Identifier(type_checker::as_identifier(&token))
                            }
                            (_, _) => push_to_options(latest, token),
                        },
                        Expression::Unary(_, operand) => match **operand {
                            Expression::None => {
                                *operand = Box::new(Expression::Identifier(
                                    type_checker::as_identifier(&token),
                                ))
                            }
                            _ => push_to_options(latest, token),
                        },
                        Expression::Identifier(_) => {
                            if type_checker::check_binop(&token) {
                                let prev = latest.options.pop().unwrap();
                                latest.options.push(Expression::Binary(
                                    string_to_binop(&token),
                                    Box::new((prev, Expression::None)),
                                ));
                            } else {
                                push_to_options(latest, token);
                            }
                        }
                        Expression::None => push_to_options(latest, token),
                    };

                    Step::DefineConstraintOption
                }
            }
        }
        Step::DefineConstraintOptionCloseParen => {
            parser.pop();
            match parser.peek().as_str() {
                COMMA => {
                    parser.pop();
                    Step::DefineConstraintOn
                }
                EXISTS | UNIQUE | PKEY | FKEY | SUCHTHAT | DEFAULT | INC => Step::DefineConstraint,
                CLOSE_PAREN => {
                    parser.pop();
                    Step::DefineConstraintCloseParen
                }
                found_token => {
                    panic!("Expected comma or constraint, found {:?}", found_token)
                }
            }
        }
        Step::DefineConstraintCloseParen => {
            let token = parser.pop();
            parser.ensure_token(token, MODE);
            Step::DefineTableMode
        }
        found => panic!(
            "Incorrect module used. Table creation module used to handle step {:?}",
            found
        ),
    }
}
