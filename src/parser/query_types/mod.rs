pub mod inserting;
pub mod reading;
pub mod table_creation;

pub mod type_checker {
    use crate::parser::{
        constants::{BINOPS, OPERATORS, UNOPS},
        query::Identifier,
    };

    ///  Splits a token representing an array into a vec of it's tokens (in String form)
    ///
    ///  # Example
    ///  ```
    ///  assert_eq!(maybe_sql::parser::query_types::type_checker::split_array(&"['a', 'bc', 'def']".to_string()), vec!["'a'", "'bc'", "'def'"]);
    ///  assert_eq!(maybe_sql::parser::query_types::type_checker::split_array(&"[1, 2, 3]".to_string()), vec!["1", "2", "3"]);
    ///  ```
    pub fn split_array(token: &String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut curr = String::new();
        let mut quotes = false;

        let characters = token.chars().collect::<Vec<char>>();

        for i in 1..(characters.len() - 1) {
            if characters[i] == '\'' && characters[i - 1] != '\\' {
                if !quotes {
                    quotes = true;
                    curr.push(characters[i]);
                } else {
                    quotes = false;
                    curr.push(characters[i]);
                    res.push(curr);
                    curr = String::new();
                }
            } else {
                if !quotes && characters[i] != ',' && characters[i] != ' ' {
                    curr.push(characters[i]);
                } else if !quotes && characters[i] == ',' && !curr.is_empty() {
                    res.push(curr);
                    curr = String::new();
                } else if quotes {
                    curr.push(characters[i]);
                }
            }
        }

        if !curr.is_empty() {
            res.push(curr);
        }

        res
    }

    pub fn check_int_literal(token: &String) -> bool {
        for character in token.chars() {
            if !character.is_digit(10) {
                return false;
            }
        }
        true
    }

    pub fn check_string_literal(token: &String) -> bool {
        token.starts_with("'") && token.ends_with("'")
    }

    pub fn check_operator(token: &String) -> bool {
        OPERATORS.contains(&token.as_str())
    }

    pub fn check_binop(token: &String) -> bool {
        BINOPS.contains(&token.as_str())
    }

    pub fn check_unop(token: &String) -> bool {
        UNOPS.contains(&token.as_str())
    }

    pub fn check_identifier(token: &String) -> bool {
        check_int_literal(&token) || check_string_literal(&token) || !check_operator(token)
    }

    pub fn check_field(token: &String) -> bool {
        !(check_int_literal(&token) || check_string_literal(&token) || check_operator(&token))
    }

    pub fn check_array(token: &String) -> bool {
        token.starts_with("[") && token.ends_with("]")
    }

    pub fn as_identifier(token: &String) -> Identifier {
        if check_int_literal(token) {
            Identifier::IntLiteral(token.parse::<i32>().unwrap())
        } else if check_string_literal(token) {
            Identifier::StringLiteral(token.clone())
        } else if check_array(token) {
            let mut res: Vec<Identifier> = Vec::new();
            for token in split_array(&token.clone()) {
                res.push(as_identifier(&token));
            }
            Identifier::Array(res)
        } else if !check_operator(token) {
            Identifier::Field(token.clone())
        } else {
            panic!("Given token {} is not an identifier", token)
        }
    }
}
