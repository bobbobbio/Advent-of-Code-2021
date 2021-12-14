#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    src: [char; 2],
    dst: char,
}

impl HasParser for Rule {
    #[into_parser]
    fn parser() -> _ {
        let src = (upper(), upper()).skip(string(" -> ")).map(|(a, b)| [a, b]);
        let dst = upper();
        (src, dst).map(|(src, dst)| Self { src, dst })
    }
}

#[derive(Debug)]
struct Input {
    template: String,
    rules: List<Rule, NewLine>,
}

impl HasParser for Input {
    #[into_parser]
    fn parser() -> _ {
        let template = many1(upper()).skip(string("\n\n"));
        (template, List::<Rule, NewLine>::parser())
            .map(|(template, rules)| Self { template, rules })
    }
}

#[derive(Default, Clone)]
struct Stats([u64; 26]);

impl Stats {
    fn new() -> Self {
        Self([0; 26])
    }

    fn union(&mut self, other: &Self) {
        for (d, s) in self.0.iter_mut().zip(other.0.iter()) {
            *d += *s;
        }
    }

    fn incr(&mut self, c: char) {
        self.0[(c as usize) - ('A' as usize)] += 1;
    }
}

#[memoise::memoise_map(pair, depth)]
fn apply_rules(pair: [char; 2], rules: &HashMap<[char; 2], char>, depth: u64) -> Stats {
    let mut stats = Stats::new();
    if depth > 0 {
        if let Some(&c) = rules.get(&pair) {
            stats.union(&apply_rules([pair[0], c], rules, depth - 1));
            stats.union(&apply_rules([c, pair[1]], rules, depth - 1));
            return stats;
        }
    }
    stats.incr(pair[0]);
    stats
}

fn run_polymer_steps(i: Input, n: u64) -> u64 {
    let rules: HashMap<_, _> = i.rules.into_iter().map(|r| (r.src, r.dst)).collect();
    let mut stats = Stats::new();
    let cs: Vec<_> = i.template.chars().collect();
    for pair in cs.windows(2) {
        stats.union(&apply_rules(pair.try_into().unwrap(), &rules, n));
    }
    stats.incr(*cs.last().unwrap());

    let mut v: Vec<_> = stats.0.iter().filter(|&&v| v != 0).collect();
    v.sort();
    *v.last().unwrap() - v[0]
}

#[part_one]
fn part_one(i: Input) -> u64 {
    run_polymer_steps(i, 10)
}

#[part_two]
fn part_two(i: Input) -> u64 {
    run_polymer_steps(i, 40)
}

harness!();
