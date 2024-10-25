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
    TableCreation,
}

#[derive(Default, Debug)]
pub struct Query {
    pub _type: QueryType,
    pub db_name: String,
    pub table_name: String,

    //               DTYPE ,    OPTIONS , IDTFR
    pub fields: Vec<(String, Vec<String>, String)>,
    pub modes: Vec<String>,
}
