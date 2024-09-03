use super::*;

use std::rc::Rc;

pub struct LiteralParser {
    pub literal: &'static str,
}

impl LiteralParser {
    pub fn new(literal: &'static str) -> Rc<Self> {
        Rc::new(LiteralParser { literal })
    }
}

impl Parser<String> for LiteralParser {
    fn parse(&self, input: ParserInput) -> ParserOutput<String> {
        let input_str = input.input;
        if input_str.starts_with(self.literal) {
            ParserOutput::Result(
                self.literal.to_string(),
                ParserInput {
                    input: input_str[self.literal.len()..].to_string(),
                    location: Location {
                        line: input.location.line,
                        column: input.location.column + self.literal.len() as u32,
                    },
                },
            )
        } else {
            ParserOutput::Error(
                input.location,
                format!("Expected literal: {}", self.literal),
            )
        }
    }
}
