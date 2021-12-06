#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use advent::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl HasParser for Direction {
    #[into_parser]
    fn parser() -> _ {
        let dir = string("forward").or(string("up")).or(string("down"));
        let num = many1(digit()).map(|n: String| n.parse::<u32>().unwrap());
        (dir, spaces(), num).map(|(d, _, n)| match d {
            "forward" => Self::Forward(n),
            "up" => Self::Up(n),
            "down" => Self::Down(n),
            _ => unreachable!(),
        })
    }
}

#[part_one]
fn part_one(directions: List<Direction, NewLine>) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    for d in directions {
        match d {
            Direction::Forward(n) => pos += n,
            Direction::Up(n) => depth -= n,
            Direction::Down(n) => depth += n,
        }
    }
    pos * depth
}

#[part_two]
fn part_two(directions: List<Direction, NewLine>) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for d in directions {
        match d {
            Direction::Forward(n) => {
                pos += n;
                depth += aim * n;
            }
            Direction::Up(n) => aim -= n,
            Direction::Down(n) => aim += n,
        }
    }
    pos * depth
}

harness!();
