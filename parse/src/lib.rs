use combine::stream::{easy, position};
use prelude::*;
use std::io::{self, BufRead};
use std::num;

pub mod prelude {
    pub use combine::parser::char::*;
    pub use combine::*;
    pub use combine::{Parser, Stream};
    pub use parse_macro::into_parser;
    pub use std::str::FromStr;
}

#[derive(Debug)]
pub enum Error {
    ParseInt(num::ParseIntError),
    Io(io::Error),
    ParseError(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<easy::Errors<char, &str, position::SourcePosition>> for Error {
    fn from(e: easy::Errors<char, &str, position::SourcePosition>) -> Self {
        Self::ParseError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! parser_from_str {
    ($s:ident) => {
        impl ::std::str::FromStr for $s {
            type Err = $crate::Error;
            fn from_str(input: &str) -> $crate::Result<Self> {
                let (p, _): (Self, _) = Self::parser()
                    .skip(::combine::eof())
                    .easy_parse(::combine::stream::position::Stream::new(input))?;
                Ok(p)
            }
        }
    };
}

pub fn parse_lines<R: BufRead, T: FromStr>(lines: R) -> Result<Vec<T>>
where
    Error: From<<T as FromStr>::Err>,
{
    let mut values = vec![];
    for maybe_line in lines.lines() {
        values.push(maybe_line?.parse()?);
    }
    Ok(values)
}

pub fn parse_from_reader<T: FromStr>(mut reader: impl io::Read) -> Result<T>
where
    Error: From<<T as FromStr>::Err>,
{
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s.parse()?)
}

pub fn u32_parser<Input>() -> impl Parser<Input, Output = u32>
where
    Input: Stream<Token = char>,
{
    many1(digit()).map(|s: String| s.parse::<u32>().unwrap())
}
