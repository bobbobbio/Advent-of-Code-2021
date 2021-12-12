#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use multiset::HashMultiSet;
use std::collections::{HashMap, HashSet};
use std::{fmt, matches};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum CaveKind {
    Big,
    Small,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cave {
    kind: CaveKind,
    name: String,
}

impl HasParser for Cave {
    #[into_parser]
    fn parser() -> _ {
        let big = many1(upper()).map(|name| Self {
            kind: CaveKind::Big,
            name,
        });
        let small = many1(lower()).map(|name| Self {
            kind: CaveKind::Small,
            name,
        });
        big.or(small)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Node {
    Cave(Cave),
    Start,
    End,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Cave(c) => write!(f, "{}", &c.name),
        }
    }
}

impl Node {
    fn is_small_cave(&self) -> bool {
        matches!(
            self,
            Self::Cave(Cave {
                kind: CaveKind::Small,
                ..
            })
        )
    }

    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }
}

impl HasParser for Node {
    #[into_parser]
    fn parser() -> _ {
        let start = string("start").map(|_| Self::Start);
        let end = string("end").map(|_| Self::End);
        let cave = Cave::parser().map(Self::Cave);
        choice((attempt(start), attempt(end), cave))
    }
}

#[derive(Debug)]
struct CaveSystem {
    edges: HashMap<Node, HashSet<Node>>,
}

#[derive(Clone)]
struct Cursor {
    node: Node,
    index: usize,
}

impl fmt::Debug for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.node)
    }
}

impl Cursor {
    fn new(node: Node) -> Self {
        Self { node, index: 0 }
    }

    fn start() -> Self {
        Self {
            node: Node::Start,
            index: 0,
        }
    }
}

#[derive(Debug)]
struct PathStack {
    stack: Vec<Cursor>,
    set: HashMultiSet<Node>,
}

impl PathStack {
    fn new() -> Self {
        Self {
            stack: vec![Cursor::start()],
            set: HashMultiSet::new(),
        }
    }

    fn push(&mut self, c: Cursor) {
        self.set.insert_times(c.node.clone(), 1);
        self.stack.push(c);
    }

    fn pop(&mut self) {
        let n = self.stack.pop().unwrap();
        self.set.remove_times(&n.node, 1);
    }

    fn has_duplicate_small_cave(&self) -> bool {
        let mut s = HashSet::new();
        for c in self.stack.iter().filter(|c| c.node.is_small_cave()) {
            if !s.insert(c.node.clone()) {
                return true;
            }
        }

        return false;
    }

    fn contains(&self, n: &Node) -> bool {
        self.set.contains(n)
    }

    fn last(&self) -> Option<Cursor> {
        self.stack.last().map(|c| c.clone())
    }

    fn last_mut(&mut self) -> &mut Cursor {
        self.stack.last_mut().unwrap()
    }
}

impl CaveSystem {
    fn from_edges(edges_in: HashSet<(Node, Node)>) -> Self {
        let mut edges = HashMap::<Node, HashSet<Node>>::new();
        for (s, d) in edges_in {
            edges.entry(s.clone()).or_default().insert(d.clone());
            edges.entry(d).or_default().insert(s);
        }
        Self { edges }
    }

    fn num_paths(&self, small_cave_twice: bool) -> u64 {
        let mut path = PathStack::new();

        let mut num_paths = 0;
        'outer: while let Some(c) = path.last() {
            if c.node.is_end() {
                num_paths += 1;
                path.pop();
                continue;
            }

            if let Some(edges) = self.edges.get(&c.node) {
                for (n, e) in edges.iter().skip(c.index).enumerate() {
                    if e.is_start() {
                        continue;
                    }

                    if e.is_small_cave() && path.contains(e) {
                        if !small_cave_twice || path.has_duplicate_small_cave() {
                            continue;
                        }
                    }

                    path.last_mut().index += n + 1;

                    path.push(Cursor::new(e.clone()));
                    continue 'outer;
                }
            }
            path.pop();
        }
        num_paths
    }
}

impl HasParser for CaveSystem {
    #[into_parser]
    fn parser() -> _ {
        let line = (Node::parser().skip(token('-')), Node::parser());
        many1(line.skip(token('\n'))).map(Self::from_edges)
    }
}

#[part_one]
fn part_one(system: CaveSystem) -> u64 {
    system.num_paths(false)
}

#[part_two]
fn part_two(system: CaveSystem) -> u64 {
    system.num_paths(true)
}

harness!();
