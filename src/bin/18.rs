use itertools::Itertools;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

const NR: usize = 71;
const NC: usize = 71;
const START: (usize, usize) = (0, 0);
const END: (usize, usize) = (NR - 1, NR - 1);
const BYTES: usize = 1024;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Vec2D<T>
where
    T: Copy + Hash + Eq,
{
    inner: Vec<Vec<T>>,
}

impl<T: Copy + Hash + Eq> Vec2D<T> {
    fn new(vec: Vec<Vec<T>>) -> Vec2D<T> {
        Vec2D { inner: vec }
    }

    fn get(&self, (r, c): (usize, usize)) -> T {
        self.inner[r][c]
    }

    fn set(&mut self, (r, c): (usize, usize), v: T) {
        self.inner[r][c] = v;
    }
}

fn neighbors((r, c): (usize, usize)) -> HashSet<(usize, usize)> {
    let mut nrs = HashSet::new();

    if r != 0 {
        nrs.insert((r - 1, c));
    }
    if r != NR - 1 {
        nrs.insert((r + 1, c));
    }
    if c != 0 {
        nrs.insert((r, c - 1));
    }
    if c != NC - 1 {
        nrs.insert((r, c + 1));
    }

    nrs
}

fn main() {
    let input = include_str!("../../input/18.txt");

    println!("{}", part1(input).unwrap());
    let (r, c) = part2(input).unwrap();
    println!("{r},{c}");
}

fn part2(input: &str) -> Option<(usize, usize)> {
    let corruption: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .collect();

    if let Some(index) = (BYTES + 1..corruption.len())
        .rev()
        .find(|c| find_path(HashSet::from_iter(corruption.clone().into_iter().take(*c))))
    {
        Some(corruption[index])
    } else {
        None
    }
}

fn find_path(corruption: HashSet<(usize, usize)>) -> bool {
    let mut heap = BinaryHeap::new();
    heap.push(START);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(pos) = heap.pop() {
        visited.insert(pos);
        if pos == END {
            return true;
        }

        for n in neighbors(pos).difference(&corruption) {
            if !visited.contains(n) {
                heap.push(*n);
            }
        }
    }

    false
}

fn part1(input: &str) -> Option<usize> {
    let corruption: HashSet<(usize, usize)> = input
        .lines()
        .take(BYTES)
        .map(|l| {
            l.split(",")
                .map(|c| c.parse().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .collect();

    let mut dist = Vec2D::new(
        (0..NR)
            .map(|_| (0..NC).map(|_| usize::MAX).collect_vec())
            .collect_vec(),
    );
    dist.set(START, 0);

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        pos: START,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == END {
            return Some(cost);
        }

        if cost > dist.get(pos) {
            continue;
        }

        for n in neighbors(pos).difference(&corruption) {
            let next = State {
                cost: cost + 1,
                pos: *n,
            };

            if next.cost < dist.get(next.pos) {
                heap.push(next);
                dist.set(next.pos, next.cost);
            }
        }
    }

    None
}
