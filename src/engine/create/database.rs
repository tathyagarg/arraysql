use crate::parser::{query::Identifier, Parser};
use std::fs;

pub fn create_database(parser: &Parser) -> Result<(), std::io::Error> {
    if let Identifier::StringLiteral(db_name) = &parser.query_data.db_name {
        fs::create_dir(db_name)?;
        return Ok(());
    }
    panic!("Database name not found, or it was not in a string literal identifier")
}
