use super::parser::{query::QueryType, Parser};

pub mod create;

pub fn engine(parser: &Parser) -> Result<(), std::io::Error> {
    match parser.query_data._type {
        QueryType::None => panic!("No query type in query data."),
        QueryType::DatabaseCreation => create::database::create_database(parser),
        _ => Ok(()),
    }
}
