use query::QueryType;
use regex::Regex;
use std::cmp::min;

pub mod query;
pub mod step;

use step::Step;

#[derive(Default, Debug)]
pub struct Parser {
    pub query: String,
    pub location: usize,
    pub query_data: query::Query,
    pub step: Step,
}

const DATABASE: &str = "DATABASE";
const TABLE: &str = "TABLE";
const ON: &str = "ON";
const STRUCTURED: &str = "STRUCTURED";
const MODE: &str = "MODE";
const FINSERT: &str = "FINSERT";
const FREAD: &str = "FREAD";
const FDELETE: &str = "FDELETE";
const LMEM: &str = "LMEM";
const CONSTRAINED: &str = "CONSTRAINED";

const EXISTS: &str = "EXISTS";
const UNIQUE: &str = "UNIQUE";
const PKEY: &str = "PKEY";
const FKEY: &str = "FKEY";
const SUCHTHAT: &str = "SUCHTHAT";
const DEFAULT: &str = "DEFAULT";
const INC: &str = "INC";

const OPEN_PAREN: &str = "(";
const CLOSE_PAREN: &str = ")";
const COMMA: &str = ",";
const SEMICOLON: &str = ";";

pub const KEYWORDS: &[&str] = &[
    OPEN_PAREN,
    CLOSE_PAREN,
    "=",
    ">",
    "<",
    "!=",
    ">=",
    "<=",
    COMMA,
    SEMICOLON,
    DATABASE,
    TABLE,
    ON,
    STRUCTURED,
    MODE,
    FINSERT,
    FREAD,
    FDELETE,
    LMEM,
    CONSTRAINED,
    EXISTS,
    UNIQUE,
    PKEY,
    FKEY,
    SUCHTHAT,
    DEFAULT,
    INC,
];

const DT_STRING: &str = "STRING";
const DT_OPTIONS: &str = "OPTIONS";
const DT_CHAR: &str = "CHAR";
const DT_BYTES: &str = "BYTES";
const DT_UINT: &str = "UINT";
const DT_INT: &str = "INT";
const DT_FLOAT: &str = "FLOAT";
const DT_TIMESTAMP: &str = "TIMESTAMP";

const DATATYPES: [&str; 8] = [
    DT_STRING,
    DT_OPTIONS,
    DT_CHAR,
    DT_BYTES,
    DT_UINT,
    DT_INT,
    DT_FLOAT,
    DT_TIMESTAMP,
];

impl Parser {
    pub fn reset(&mut self) {
        *self = Parser::default();
    }

    pub fn set_query(&mut self, query: String) -> &mut Parser {
        self.query = query;
        self
    }

