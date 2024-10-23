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

pub const KEYWORDS: &[&str] = &[
    "(", ")", "=", ">", "<", "!=", ">=", "<=", ",", "DATABASE", "TABLE",
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
                Step::Start => match self.peek().to_uppercase().as_str() {
                    "DATABASE" => {
                        self.query_data._type = QueryType::DatabaseCreation;
                        self.pop();
                        self.step = Step::DatabaseName;
                    }
                    _ => {
                        panic!("Invalid token found {}", self.peek());
                    }
                },
                Step::DatabaseName => {
                    let identifier = self.peek_identifier();
                    if identifier.is_empty() {
                        panic!("Expected identifier for database name.");
                    }

                    self.query_data.db_name = identifier;
                    self.step = Step::End;
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
