mod parser;

fn main() {
    let mut p = parser::Parser::default();
    p.set_query(String::from("DATABASE ;"));
    p.parse();

    println!("{:?}", p);
}