    pub fn parse(&mut self) {
        self.step = Step::Start;

        while self.location < self.query.len() {
            match self.step {
                Step::Start => match self.pop().to_uppercase().as_str() {
                    DATABASE => {
                        self.query_data._type = QueryType::DatabaseCreation;
                        self.step = Step::DefineDatabaseName;
                    }
                    TABLE => {
                        self.query_data._type = QueryType::TableCreation;
                        self.step = Step::DefineTableName;
                    }
                    _ => {
                        panic!("Invalid token found {}", self.peek());
                    }
                },
                Step::DefineDatabaseName => {
                    let identifier = self.pop_identifier();
                    if identifier.is_empty() {
                        panic!("Expected identifier for database name.");
                    }

                    self.query_data.db_name = identifier;
                    self.step = Step::End;
                }
                Step::DefineTableName => {
                    let identifier = self.pop_identifier();
                    if identifier.is_empty() {
                        panic!("Expected indentifier for table name");
                    }

                    self.query_data.table_name = identifier;
                    self.step = Step::DefineTableOn;
                }
                Step::DefineTableOn => {
                    let token = self.pop().to_uppercase();
                    if token != ON {
                        panic!("Expected ON, got {}", token);
                    }
                    self.step = Step::DefineTableDatabase;
                }
                Step::DefineTableDatabase => {
                    let identifier = self.pop_identifier();
                    if identifier.is_empty() {
                        panic!("Expected identifier for database name.");
                    }

                    self.query_data.db_name = identifier;
                    self.step = Step::DefineTableStructure;
                }
                Step::DefineTableStructure => {
                    let token = self.pop().to_uppercase();
                    if token != STRUCTURED {
                        panic!("Expected STRUCTURED, got {}", token);
                    }
                    self.step = Step::DefineTableStructureOpenParen;
                }
                Step::DefineTableStructureOpenParen => {
                    let open_paren = self.pop();
                    if open_paren != OPEN_PAREN {
                        panic!("Expected open parenthesis, found {}", open_paren);
                    }

                    self.step = Step::DefineFieldDatatype;
                }
                Step::DefineFieldDatatype => {
                    let token = self.pop().to_uppercase();
                    if !DATATYPES.contains(&token.as_str()) {
                        panic!("Expected a Datatype, found {}", token);
                    };

                    self.query_data
                        .fields
                        .push((token, Vec::new(), String::new()));
                    self.step = Step::DefineFieldDatatypeOpenParen;
                }
                Step::DefineFieldDatatypeOpenParen => {
                    let token = self.pop();
                    if token != OPEN_PAREN {
                        panic!(
                            "Expected open paren after datatype for options, found {}",
                            token
                        );
                    }

                    let next_token = self.peek();
                    self.step = if next_token == CLOSE_PAREN {
                        self.pop();
                        Step::DefineFieldIdentifier
                    } else {
                        Step::DefineFieldDatatypeOption
                    }
                }
                Step::DefineFieldDatatypeOption => {
                    let token = self.pop();
                    let (_, ref mut options, _) = self.query_data.fields.last_mut().unwrap();
                    options.push(token);

                    let next_token = self.pop();
                    self.step = match next_token.as_str() {
                        CLOSE_PAREN => Step::DefineFieldIdentifier,
                        COMMA => Step::DefineFieldDatatypeOption,
                        _ => panic!("Expected close paren ')' or comma ',', got {}", next_token),
                    }
                }
                Step::DefineFieldIdentifier => {
                    let token = self.pop_identifier();
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

                    let (_, _, ref mut identifier) = self.query_data.fields.last_mut().unwrap();
                    *identifier = token;

                    let next_token = self.pop();
                    self.step = match next_token.as_str() {
                        CLOSE_PAREN => Step::DefineTableStructureCloseParen,
                        COMMA => Step::DefineFieldDatatype,
                        _ => panic!("Expected close paren ')' or comma ',', got {}", next_token),
                    }
                }
                Step::DefineTableStructureCloseParen => {
                    self.step = match self.pop().to_uppercase().as_str() {
                        SEMICOLON => Step::End,
                        MODE => Step::DefineTableMode,
                        CONSTRAINED => Step::DefineConstraintOpenParen,
                        found => panic!("Unexpected token {}", found),
                    };
                }
                Step::DefineTableMode => {
                    let token = self.pop().to_uppercase();
                    if ![FINSERT, FREAD, FDELETE, LMEM].contains(&token.as_str()) {
                        panic!("Expected a mode, found {}", token);
                    }

                    self.query_data.modes.push(token);
                    self.step = match self.peek().as_str() {
                        SEMICOLON => Step::End,
                        FINSERT | FREAD | FDELETE | LMEM => Step::DefineTableMode,
                        token => panic!("Expected another mode or semicolon, found {}", token),
                    }
                }
                Step::DefineConstraintOpenParen => {
                    let token = self.pop();
                    if token != OPEN_PAREN {
                        panic!("Expected open paren, found {}", token);
                    }

                    self.step = Step::DefineConstraintOn;
                }
                Step::DefineConstraintOn => {
                    let token = self.pop();
                    if token != ON {
                        panic!("Expected ON, found {}", token);
                    }

                    self.step = Step::DefineConstraintIdentifier;
                }
                Step::DefineConstraintIdentifier => {
                    let token = self.pop_identifier();
                    self.query_data.constraints.push((token, Vec::new()));
                    self.step = Step::DefineConstraint;
                }
                Step::DefineConstraint => {
                    let token = self.pop();
                    if ![EXISTS, UNIQUE, PKEY, FKEY, SUCHTHAT, DEFAULT, INC]
                        .contains(&token.as_str())
                    {
                        panic!("Expected a constraint, found {}", token);
                    }
                    let (_, ref mut constraints) = self.query_data.constraints.last_mut().unwrap();
                    constraints.push((token, Vec::new()));

                    self.step = match self.peek().as_str() {
                        OPEN_PAREN => Step::DefineConstraintOptionOpenParen,
                        EXISTS | UNIQUE | PKEY | FKEY | SUCHTHAT | DEFAULT | INC => {
                            Step::DefineConstraint
                        }
                        COMMA => {
                            self.pop();
                            Step::DefineConstraintOn
                        }
                        CLOSE_PAREN => Step::DefineConstraintCloseParen,
                        _ => panic!(),
                    }
                }
                Step::DefineConstraintOptionOpenParen => {
                    self.pop();
                    self.step = Step::DefineConstraintOption;
                }
                Step::DefineConstraintOption => {
                    let token = self.peek();
                    self.step = match token.as_str() {
                        CLOSE_PAREN => Step::DefineConstraintOptionCloseParen,
                        _ => {
                            self.pop();
                            let (_, ref mut constraints) =
                                self.query_data.constraints.last_mut().unwrap();
                            let (_, ref mut constraint_options) = constraints.last_mut().unwrap();
                            constraint_options.push(token);
                            Step::DefineConstraintOption
                        }
                    }
                }
                Step::DefineConstraintOptionCloseParen => {
                    self.pop();
                    self.step = match self.peek().as_str() {
                        COMMA => {
                            self.pop();
                            Step::DefineConstraintOn
                        }
                        EXISTS | UNIQUE | PKEY | FKEY | SUCHTHAT | DEFAULT | INC => {
                            Step::DefineConstraint
                        }
                        CLOSE_PAREN => Step::DefineConstraintCloseParen,
                        found_token => {
                            panic!("Expected comma or constraint, found {}", found_token)
                        }
                    }
                }
                Step::DefineConstraintCloseParen => {
                    self.pop();
                    self.step = match self.pop().as_str() {
                        MODE => Step::DefineTableMode,
                        found_token => panic!("Expected mode or end query, found {}", found_token),
                    }
                }
                Step::End => return,
            }
        }
    }

