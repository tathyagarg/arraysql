use crate::parser::{query::Identifier, Parser};
use std::fs;

pub fn create_table(parser: &Parser) -> Result<(), std::io::Error> {
    if let Identifier::StringLiteral(db_name) = &parser.query_data.db_name {
        if let Identifier::StringLiteral(table_name) = &parser.query_data.table_name {
            fs::File::create(format!("{}/{}.asql", db_name, table_name))?;
            return Ok(());
        }
        panic!("Table name not found, or it was not in a string literal identifier")
    }
    panic!("Database name not found, or it was not in a string literal identifier")
}
