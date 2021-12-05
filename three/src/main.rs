use parse::{parse_lines, parser_from_str, prelude::*, Result};
use std::io;

const NUM_BITS: usize = 12;

#[derive(Default, Debug, Clone, Copy)]
struct Number([u8; NUM_BITS]);

impl Number {
    #[into_parser]
    fn parser() -> _ {
        let one_or_zero = char('1')
            .or(char('0'))
            .map(|c| if c == '1' { 1 } else { 0 });
        let num = count_min_max(NUM_BITS, NUM_BITS, one_or_zero);
        num.map(|n: Vec<u8>| Self(n.try_into().unwrap()))
    }
}

parser_from_str!(Number);

impl Number {
    fn to_decimal(&self) -> u32 {
        let mut n = 0;
        for &bit in &self.0 {
            n <<= 1;
            n |= bit as u32;
        }
        n
    }

    fn to_flipped(&self) -> Self {
        let mut new = self.clone();
        new.0.iter_mut().for_each(|n| *n = (*n == 0) as u8);
        new
    }
}

fn most_common_bit(numbers: &[Number], pos: usize) -> u8 {
    (numbers.iter().filter(|n| n.0[pos] == 1).count() * 2 >= numbers.len()) as u8
}

fn calculate_gamma(numbers: &[Number]) -> Number {
    let mut gamma = Number::default();
    gamma.0.iter_mut().enumerate().for_each(|(i, bit)| {
        *bit = most_common_bit(numbers, i);
    });
    gamma
}

fn part_one(numbers: &[Number]) {
    let gamma = calculate_gamma(numbers);
    let epsilon = gamma.to_flipped();

    let gamma = gamma.to_decimal();
    let epsilon = epsilon.to_decimal();

    let power = gamma * epsilon;
    println!("{}", power);
}

fn filter_numbers(numbers: &[Number], most_common: bool) -> u32 {
    let mut candidates = numbers.to_vec();
    for i in 0..NUM_BITS {
        if candidates.len() == 1 {
            break;
        }
        let mut bit_to_match = most_common_bit(&candidates, i);
        if !most_common {
            bit_to_match = (bit_to_match == 0) as u8;
        }
        candidates = candidates
            .into_iter()
            .filter(|n| n.0[i] == bit_to_match)
            .collect();
    }
    assert_eq!(candidates.len(), 1);
    candidates[0].to_decimal()
}

fn part_two(numbers: &[Number]) {
    let oxygen_gen_rating = filter_numbers(numbers, true);
    let co2_scrubber_rating = filter_numbers(numbers, false);
    println!("{}", oxygen_gen_rating * co2_scrubber_rating);
}

fn main() -> Result<()> {
    let numbers: Vec<Number> = parse_lines(io::stdin().lock())?;

    println!("Part 1");
    part_one(&numbers);

    println!("Part 2");
    part_two(&numbers);

    Ok(())
}
