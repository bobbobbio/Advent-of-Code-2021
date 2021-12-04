use parse::{parse_lines, parser_from_str, prelude::*, Result};
use std::io;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Direction {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
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

parser_from_str!(Direction);

fn part_one(directions: &[Direction]) {
    let mut pos = 0;
    let mut depth = 0;
    for d in directions {
        match d {
            Direction::Forward(n) => pos += n,
            Direction::Up(n) => depth -= n,
            Direction::Down(n) => depth += n,
        }
    }
    println!("{}", pos * depth);
}

fn part_two(directions: &[Direction]) {
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
    println!("{}", pos * depth);
}

fn main() -> Result<()> {
    let directions: Vec<Direction> = parse_lines(io::stdin().lock())?;

    println!("Part 1");
    part_one(&directions);

    println!("Part 2");
    part_two(&directions);

    Ok(())
}
