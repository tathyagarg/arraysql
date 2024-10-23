pub mod ast;
pub mod step;

pub struct Parse {
    query: String,
    location: usize,
    ast: ast::AST,
    step: u32,
}

impl Parse {
    pub fn parse(&mut self) {}

    pub fn peek(&mut self) {}

    pub fn pop(&mut self) {}
}
