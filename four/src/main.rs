#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use advent::prelude::*;

#[derive(Clone, Debug)]
struct BingoCell {
    value: u32,
    marked: bool,
}

impl BingoCell {
    fn new(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Clone, Debug)]
struct BingoBoard {
    board: Vec<Vec<BingoCell>>,
}

impl BingoBoard {
    fn new(values: Vec<Vec<u32>>) -> Self {
        Self {
            board: values
                .into_iter()
                .map(|row| row.into_iter().map(BingoCell::new).collect())
                .collect(),
        }
    }

    fn cells(&self) -> impl Iterator<Item = &BingoCell> {
        self.board.iter().map(|row| row.iter()).flatten()
    }

    fn cells_mut(&mut self) -> impl Iterator<Item = &mut BingoCell> {
        self.board.iter_mut().map(|row| row.iter_mut()).flatten()
    }

    fn width(&self) -> usize {
        self.board.iter().map(|v| v.len()).max().unwrap_or(0)
    }

    fn has_won(&self) -> bool {
        for row in &self.board {
            if row.iter().all(|c| c.marked) {
                return true;
            }
        }

        for column in 0..self.width() {
            if self.board.iter().map(|v| &v[column]).all(|c| c.marked) {
                return true;
            }
        }

        return false;
    }

    fn score(&self) -> u32 {
        self.cells()
            .filter_map(|c| (!c.marked).then(|| c.value))
            .sum()
    }

    fn mark_value(&mut self, value: u32) {
        self.cells_mut().for_each(|c| {
            if c.value == value {
                c.marked = true
            }
        });
    }
}

impl HasParser for BingoBoard {
    #[into_parser]
    fn parser() -> _ {
        let spaces = many::<String, _, _>(token(' '));
        let spaces1 = many1::<String, _, _>(token(' '));
        let one_line = sep_by1(spaces.with(u32::parser()), spaces1).skip(newline());
        many1(one_line).map(Self::new)
    }
}

#[derive(Clone, Debug)]
struct BingoGame {
    input: List<u32, Comma>,
    boards: Vec<BingoBoard>,
}

impl HasParser for BingoGame {
    #[into_parser]
    fn parser() -> _ {
        let input = List::<u32, Comma>::parser().skip(newline());
        let boards = sep_by1(BingoBoard::parser(), newline());
        (input.skip(newline()), boards).map(|(input, boards)| Self { input, boards })
    }
}

#[part_one]
fn part_one(mut b: BingoGame) -> u32 {
    for &value in &b.input {
        for board in &mut b.boards {
            board.mark_value(value);
            if board.has_won() {
                return board.score() * value;
            }
        }
    }
    panic!("No winner");
}

#[part_two]
fn part_two(mut b: BingoGame) -> u32 {
    let mut unwon_boards = b.boards.len();
    for &value in &b.input {
        for board in &mut b.boards {
            if board.has_won() {
                continue;
            }
            board.mark_value(value);

            if board.has_won() {
                unwon_boards -= 1;
                if unwon_boards == 0 {
                    return board.score() * value;
                }
            }
        }
    }
    panic!("Unwinable board");
}

harness!();