    pub fn peek(&mut self) -> String {
        let (data, _) = self.peek_with_length();
        data
    }

    pub fn peek_identifier(&mut self) -> String {
        let (data, _) = self.peek_identifier_with_length();
        data
    }

    pub fn pop(&mut self) -> String {
        let (data, len) = self.peek_with_length();
        self.location += len;
        self.pop_whitespace();

        data
    }

    fn pop_whitespace(&mut self) {
        while self.location < self.query.len()
            && self.query.chars().nth(self.location).unwrap() == ' '
        {
            self.location += 1;
        }
    }

    pub fn pop_identifier(&mut self) -> String {
        let (data, len) = self.peek_identifier_with_length();
        self.location += len;
        self.pop_whitespace();

        data
    }

    fn peek_with_length(&self) -> (String, usize) {
        if self.location >= self.query.len() {
            return (String::new(), 0);
        }
        for keyword in KEYWORDS {
            let token = &self.query
                [self.location..min(self.query.len(), self.location + keyword.len())]
                .to_uppercase();
            if token == keyword {
                return (token.to_string(), token.len());
            }
        }
        if self.query.chars().nth(self.location).unwrap() == '\'' {
            return self.peek_string_with_length();
        }
        self.peek_identifier_with_length()
    }

    fn peek_string_with_length(&self) -> (String, usize) {
        if self.query.len() < self.location
            || self.query.chars().nth(self.location).unwrap() != '\''
        {
            return (String::new(), 0);
        }

        for i in (self.location + 1)..(self.query.len()) {
            if self.query.chars().nth(i).unwrap() == '\''
                && self.query.chars().nth(i - 1).unwrap() != '\\'
            {
                return (
                    self.query[self.location + 1..i].to_string(),
                    self.query[self.location + 1..i].len() + 2,
                );
            }
        }
        (String::new(), 0)
    }

    fn peek_identifier_with_length(&self) -> (String, usize) {
        let re = Regex::new(r"[a-zA-Z0-9_*]").unwrap();
        for i in self.location..self.query.len() {
            if !re.is_match(self.query.chars().nth(i).unwrap().to_string().as_str()) {
                return (
                    self.query[self.location..i].to_string(),
                    self.query[self.location..i].len(),
                );
            }
        }
        (
            self.query[self.location..].to_string(),
            self.query[self.location..].len(),
        )
    }
}
