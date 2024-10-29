use crate::parser::query::string_to_binop;
use crate::parser::query::string_to_unop;
use crate::parser::query::Expression;
use crate::parser::query::Identifier;
use crate::parser::query_types::type_checker::as_identifier;

use super::super::constants::*;
use super::super::Parser;
use super::super::Step;
use super::type_checker;

fn parse_condition(tokens: Vec<String>) -> Expression {
    let mut res = Expression::None;

    println!("{:?}", tokens);

    for (i, token) in tokens.iter().enumerate() {
        if CONNECTORS.contains(&token.as_str()) {
            res = Expression::Binary(
                string_to_binop(token),
                Box::new((res, parse_condition(tokens[i + 1..].to_vec()))),
            );
            return res;
        } else if UNOPS.contains(&token.as_str()) {
            res = Expression::Unary(string_to_unop(token), Box::new(Expression::None));
        } else if BINOPS.contains(&token.as_str()) {
            res = Expression::Binary(string_to_binop(token), Box::new((res, Expression::None)));
        } else {
            // Identifier
            match res {
                Expression::None => res = Expression::Identifier(as_identifier(token)),
                Expression::Identifier(_) => panic!("Found identifier after identifier"),
                Expression::Unary(ref operator, ref operand) => {
                    if let Expression::None = **operand {
                        res = Expression::Unary(
                            operator.clone(),
                            Box::new(Expression::Identifier(as_identifier(token))),
                        )
                    }
                }
                Expression::Binary(ref operator, ref operands) => {
                    if let (op1, Expression::None) = &**operands {
                        res = Expression::Binary(
                            operator.clone(),
                            Box::new((op1.clone(), Expression::Identifier(as_identifier(token)))),
                        )
                    }
                }
            }
        }
    }

    res
}

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
            parser.query_data.db_name = Identifier::StringLiteral(token.clone());

            match parser.pop().as_str() {
                SEMICOLON => Step::End,
                WHERE => Step::ReadConditionPart,
                found_token => panic!("Unexpected token {}", found_token),
            }
        }
        Step::ReadConditionPart => {
            let mut tokens: Vec<String> = Vec::new();
            while parser.peek() != SEMICOLON {
                tokens.push(parser.pop());
            }

            parser.query_data.conditions = parse_condition(tokens[1..tokens.len() - 1].to_vec());

            Step::End
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
