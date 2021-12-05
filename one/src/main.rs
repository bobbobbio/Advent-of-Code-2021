use advent::prelude::*;

#[part_one]
fn part_one(numbers: Vec<u32>) -> u32 {
    let iter_a = numbers.iter();
    let iter_b = numbers.iter().skip(1);
    iter_a
        .zip(iter_b)
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum()
}

#[part_two]
fn part_two(numbers: Vec<u32>) -> u32 {
    let iter_a = numbers.iter();
    let iter_b = numbers.iter().skip(1);
    let iter_c = numbers.iter().skip(2);
    let iter = iter_a.zip(iter_b).zip(iter_c).map(|((a, b), c)| a + b + c);

    part_one(iter.collect::<Vec<u32>>())
}

harness!();
