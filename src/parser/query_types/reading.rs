use crate::parser::query::string_to_binop;
use crate::parser::query::string_to_unop;
use crate::parser::query::Expression;
use crate::parser::query::Identifier;

use super::super::constants::*;
use super::super::Parser;
use super::super::Step;
use super::type_checker;

pub fn reading(parser: &mut Parser, step: Step) -> Step {
    match step {
        Step::ReadTableName => {
            let token = parser.pop_identifier();
            parser.query_data.table_name = Identifier::StringLiteral(token);

            {
                let token = parser.pop();
                parser.ensure_token(token, ON);

                Step::ReadDatabaseName
            }
        }
        Step::ReadDatabaseName => {
            let token = parser.pop_identifier();
            parser.query_data.db_name = Identifier::StringLiteral(token);

            match parser.pop().as_str() {
                SEMICOLON => Step::End,
                WHERE => Step::ReadConditionPart,
                found_token => panic!("Unexpected token {}", found_token),
            }
        }
        Step::ReadConditionPart => {
            let token = parser.pop();
            match &parser.query_data.conditions {
                Expression::None => {
                    if type_checker::check_unop(&token) {
                        parser.query_data.conditions =
                            Expression::Unary(string_to_unop(&token), Box::new(Expression::None))
                    } else {
                        parser.query_data.conditions =
                            Expression::Identifier(type_checker::as_identifier(&token))
                    }
                }
                Expression::Identifier(_) => {
                    if type_checker::check_binop(&token) {
                        let prev = parser.query_data.conditions.clone();
                        parser.query_data.conditions = Expression::Binary(
                            string_to_binop(&token),
                            Box::new((prev, Expression::None)),
                        );
                    } else if type_checker::check_unop(&token) {
                        parser.query_data.conditions =
                            Expression::Unary(string_to_unop(&token), Box::new(Expression::None))
                    } else {
                        parser.query_data.conditions =
                            Expression::Identifier(type_checker::as_identifier(&token))
                    }
                }
                Expression::Unary(operator, operand) => match **operand {
                    Expression::None => {
                        parser.query_data.conditions = Expression::Unary(
                            operator.clone(),
                            Box::new(Expression::Identifier(type_checker::as_identifier(&token))),
                        )
                    }
                    _ => todo!("Is this even possible? - UnOp"),
                },
                Expression::Binary(operator, operands) => match &**operands {
                    (op, Expression::None) => {
                        parser.query_data.conditions = Expression::Binary(
                            operator.clone(),
                            Box::new((
                                op.clone(),
                                Expression::Identifier(type_checker::as_identifier(&token)),
                            )),
                        )
                    }
                    (_, _) => todo!("Is this even possible? - BinOp"),
                },
            }

            match parser.peek().as_str() {
                SEMICOLON => Step::End,
                _ => Step::ReadConditionPart,
            }
        }
        Step::ReadFieldIdentifier => {
            let token = parser.pop_identifier();
            if !type_checker::check_field(&token) {
                panic!("Expected a field, found {}", token);
            }
            parser.query_data.read_fields.push(Identifier::Field(token));

            match parser.pop().as_str() {
                COMMA => Step::ReadFieldIdentifier,
                CLOSE_PAREN => {
                    let token = parser.pop();
                    parser.ensure_token(token, ON);

                    Step::ReadTableName
                }
                found_token => panic!("Unexpected token {:?}", found_token),
            }
        }
        found => panic!(
            "Incorrect module used. Read module used to handle step {:?}",
            found
        ),
    }
}
