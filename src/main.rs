mod parser;

use std::rc::Rc;

use parser::common::*;
use parser::*;

#[derive(Debug)]
enum Literals {
    A,
    B,
}

fn main() {
    let a_parser = LiteralParser::new("a").map(Rc::new(|_| Literals::A));
    let b_parser = LiteralParser::new("b").map(Rc::new(|_| Literals::B));
    let char_parser = b_parser.or(a_parser);
    let input = ParserInput::new("abs".to_string());
    let result = char_parser.parse(input);
    println!("{:?}", result);
}
