use crate::parser::query::Identifier;
use crate::parser::query_types::type_checker;

use super::super::constants::*;
use super::super::Parser;
use super::super::Step;

pub fn inserting(parser: &mut Parser, step: Step) -> Step {
    match step {
        Step::InsertValueIdentifier => {
            let token = parser.pop_string_or_identifier();
            parser
                .query_data
                .inserted_value
                .push(type_checker::as_identifier(&token));

            match parser.peek().as_str() {
                CLOSE_PAREN => {
                    let token = parser.pop();
                    parser.ensure_token(token, CLOSE_PAREN);

                    let token = parser.pop();
                    parser.ensure_token(token, ON);

                    Step::InsertTable
                }
                COMMA => {
                    parser.pop();
                    Step::InsertValueIdentifier
                }
                found_token => panic!("Unexpected token {:?}", found_token),
            }
        }
        Step::InsertTable => {
            let token = parser.pop_identifier();
            parser.query_data.table_name = Identifier::StringLiteral(token);

            {
                let token = parser.pop();
                parser.ensure_token(token, STRUCTURED);

                let token = parser.pop();
                parser.ensure_token(token, OPEN_PAREN);
                Step::InsertFieldIdentifier
            }
        }
        Step::InsertFieldIdentifier => {
            let token = parser.pop_string_or_identifier();
            parser
                .query_data
                .inserted_field
                .push(type_checker::as_identifier(&token));

            match parser.peek().as_str() {
                CLOSE_PAREN => {
                    let token = parser.pop();
                    parser.ensure_token(token, CLOSE_PAREN);

                    let token = parser.pop();
                    parser.ensure_token(token, ON);

                    Step::InsertDatabase
                }
                COMMA => {
                    parser.pop();
                    Step::InsertFieldIdentifier
                }
                found_token => panic!("Unexpected token {:?}", found_token),
            }
        }
        Step::InsertDatabase => {
            let token = parser.pop_identifier();
            parser.query_data.db_name = Identifier::StringLiteral(token);

            let token = parser.pop();
            parser.ensure_token(token, SEMICOLON);
            Step::End
        }
        found => panic!(
            "Incorrect module used. Inserting module used to handle step {:?}",
            found
        ),
    }
}
