use std::io::{self, BufRead};
use std::num;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    ParseInt(num::ParseIntError),
    Io(io::Error),
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

type Result<T> = std::result::Result<T, Error>;

fn part_one(numbers: &[u32]) {
    let iter_a = numbers.iter();
    let iter_b = numbers.iter().skip(1);
    let result: u32 = iter_a
        .zip(iter_b)
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();
    println!("{}", result);
}

fn part_two(numbers: &[u32]) {
    let iter_a = numbers.iter();
    let iter_b = numbers.iter().skip(1);
    let iter_c = numbers.iter().skip(2);
    let iter = iter_a.zip(iter_b).zip(iter_c).map(|((a, b), c)| a + b + c);

    part_one(&iter.collect::<Vec<u32>>());
}

fn parse_lines<R: BufRead, T: FromStr>(lines: R) -> Result<Vec<T>>
where
    Error: From<<T as FromStr>::Err>,
{
    let mut values = vec![];
    for maybe_line in lines.lines() {
        values.push(maybe_line?.parse()?);
    }
    Ok(values)
}

fn main() -> Result<()> {
    let numbers: Vec<u32> = parse_lines(io::stdin().lock())?;

    println!("Part 1");
    part_one(&numbers);

    println!("Part 2");
    part_two(&numbers);

    Ok(())
}
