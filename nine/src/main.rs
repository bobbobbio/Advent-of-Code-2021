#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use enum_iterator::IntoEnumIterator;
use std::collections::HashSet;

#[derive(Debug)]
struct Floor {
    grid: Vec<Vec<u8>>,
}

#[derive(IntoEnumIterator)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Floor {
    fn positions(&self) -> impl Iterator<Item = (usize, usize)> + 'static {
        let height = self.height();
        let width = self.width();
        (0..height)
            .map(move |y| (0..width).map(move |x| (x, y)))
            .flatten()
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y][x]
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn try_get_adjacent(&self, x: usize, y: usize, d: Direction) -> Option<(usize, usize)> {
        match d {
            Direction::Up => (y > 0).then(|| (x, y - 1)),
            Direction::Down => (y < self.height() - 1).then(|| (x, y + 1)),
            Direction::Left => (x > 0).then(|| (x - 1, y)),
            Direction::Right => (x < self.width() - 1).then(|| (x + 1, y)),
        }
    }

    fn all_adjacent<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = u8> + 'a {
        Direction::into_enum_iter()
            .map(move |d| self.try_get_adjacent(x, y, d).map(|(x, y)| self.get(x, y)))
            .flatten()
    }

    fn low_points<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.positions().filter(|&(x, y)| {
            let v = self.get(x, y);
            self.all_adjacent(x, y).all(|nv| nv > v)
        })
    }

    fn basin_size_helper(&self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) {
        if !visited.contains(&(x, y)) {
            visited.insert((x, y));

            let good_directions = Direction::into_enum_iter()
                .map(|d| self.try_get_adjacent(x, y, d))
                .flatten()
                .filter(|&(x, y)| self.get(x, y) < 9);

            for (x, y) in good_directions {
                self.basin_size_helper(x, y, visited);
            }
        }
    }

    fn basin_size(&self, x: usize, y: usize) -> u32 {
        let mut visited = HashSet::new();
        self.basin_size_helper(x, y, &mut visited);
        visited.len() as u32
    }
}

impl HasParser for Floor {
    #[into_parser]
    fn parser() -> _ {
        let line = many1(digit().map(|d| d.to_string().parse::<u8>().unwrap()));
        many1(line.skip(token('\n'))).map(|grid| Self { grid })
    }
}

#[part_one]
fn part_one(floor: Floor) -> u32 {
    let mut score = 0;
    for (x, y) in floor.low_points() {
        score += 1 + floor.get(x, y) as u32;
    }
    score
}

#[part_two]
fn part_two(floor: Floor) -> u32 {
    let mut basin_sizes: Vec<_> = floor
        .low_points()
        .map(|(x, y)| floor.basin_size(x, y))
        .collect();
    basin_sizes.sort();
    basin_sizes.into_iter().rev().take(3).product()
}

harness!();
