#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use combine::stream::easy::{Error, Errors, Info};
use combine::stream::position;
use std::matches;

parser! {
    fn valid_chunk_parser_recurse[Input]()(Input) -> ValidChunk
        where [Input: Stream<Token = char>]
    {
        ValidChunk::parser()
    }
}

#[derive(Debug)]
enum ChunkValidationError {
    Incomplete(Option<char>),
    Corrupt(usize),
}

fn closing_char(c: char) -> bool {
    matches!(c, ')' | '}' | ']' | '>')
}

impl<'a> From<Errors<char, &'a str, position::SourcePosition>> for ChunkValidationError {
    fn from(e: Errors<char, &'a str, position::SourcePosition>) -> Self {
        if e.errors
            .iter()
            .any(|e| matches!(e, Error::Unexpected(Info::Static("end of input"))))
        {
            let c = e
                .errors
                .iter()
                .filter_map(|e| {
                    if let Error::Expected(Info::Token(c)) = e {
                        closing_char(*c).then(|| *c)
                    } else {
                        None
                    }
                })
                .next();
            Self::Incomplete(c)
        } else {
            Self::Corrupt(e.position.column as usize)
        }
    }
}

#[derive(Debug)]
struct ValidChunk;

impl HasParser for ValidChunk {
    #[into_parser]
    fn parser() -> _ {
        many(choice((
            (char('('), valid_chunk_parser_recurse(), char(')')),
            (char('<'), valid_chunk_parser_recurse(), char('>')),
            (char('['), valid_chunk_parser_recurse(), char(']')),
            (char('{'), valid_chunk_parser_recurse(), char('}')),
        )))
        .map(|_: Vec<_>| Self)
    }
}

fn completion_for(c: char) -> char {
    match c {
        '(' => ')',
        '<' => '>',
        '[' => ']',
        '{' => '}',
        _ => panic!(),
    }
}

#[derive(Clone, Debug)]
struct Chunk(String);

impl Chunk {
    fn validate(&self) -> std::result::Result<(), ChunkValidationError> {
        parse_str::<ValidChunk>(&self.0)?;
        Ok(())
    }

    fn autocomplete(&self) -> String {
        let mut new = self.clone();
        let start = new.0.len();
        while let Err(ChunkValidationError::Incomplete(c)) = new.validate() {
            if let Some(c) = c {
                new.0.push(c);
            } else {
                new.0.push(completion_for(new.0.chars().last().unwrap()));
            }
        }
        new.0[start..].into()
    }
}

impl HasParser for Chunk {
    #[into_parser]
    fn parser() -> _ {
        many1(none_of("\n".chars())).map(Self)
    }
}

fn score_for_bad_char(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn score_for_autocomplete_char(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

#[part_one]
fn part_one(lines: List<Chunk, NewLine>) -> u32 {
    let mut score = 0;
    for l in lines {
        if let Err(ChunkValidationError::Corrupt(p)) = l.validate() {
            score += score_for_bad_char(l.0.chars().nth(p - 1).unwrap());
        }
    }
    score
}

#[part_two]
fn part_two(lines: List<Chunk, NewLine>) -> u64 {
    let mut scores = vec![];
    for l in lines {
        if let Err(ChunkValidationError::Incomplete(_)) = l.validate() {
            let mut score = 0;
            let completion = l.autocomplete();
            for c in completion.chars() {
                score *= 5;
                score += score_for_autocomplete_char(c);
            }
            scores.push(score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

harness!();
