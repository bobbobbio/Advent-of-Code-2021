#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use advent::prelude::*;

fn run_simluation(starting_fish: List<usize, Comma>, days: usize) -> usize {
    let mut current_gen = [0usize; 9];
    let mut next_gen = [0usize; 9];

    for f in starting_fish {
        current_gen[f] += 1;
    }

    for _day in 0..days {
        for i in 0..current_gen.len() {
            if current_gen[i] == 0 {
                continue;
            }

            if i == 0 {
                next_gen[8] += current_gen[i];
                next_gen[6] += current_gen[i];
            } else {
                next_gen[i - 1] += current_gen[i];
            }
        }
        std::mem::swap(&mut current_gen, &mut next_gen);
        next_gen = [0; 9];
    }
    current_gen.into_iter().sum()
}

#[part_one]
fn part_one(fish: List<usize, Comma>) -> usize {
    run_simluation(fish, 80)
}

#[part_two]
fn part_two(fish: List<usize, Comma>) -> usize {
    run_simluation(fish, 256)
}

harness!();
