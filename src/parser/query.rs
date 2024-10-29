use super::constants::*;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum QueryType {
    #[default]
    None,
    DatabaseCreation,
    TableCreation,
    Insert,
    Read,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnaryOperation {
    Abs,
    Negative,
    LogicalNot,
    BitwiseNot,
    Exists,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equals,
    NotEquals,
    GreaterThanEqualTo,
    LesserThanEqualTo,
    GreaterThan,
    LesserThan,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Identifier {
    StringLiteral(String),
    IntLiteral(i32),
    Datatype(String),
    Field(String),
    Array(Vec<Identifier>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    None,
    Identifier(Identifier),
    Unary(UnaryOperation, Box<Expression>),
    Binary(BinaryOperation, Box<(Expression, Expression)>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    Fadd,
    Fread,
    Fdelete,
    Lmem,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConstraintType {
    None,
    Exists,
    Unique,
    PKey,
    FKey,
    Suchthat,
    Default,
    Inc,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub options: Vec<Expression>,
}

#[derive(Debug)]
pub struct Query {
    pub _type: QueryType,
    pub db_name: Identifier,
    pub table_name: Identifier,

    // ============ Table Creation ============
    //                 DTYPE        OPTIONS          IDTFR
    pub fields: Vec<(Identifier, Vec<Identifier>, Identifier)>,
    pub modes: Vec<Mode>,
    pub constraints: HashMap<Identifier, Vec<Constraint>>,
    pub curr_constraint: Identifier,
    //
    // ============ Insertions ============
    pub inserted_value: Vec<Identifier>,
    pub inserted_field: Vec<Identifier>,
    //
    // ============ Reading ============
    pub read_fields: Vec<Identifier>,
    pub conditions: Expression,
}

pub fn string_to_unop(token: &String) -> UnaryOperation {
    match token.as_str() {
        ABS => UnaryOperation::Abs,
        NEG => UnaryOperation::Negative,
        NOT => UnaryOperation::LogicalNot,
        BWNOT => UnaryOperation::BitwiseNot,
        EXISTS => UnaryOperation::Exists,
        _ => panic!("Expected unary operation, found {}", token),
    }
}

pub fn string_to_binop(token: &String) -> BinaryOperation {
    match token.as_str() {
        ADD => BinaryOperation::Addition,
        SUB => BinaryOperation::Subtraction,
        MUL => BinaryOperation::Multiplication,
        DIV => BinaryOperation::Division,
        EQ => BinaryOperation::Equals,
        NE => BinaryOperation::NotEquals,
        GE => BinaryOperation::GreaterThanEqualTo,
        LE => BinaryOperation::LesserThanEqualTo,
        GT => BinaryOperation::GreaterThan,
        LT => BinaryOperation::LesserThan,
        _ => panic!("Expected binary operation, found {}", token),
    }
}
