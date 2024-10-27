use super::super::constants::*;
use super::super::Parser;
use super::super::Step;

pub fn reading(parser: &mut Parser, step: Step) -> Step {
    match step {
        Step::ReadTableName => {
            let token = parser.pop_identifier();
            parser.query_data.table_name = token;

            {
                let token = parser.pop();
                parser.ensure_token(token, ON);

                Step::ReadDatabaseName
            }
        }
        Step::ReadDatabaseName => {
            let token = parser.pop_identifier();
            parser.query_data.db_name = token;

            match parser.pop().as_str() {
                SEMICOLON => Step::End,
                WHERE => Step::ReadConditionPart,
                found_token => panic!("Unexpected token {}", found_token),
            }
        }
        Step::ReadConditionPart => {
            let token = parser.pop();
            parser.query_data.conditions.push(token.clone());

            match parser.peek().as_str() {
                SEMICOLON => Step::End,
                _ => Step::ReadConditionPart,
            }
        }
        Step::ReadFieldIdentifier => {
            let token = parser.pop_identifier();
            parser.query_data.read_fields.push(token.to_string());

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
