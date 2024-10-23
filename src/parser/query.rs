use std::collections::HashMap;

#[derive(Debug)]
pub enum Operator {
    Equal,              // ==
    NotEqual,           // !=
    GreaterThan,        // >
    LesserThan,         // <
    GreaterThanEqualTo, // >=
    LesserThanEqualTo,  // <=
}

#[derive(Debug)]
pub struct Condition {
    pub operand1: String,
    pub operand1isfield: bool,
    pub operator: Operator,
    pub operand2: String,
    pub operand2isfield: bool,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum QueryType {
    #[default]
    None,
    DatabaseCreation,
}

#[derive(Default, Debug)]
pub struct Query {
    pub _type: QueryType,
    pub db_name: String,
    pub table_name: String,
    pub conditions: Vec<Condition>,
    pub updates: HashMap<String, String>,
    pub inserts: Vec<Vec<String>>,
    pub fields: Vec<String>,
    pub aliases: HashMap<String, String>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            _type: QueryType::None,
            db_name: String::new(),
            table_name: String::new(),
            conditions: Vec::new(),
            updates: HashMap::new(),
            inserts: Vec::new(),
            fields: Vec::new(),
            aliases: HashMap::new(),
        }
    }
}
