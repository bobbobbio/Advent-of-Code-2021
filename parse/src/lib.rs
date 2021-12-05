use combine::stream::{easy, position};
use prelude::*;
use std::convert::Infallible;
use std::{
    io, iter, num,
    ops::{Deref, DerefMut},
    result, slice, str, vec,
};

pub mod prelude {
    pub use super::*;
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

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
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

pub fn u32_parser<Input>() -> impl Parser<Input, Output = u32>
where
    Input: Stream<Token = char>,
{
    many1(digit()).map(|s: String| s.parse::<u32>().unwrap())
}

#[derive(Clone, Debug)]
pub struct List<T>(Vec<T>);

impl List<u32> {
    #[into_parser]
    pub fn parser() -> _ {
        sep_by1(u32_parser(), token(',')).map(Self)
    }
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.0.iter_mut()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> str::FromStr for List<T>
where
    T: str::FromStr,
{
    type Err = <T as str::FromStr>::Err;

    fn from_str(lines: &str) -> result::Result<Self, Self::Err> {
        let mut values = vec![];
        for line in lines.lines() {
            values.push(line.parse()?);
        }
        Ok(Self(values))
    }
}

impl<T> iter::FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter::FromIterator::from_iter(iter))
    }
}

impl<T> AsRef<[T]> for List<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> AsMut<[T]> for List<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T> Deref for List<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.0.deref()
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.deref_mut()
    }
}
