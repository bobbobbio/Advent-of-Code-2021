use parse::{prelude::*, *};
use std::io;

#[derive(Clone, Debug)]
pub struct NumberList(Vec<u32>);

impl NumberList {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        sep_by1(u32_parser(), token(',')).map(Self)
    }
}

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

impl BingoBoard {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        let spaces = many::<String, _, _>(token(' '));
        let spaces1 = many1::<String, _, _>(token(' '));
        let one_line = sep_by1(spaces.with(u32_parser()), spaces1).skip(newline());
        many1(one_line).map(Self::new)
    }
}

#[derive(Clone, Debug)]
struct BingoGame {
    input: NumberList,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        let input = NumberList::parser().skip(newline());
        let boards = sep_by1(BingoBoard::parser(), newline());
        (input.skip(newline()), boards).map(|(input, boards)| Self { input, boards })
    }
}

parser_from_str!(BingoGame);

fn part_one(mut b: BingoGame) {
    for &value in &b.input.0 {
        for board in &mut b.boards {
            board.mark_value(value);
            if board.has_won() {
                println!("{}", board.score() * value);
                return;
            }
        }
    }
    panic!("No winner");
}

fn part_two(mut b: BingoGame) {
    let mut unwon_boards = b.boards.len();
    for &value in &b.input.0 {
        for board in &mut b.boards {
            if board.has_won() {
                continue;
            }
            board.mark_value(value);

            if board.has_won() {
                unwon_boards -= 1;
                if unwon_boards == 0 {
                    println!("{}", board.score() * value);
                    return;
                }
            }
        }
    }
    panic!("Unwinable board");
}

fn main() -> Result<()> {
    let b: BingoGame = parse_from_reader(io::stdin().lock())?;

    println!("Part 1");
    part_one(b.clone());

    println!("Part 2");
    part_two(b);

    Ok(())
}