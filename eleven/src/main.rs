#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use enum_iterator::IntoEnumIterator;
use std::{fmt, mem};

#[derive(IntoEnumIterator)]
enum Direction {
    Down,
    Left,
    LowerLeft,
    LowerRight,
    Right,
    Up,
    UpperLeft,
    UpperRight,
}

struct Cavern {
    grid: Vec<Vec<u8>>,
    total_flashes: u64,
}

impl fmt::Debug for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &n in row {
                write!(f, "{}", n)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Cavern {
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

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.grid[y][x]
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
            Direction::UpperLeft => (x > 0 && y > 0).then(|| (x - 1, y - 1)),
            Direction::UpperRight => (x < self.width() - 1 && y > 0).then(|| (x + 1, y - 1)),
            Direction::LowerLeft => (x > 0 && y < self.height() - 1).then(|| (x - 1, y + 1)),
            Direction::LowerRight => {
                (x < self.width() - 1 && y < self.height() - 1).then(|| (x + 1, y + 1))
            }
        }
    }

    fn all_adjacent<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'a {
        Direction::into_enum_iter()
            .map(move |d| self.try_get_adjacent(x, y, d))
            .flatten()
    }

    fn simulate(&mut self) {
        for (x, y) in self.positions() {
            *self.get_mut(x, y) += 1;
        }

        let mut pos: Vec<_> = self
            .positions()
            .filter(|&(x, y)| self.get(x, y) > 9)
            .collect();
        while !pos.is_empty() {
            for (x, y) in mem::take(&mut pos) {
                let adj: Vec<_> = self
                    .all_adjacent(x, y)
                    .filter(|&(x, y)| self.get(x, y) < 10)
                    .collect();
                for (ax, ay) in adj {
                    *self.get_mut(ax, ay) += 1;
                    if self.get(ax, ay) > 9 {
                        pos.push((ax, ay))
                    }
                }
            }
        }

        for (x, y) in self.positions() {
            if self.get(x, y) > 9 {
                self.total_flashes += 1;
                *self.get_mut(x, y) = 0;
            }
        }
    }
}

impl HasParser for Cavern {
    #[into_parser]
    fn parser() -> _ {
        let line = many1(digit().map(|d| d.to_string().parse::<u8>().unwrap()));
        many1(line.skip(token('\n'))).map(|grid| Self {
            grid,
            total_flashes: 0,
        })
    }
}

#[part_one]
fn part_one(mut cavern: Cavern) -> u64 {
    for _ in 0..100 {
        cavern.simulate();
    }
    cavern.total_flashes
}

#[part_two]
fn part_two(mut cavern: Cavern) -> u64 {
    let mut step = 1;
    loop {
        cavern.simulate();
        if cavern.positions().all(|(x, y)| cavern.get(x, y) == 0) {
            break step;
        }
        step += 1;
    }
}

harness!();
