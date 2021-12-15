#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use enum_iterator::IntoEnumIterator;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<u64>>,
    scale: usize,
}

impl HasParser for Board {
    #[into_parser]
    fn parser() -> _ {
        many1(many1(digit().map(|d| d.to_string().parse::<u64>().unwrap())).skip(token('\n')))
            .map(|grid| Self { grid, scale: 1 })
    }
}

impl Board {
    fn unscaled_height(&self) -> usize {
        self.grid[0].len()
    }

    fn unscaled_width(&self) -> usize {
        self.grid.len()
    }

    fn height(&self) -> usize {
        self.unscaled_height() * self.scale
    }

    fn width(&self) -> usize {
        self.unscaled_width() * self.scale
    }

    fn get(&self, x: usize, y: usize) -> u64 {
        let b_x = x / self.unscaled_width();
        let b_y = y / self.unscaled_height();

        let v = self.grid[y % self.unscaled_height()][x % self.unscaled_width()];
        ((v - 1 + (b_x + b_y) as u64) % 9) + 1
    }

    fn try_get_adjacent(&self, x: usize, y: usize, d: Direction) -> Option<(usize, usize)> {
        match d {
            Direction::Up => (y > 0).then(|| (x, y - 1)),
            Direction::Down => (y < self.height() - 1).then(|| (x, y + 1)),
            Direction::Left => (x > 0).then(|| (x - 1, y)),
            Direction::Right => (x < self.width() - 1).then(|| (x + 1, y)),
        }
    }

    fn all_adjacent<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'a {
        Direction::into_enum_iter()
            .map(move |d| self.try_get_adjacent(x, y, d))
            .flatten()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode((usize, usize), u64);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_path_cost(b: &Board) -> u64 {
    let endx = b.width() - 1;
    let endy = b.height() - 1;
    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![u64::MAX; b.width()]; b.height()];

    heap.push(HeapNode((endx, endy), 0));

    while let Some(HeapNode((x, y), mut cost)) = heap.pop() {
        if (x, y) == (0, 0) {
            return cost;
        }

        if cost > dist[y][x] {
            continue;
        }

        cost += b.get(x, y);
        for (nx, ny) in b.all_adjacent(x, y) {
            if cost < dist[ny][nx] {
                heap.push(HeapNode((nx, ny), cost));
                dist[ny][nx] = cost;
            }
        }
    }

    panic!();
}

#[derive(IntoEnumIterator)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[part_one]
fn part_one(b: Board) -> u64 {
    min_path_cost(&b)
}

#[part_two]
fn part_two(mut b: Board) -> u64 {
    b.scale = 5;
    min_path_cost(&b)
}

harness!();
