use advent::prelude::*;
use std::collections::HashMap;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    #[into_parser]
    fn parser() -> _ {
        (u32_parser().skip(char(',')), u32_parser()).map(|(x, y)| Self {
            x: x as i32,
            y: y as i32,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

#[derive(Debug)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    #[into_parser]
    fn parser() -> _ {
        (Position::parser().skip(string(" -> ")), Position::parser())
            .map(|(start, end)| Self { start, end })
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn length(&self) -> f64 {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        (((x * x) + (y * y)) as f64).sqrt()
    }

    fn slope(&self) -> Vector {
        let length = self.length();
        let v = Vector {
            x: ((self.end.x - self.start.x) as f64 / length).round() as i32,
            y: ((self.end.y - self.start.y) as f64 / length).round() as i32,
        };
        assert!(v.x == 1 || v.x == 0 || v.x == -1);
        assert!(v.y == 1 || v.y == 0 || v.y == -1);
        assert!(!(v.x == 0 && v.y == 0));
        v
    }

    fn positions(&self) -> Vec<Position> {
        assert!(self.start != self.end);

        let mut result = vec![];

        let slope = self.slope();
        let mut pos = self.start;

        while pos != self.end {
            result.push(pos);
            pos += slope;
        }
        result.push(pos);

        result
    }
}

#[derive(Default)]
struct Board {
    board: HashMap<Position, i32>,
}

impl Board {
    fn incr(&mut self, pos: Position) {
        let e = self.board.entry(pos).or_insert(0);
        (*e) += 1;
    }
}

parser_from_str!(Line);

#[part_one]
fn part_one(lines: List<Line>) -> usize {
    part_two(
        lines
            .into_iter()
            .filter(|l| l.is_horizontal_or_vertical())
            .collect(),
    )
}

#[part_two]
fn part_two(lines: List<Line>) -> usize {
    let mut board = Board::default();
    for line in lines.iter() {
        for pos in line.positions() {
            board.incr(pos);
        }
    }
    board.board.into_values().filter(|&v| v > 1).count()
}

harness!();
