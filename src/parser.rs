pub mod common;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ParserInput {
    pub input: String,
    pub location: Location,
}

impl ParserInput {
    pub fn new(input: String) -> Self {
        ParserInput {
            input,
            location: Location { line: 0, column: 0 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub enum ParserOutput<T> {
    Result(T, ParserInput),
    Error(Location, String),
}

pub trait Parser<T> {
    fn parse(&self, input: ParserInput) -> ParserOutput<T>;
}

impl<T: 'static, F> Parser<T> for F
where
    F: Fn(ParserInput) -> ParserOutput<T>,
{
    fn parse(&self, input: ParserInput) -> ParserOutput<T> {
        self(input)
    }
}

pub trait ParserCombinators<T: 'static>: Parser<T> + 'static {
    fn map<U: 'static>(self: Rc<Self>, f: Rc<dyn Fn(T) -> U>) -> Rc<dyn Parser<U>> {
        Rc::new(move |input: ParserInput| match self.parse(input) {
            ParserOutput::Result(value, remaining) => ParserOutput::Result(f(value), remaining),
            ParserOutput::Error(location, msg) => ParserOutput::Error(location, msg),
        })
    }

    fn then<U: 'static>(self: Rc<Self>, next: Rc<dyn Parser<U>>) -> Rc<dyn Parser<(T, U)>> {
        Rc::new(move |input: ParserInput| match self.parse(input.clone()) {
            ParserOutput::Result(value, remaining) => match next.parse(remaining) {
                ParserOutput::Result(next_value, final_input) => {
                    ParserOutput::Result((value, next_value), final_input)
                }
                ParserOutput::Error(location, msg) => ParserOutput::Error(location, msg),
            },
            ParserOutput::Error(location, msg) => ParserOutput::Error(location, msg),
        })
    }

    fn or(self: Rc<Self>, alternative: Rc<dyn Parser<T>>) -> Rc<dyn Parser<T>> {
        Rc::new(move |input: ParserInput| match self.parse(input.clone()) {
            success @ ParserOutput::Result(_, _) => success,
            ParserOutput::Error(_, _) => alternative.parse(input),
        })
    }
}

impl<T: 'static, P: ?Sized + 'static> ParserCombinators<T> for P where P: Parser<T> {}
