use parse::{parse_lines, Result};
use std::io;

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

fn main() -> Result<()> {
    let numbers: Vec<u32> = parse_lines(io::stdin().lock())?;

    println!("Part 1");
    part_one(&numbers);

    println!("Part 2");
    part_two(&numbers);

    Ok(())
}
