#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use enum_iterator::IntoEnumIterator;
use enumset::{EnumSet, EnumSetType};
use std::collections::HashMap;

#[derive(EnumSetType, IntoEnumIterator, Debug, Hash)]
#[repr(usize)]
enum CodedSegment {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl HasParser for CodedSegment {
    #[into_parser]
    fn parser() -> _ {
        choice((
            char('a').map(|_| Self::A),
            char('b').map(|_| Self::B),
            char('c').map(|_| Self::C),
            char('d').map(|_| Self::D),
            char('e').map(|_| Self::E),
            char('f').map(|_| Self::F),
            char('g').map(|_| Self::G),
        ))
    }
}

#[derive(Debug)]
struct Digit(EnumSet<CodedSegment>);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        many1(CodedSegment::parser()).map(|c: Vec<CodedSegment>| Self(c.into_iter().collect()))
    }
}

#[derive(Debug)]
struct Input {
    signals: Vec<Digit>,
    outputs: Vec<Digit>,
}

impl HasParser for Input {
    #[into_parser]
    fn parser() -> _ {
        let signals = many1(Digit::parser().skip(token(' ')));
        let outputs = sep_by1(Digit::parser(), token(' '));
        (signals.skip(string("| ")), outputs).map(|(signals, outputs)| Self { signals, outputs })
    }
}

#[part_one]
fn part_one(inputs: List<Input, NewLine>) -> u32 {
    let mut total = 0;
    for i in inputs {
        for o in i.outputs {
            if numbers_with_len(o.0.len()).len() == 1 {
                total += 1;
            }
        }
    }
    total
}

#[derive(EnumSetType, IntoEnumIterator, Debug, Hash, PartialOrd, Ord)]
enum Segment {
    Bottom,
    LowerLeft,
    LowerRight,
    Middle,
    Top,
    UpperLeft,
    UpperRight,
}

use Segment::*;

const ZERO: [Segment; 6] = [Bottom, LowerLeft, LowerRight, Top, UpperLeft, UpperRight];
const ONE: [Segment; 2] = [LowerRight, UpperRight];
const TWO: [Segment; 5] = [Bottom, LowerLeft, Middle, Top, UpperRight];
const THREE: [Segment; 5] = [Bottom, LowerRight, Middle, Top, UpperRight];
const FOUR: [Segment; 4] = [LowerRight, Middle, UpperLeft, UpperRight];
const FIVE: [Segment; 5] = [Bottom, LowerRight, Middle, Top, UpperLeft];
const SIX: [Segment; 6] = [Bottom, LowerLeft, LowerRight, Middle, Top, UpperLeft];
const SEVEN: [Segment; 3] = [LowerRight, Top, UpperRight];
const EIGHT: [Segment; 7] = [
    Bottom, LowerLeft, LowerRight, Middle, Top, UpperLeft, UpperRight,
];
const NINE: [Segment; 6] = [Bottom, LowerRight, Middle, Top, UpperLeft, UpperRight];

const ALL_DIGITS: [&'static [Segment]; 10] = [
    &ZERO, &ONE, &TWO, &THREE, &FOUR, &FIVE, &SIX, &SEVEN, &EIGHT, &NINE,
];

fn number_for_segments(segments: &EnumSet<Segment>) -> Option<u32> {
    let mut seg: Vec<Segment> = segments.iter().collect();
    seg.sort();
    ALL_DIGITS.iter().position(|s| s == &seg).map(|v| v as u32)
}

fn segments_for_number(n: u32) -> EnumSet<Segment> {
    ALL_DIGITS[n as usize].iter().cloned().collect()
}

#[derive(Debug, Default, Clone)]
struct Key(HashMap<CodedSegment, Segment>);

impl Key {
    fn complete(&self) -> bool {
        self.0.len() == 7
    }

    fn try_decode(&self, d: &Digit) -> Option<u32> {
        let mut m = EnumSet::new();
        for c in d.0.iter() {
            if let Some(v) = self.0.get(&c) {
                m.insert(*v);
            } else {
                return None;
            }
        }
        number_for_segments(&m)
    }
}

fn numbers_with_len(len: usize) -> Vec<u32> {
    ALL_DIGITS
        .iter()
        .enumerate()
        .filter_map(|(n, d)| (d.len() == len).then(|| n as u32))
        .collect()
}

fn possible_segments_for_input(c: CodedSegment, input: &[Digit]) -> EnumSet<Segment> {
    let mut segments: EnumSet<_> = EIGHT.iter().cloned().collect();
    for d in input {
        if d.0.contains(c) {
            let mut segments_for_input = EnumSet::new();
            for n in numbers_with_len(d.0.len()) {
                segments_for_input = segments_for_input.union(segments_for_number(n));
            }

            segments = segments.intersection(segments_for_input);
        }
    }
    segments
}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    chosen_keys: EnumSet<CodedSegment>,
    chosen_values: EnumSet<Segment>,
}

fn solve_key(input: &Input, key: Key) -> Option<Key> {
    if key.complete() {
        for o in &input.outputs {
            if key.try_decode(o).is_none() {
                return None;
            }
        }
        return Some(key);
    }

    for c in CodedSegment::into_enum_iter() {
        if key.0.contains_key(&c) {
            continue;
        }
        for s in possible_segments_for_input(c, &input.signals) {
            if key.0.values().find(|&e| &s == e).is_some() {
                continue;
            }

            let mut new_key = key.clone();
            new_key.0.insert(c, s);
            let res = solve_key(input, new_key);

            if res.is_some() {
                return res;
            }
        }
    }
    None
}

fn solve(input: Input) -> u32 {
    let key = solve_key(&input, Key::default()).unwrap();
    let mut n = 0;
    for o in input.outputs {
        n += key.try_decode(&o).unwrap();
        n *= 10;
    }
    n / 10
}

#[part_two]
fn part_two(inputs: List<Input, NewLine>) -> u32 {
    inputs.into_iter().map(solve).sum()
}

harness!();
