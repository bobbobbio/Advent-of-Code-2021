#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;

fn sum_numbers(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + sum_numbers(n - 1)
    }
}

fn alignment_cost_part1(pos: u32, crabs: &List<u32, Comma>) -> u32 {
    crabs
        .iter()
        .map(|&c| if pos > c { pos - c } else { c - pos })
        .sum()
}

fn alignment_cost_part2(pos: u32, crabs: &List<u32, Comma>) -> u32 {
    crabs
        .iter()
        .map(|&c| sum_numbers(if pos > c { pos - c } else { c - pos }))
        .sum()
}

fn best_cost(crabs: List<u32, Comma>, cost_func: impl Fn(u32, &List<u32, Comma>) -> u32) -> u32 {
    let mut best_cost = u32::MAX;
    let max = *crabs.iter().max().unwrap();
    for pos in 0..max {
        let cost = cost_func(pos, &crabs);
        if cost < best_cost {
            best_cost = cost;
        }
    }
    best_cost
}

#[part_one]
fn part_one(crabs: List<u32, Comma>) -> u32 {
    best_cost(crabs, alignment_cost_part1)
}

#[part_two]
fn part_two(crabs: List<u32, Comma>) -> u32 {
    best_cost(crabs, alignment_cost_part2)
}

harness!();
