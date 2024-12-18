use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Neg;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Tile {
    Wall,
    Space,
    Start,
    End,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Neg for Dir {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    dir: Dir,
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
    r: usize,
    c: usize,
}

impl<T: Copy + Hash + Eq> Vec2D<T> {
    fn new(vec: Vec<Vec<T>>) -> Vec2D<T> {
        Vec2D {
            r: vec.len(),
            c: vec[0].len(),
            inner: vec,
        }
    }

    fn get(&self, (r, c): (usize, usize)) -> T {
        self.inner[r][c]
    }

    fn set(&mut self, (r, c): (usize, usize), v: T) {
        self.inner[r][c] = v;
    }

    fn neighbors(
        &self,
        (r, c): (usize, usize),
        filter: &HashSet<T>,
    ) -> HashSet<((usize, usize), Dir)> {
        let mut nrs = HashSet::new();

        if r != 0 && !filter.contains(&self.get((r - 1, c))) {
            nrs.insert(((r - 1, c), Dir::North));
        }
        if r != self.r - 1 && !filter.contains(&self.get((r + 1, c))) {
            nrs.insert(((r + 1, c), Dir::South));
        }
        if c != 0 && !filter.contains(&self.get((r, c - 1))) {
            nrs.insert(((r, c - 1), Dir::West));
        }
        if c != self.c - 1 && !filter.contains(&self.get((r, c + 1))) {
            nrs.insert(((r, c + 1), Dir::East));
        }

        nrs
    }
}

fn main() {
    let map = Vec2D::new(
        include_str!("../../input/16.txt")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Space,
                        '#' => Tile::Wall,
                        'S' => Tile::Start,
                        'E' => Tile::End,
                        t => panic!("Invalid tile {t}"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    );

    let start = (map.r - 2, 1usize);
    let end = (1usize, map.c - 2);

    let sol = part1(map, start, end);
    println!("{}\n{}", sol.0, sol.1);
}

fn path_length(
    parents: HashMap<(usize, usize), Option<HashSet<(usize, usize)>>>,
    start: (usize, usize),
) -> usize {
    let mut todo = parents[&start].clone().unwrap();
    let mut sum = 2;
    while !todo.is_empty() {
        let mut new_todo: HashSet<(usize, usize)> = HashSet::new();

        for pos in todo.iter() {
            if let Some(p) = parents[pos].clone() {
                sum += 1;
                new_todo.extend(p);
            }
        }

        todo = new_todo;
    }

    sum
}

fn part1(map: Vec2D<Tile>, start: (usize, usize), end: (usize, usize)) -> (usize, usize) {
    let mut dist = Vec2D::new(
        (0..map.r)
            .map(|_| (0..map.c).map(|_| usize::MAX).collect_vec())
            .collect_vec(),
    );
    dist.set(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        pos: start,
        dir: Dir::East,
    });

    let mut parents: HashMap<(usize, usize), Option<HashSet<(usize, usize)>>> = HashMap::new();
    parents.insert(start, None);

    let filter = HashSet::from([Tile::Wall, Tile::Start]);

    while let Some(State { cost, pos, dir }) = heap.pop() {
        if pos == end {
            return (cost, path_length(parents, pos));
        }

        if cost > dist.get(pos) {
            continue;
        }

        for n in map
            .neighbors(pos, &filter)
            .iter()
            .filter(|(_, d)| *d != -dir)
        {
            let next = State {
                cost: if n.1 == dir { cost + 1 } else { cost + 1001 },
                pos: n.0,
                dir: n.1,
            };

            let next_cost = dist.get(next.pos);

            if next.cost < next_cost {
                heap.push(next);
                dist.set(next.pos, next.cost);
                parents.insert(next.pos, Some(HashSet::from([pos])));
            } else if next.cost == next_cost || next.cost == next_cost + 1000 {
                //append

                if let Some(p) = parents
                    .entry(next.pos)
                    .or_insert(Some(HashSet::from([pos])))
                    .as_mut()
                {
                    p.insert(pos);
                }
            }
        }
    }

    return (0, 0);
}
