use query::Expression;
use query::Identifier;
use query::QueryType;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

pub mod constants;
pub mod query;
pub mod query_types;
pub mod step;

use constants::*;
use query_types::inserting;
use query_types::reading;
use query_types::table_creation;
use step::Step;

#[derive(Debug)]
pub struct Parser {
    pub query: String,
    pub location: usize,
    pub query_data: query::Query,
    pub step: Step,
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            query: String::new(),
            location: 0,
            query_data: query::Query {
                _type: QueryType::None,
                db_name: Identifier::StringLiteral(String::new()),
                table_name: Identifier::StringLiteral(String::new()),
                fields: Vec::new(),
                modes: Vec::new(),
                constraints: HashMap::new(),
                curr_constraint: Identifier::StringLiteral(String::new()),
                inserted_value: Vec::new(),
                inserted_field: Vec::new(),
                read_fields: Vec::new(),
                conditions: Expression::None,
            },
            step: Step::Start,
        }
    }

    fn ensure_token(&self, token: String, expected: &str) -> u8 {
        if token != expected {
            panic!(
                "Unexpected token found:\n\tFound   : {:?}\n\tExpected: {:?}\nQuery: {}\n       {}^{}",
                token,
                expected,
                self.query,
                (0..(self.location)).map(|_| " ").collect::<String>(),
                (0..(self.query.len() - self.location)).map(|_| " ").collect::<String>()
            );
        }
        0
    }

    pub fn reset(&mut self) {
        *self = Parser::new();
    }

    pub fn set_query(&mut self, query: String) -> &mut Parser {
        self.query = query;
        self
    }

    pub fn parse(&mut self) {
        let (mut curr, func): (Step, fn(&mut Parser, Step) -> Step) = match self.pop().as_str() {
            DATABASE => {
                self.query_data._type = QueryType::DatabaseCreation;
                let identifier = self.pop_identifier();
                if identifier.is_empty() {
                    panic!("Expected identifier for database name.");
                }

                self.query_data.db_name = Identifier::StringLiteral(identifier);

                {
                    let token = self.pop();
                    self.ensure_token(token, SEMICOLON);
                }
                return;
            }
            TABLE => {
                self.query_data._type = QueryType::TableCreation;
                (Step::DefineTableName, table_creation::table_creation)
            }
            INSERT => {
                self.query_data._type = QueryType::Insert;
                {
                    let token = self.pop();
                    self.ensure_token(token, STRUCTURED);

                    let token = self.pop();
                    self.ensure_token(token, OPEN_PAREN);
                }
                (Step::InsertValueIdentifier, inserting::inserting)
            }
            READ => {
                self.query_data._type = QueryType::Read;

                {
                    let token = self.pop();
                    self.ensure_token(token, STRUCTURED);

                    let token = self.pop();
                    self.ensure_token(token, OPEN_PAREN);
                }
                (Step::ReadFieldIdentifier, reading::reading)
            }
            found => {
                panic!("Invalid token found {:?}", found);
            }
        };
        while curr != Step::End {
            curr = func(self, curr);
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
        self.peek_string_or_indentifier_with_length()
    }

    fn pop_string_or_identifier(&mut self) -> String {
        let (res, len) = self.peek_string_or_indentifier_with_length();
        self.location += len;
        println!(
            "Loc: {} {} {:?}",
            self.location,
            self.query,
            &self.query[self.location..self.location + 2]
        );
        self.pop_whitespace();

        res
    }

    fn peek_string_or_indentifier_with_length(&self) -> (String, usize) {
        if self.query.chars().nth(self.location).unwrap() == '[' {
            return self.peek_array_with_length();
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
                println!(
                    "Returning string {} {}",
                    &self.query[self.location..i + 1],
                    self.query.clone().chars().nth(i).unwrap()
                );
                return (
                    self.query[self.location..i + 1].to_string(),
                    // Why did I remove the + 2?
                    // No fucking clue.
                    self.query[self.location..i + 1].len(), // + 2,
                );
            }
        }
        (String::new(), 0)
    }

    fn peek_array_with_length(&self) -> (String, usize) {
        if self.query.len() < self.location || self.query.chars().nth(self.location).unwrap() != '['
        {
            return (String::new(), 0);
        }

        for i in (self.location + 1)..(self.query.len()) {
            if self.query.chars().nth(i).unwrap() == ']'
                && self.query.chars().nth(i - 1).unwrap() != '\\'
            {
                return (
                    self.query[self.location..i + 1].to_string(),
                    self.query[self.location..i + 1].len(),
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
