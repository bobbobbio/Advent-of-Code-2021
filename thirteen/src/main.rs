#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use std::fmt;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl HasParser for Coordinate {
    #[into_parser]
    fn parser() -> _ {
        (usize::parser().skip(token(',')), usize::parser()).map(|(x, y)| Self { x, y })
    }
}

#[derive(Debug)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl HasParser for Fold {
    #[into_parser]
    fn parser() -> _ {
        let x_fold = string("x=").with(usize::parser().map(Self::AlongX));
        let y_fold = string("y=").with(usize::parser().map(Self::AlongY));
        let fold = x_fold.or(y_fold);
        string("fold along ").with(fold)
    }
}

#[derive(Debug)]
struct Input {
    coordinates: List<Coordinate, NewLine>,
    folds: List<Fold, NewLine>,
}

impl HasParser for Input {
    #[into_parser]
    fn parser() -> _ {
        let coordinates = List::<Coordinate, NewLine>::parser();
        let folds = List::<Fold, NewLine>::parser();
        (coordinates.skip(token('\n')), folds)
            .map(|(coordinates, folds)| Self { coordinates, folds })
    }
}

struct Paper {
    grid: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

impl Paper {
    fn from_input(input: Input) -> Self {
        let max_x = input
            .coordinates
            .iter()
            .map(|c| c.x as usize)
            .max()
            .unwrap();
        let max_y = input
            .coordinates
            .iter()
            .map(|c| c.y as usize)
            .max()
            .unwrap();
        let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
        for c in input.coordinates {
            grid[c.y as usize][c.x as usize] = true;
        }
        Self {
            grid,
            folds: input.folds.into_iter().rev().collect(),
        }
    }

    fn fold_x(&mut self, x: usize) {
        for row in &mut self.grid {
            let removed: Vec<_> = row.drain((x + 1)..).collect();
            for (d, s) in row.iter_mut().rev().skip(1).zip(removed.into_iter()) {
                *d |= s;
            }
        }
    }

    fn fold_y(&mut self, y: usize) {
        let removed: Vec<_> = self.grid.drain((y + 1)..).collect();
        for (dst, src) in self.grid.iter_mut().rev().skip(1).zip(removed.into_iter()) {
            for (d, s) in dst.into_iter().zip(src.into_iter()) {
                *d |= s;
            }
        }
    }

    fn fold(&mut self) -> bool {
        if let Some(f) = self.folds.pop() {
            match f {
                Fold::AlongX(x) => self.fold_x(x),
                Fold::AlongY(y) => self.fold_y(y),
            }
            true
        } else {
            false
        }
    }

    fn num_dots(&self) -> usize {
        self.grid
            .iter()
            .map(|r| r.iter())
            .flatten()
            .filter(|&&v| v)
            .count()
    }
}

impl fmt::Debug for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &col in row {
                if col {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[part_one]
fn part_one(i: Input) -> usize {
    let mut paper = Paper::from_input(i);
    paper.fold();
    paper.num_dots()
}

#[part_two]
fn part_two(i: Input) -> String {
    let mut paper = Paper::from_input(i);
    while paper.fold() {}
    format!("\n{:?}", paper)
}

harness!();
