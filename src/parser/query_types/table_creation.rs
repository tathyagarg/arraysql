use super::super::constants::*;
use super::super::Parser;
use super::super::Step;

pub fn table_creation(parser: &mut Parser, step: Step) -> Step {
    match step {
        Step::DefineTableName => {
            let identifier = parser.pop_identifier();
            if identifier.is_empty() {
                panic!("Expected indentifier for table name");
            }

            parser.query_data.table_name = identifier;
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

            parser.query_data.db_name = identifier;
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

            parser
                .query_data
                .fields
                .push((token, Vec::new(), String::new()));
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
            let (_, ref mut options, _) = parser.query_data.fields.last_mut().unwrap();
            options.push(token);

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
            *identifier = token;

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
            if ![FADD, FREAD, FDELETE, LMEM].contains(&token.as_str()) {
                panic!("Expected a mode, found {:?}", token);
            }

            parser.query_data.modes.push(token);
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
            parser.query_data.constraints.push((token, Vec::new()));
            Step::DefineConstraint
        }
        Step::DefineConstraint => {
            let token = parser.pop();
            if ![EXISTS, UNIQUE, PKEY, FKEY, SUCHTHAT, DEFAULT, INC].contains(&token.as_str()) {
                panic!("Expected a constraint, found {:?}", token);
            }
            let (_, ref mut constraints) = parser.query_data.constraints.last_mut().unwrap();
            constraints.push((token, Vec::new()));

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
                    parser.pop();
                    let (_, ref mut constraints) =
                        parser.query_data.constraints.last_mut().unwrap();
                    let (_, ref mut constraint_options) = constraints.last_mut().unwrap();
                    constraint_options.push(token);
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
