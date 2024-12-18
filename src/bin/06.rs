use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn rot(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn mov(dir: &Dir, mut pos: (usize, usize)) -> (usize, usize) {
    match dir {
        Dir::Up => {
            pos.0 -= 1;
        }
        Dir::Right => {
            pos.1 += 1;
        }
        Dir::Down => {
            pos.0 += 1;
        }
        Dir::Left => {
            pos.1 -= 1;
        }
    }

    pos
}

fn main() {
    let input = include_str!("../../input/06.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

// Quite lazy brute force solution
fn part2(orig_map: &Vec<Vec<char>>) -> usize {
    let mut orig_pos = (0, 0);

    'outer: for i in 0..orig_map.len() {
        for j in 0..orig_map[0].len() {
            if orig_map[i][j] == '^' {
                orig_pos = (i, j);
                break 'outer;
            }
        }
    }

    let loops = Arc::new(AtomicUsize::new(0));

    (0..orig_map.len()).into_par_iter().for_each(|r| {
        (0..orig_map[0].len()).into_par_iter().for_each(|c| {
            let mut map = orig_map.clone();
            map[r][c] = '#';

            let mut pos = orig_pos;

            let mut next: Option<&char>;
            if let Some(nextpos) = pos.0.checked_sub(1) {
                next = map[nextpos].get(pos.1 - 1);
            } else {
                next = None
            };

            let mut dir = Dir::Up;

            let mut visited: Vec<(usize, usize)> = vec![];

            loop {
                if let Some(j) = visited.iter().rposition(|p| *p == pos) {
                    if let Some(i) = visited.iter().take(j).rposition(|p| *p == pos) {
                        if visited[i..j] == visited[j..] {
                            loops.fetch_add(1, Ordering::SeqCst);
                            break;
                        }
                    }
                }

                match next {
                    Some('#') => {
                        dir = rot(dir);
                    }
                    None => {
                        break;
                    }
                    _ => {
                        visited.push(pos);
                        pos = mov(&dir, pos);
                    }
                }

                match dir {
                    Dir::Up => {
                        if let Some(nextpos) = pos.0.checked_sub(1) {
                            next = map.get(nextpos).map(|r| &r[pos.1]);
                        } else {
                            break;
                        }
                    }
                    Dir::Right => {
                        next = map[pos.0].get(pos.1 + 1);
                    }
                    Dir::Down => {
                        next = map.get(pos.0 + 1).map(|r| &r[pos.1]);
                    }
                    Dir::Left => {
                        if let Some(nextpos) = pos.1.checked_sub(1) {
                            next = map[pos.0].get(nextpos);
                        } else {
                            break;
                        }
                    }
                }
            }
        });
    });

    loops.load(Ordering::SeqCst)
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut pos = (0, 0);

    'outer: for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                pos = (i, j);
                break 'outer;
            }
        }
    }

    let mut next: Option<&char>;
    if let Some(nextpos) = pos.0.checked_sub(1) {
        next = map[nextpos].get(pos.1 - 1);
    } else {
        next = None
    };

    let mut dir = Dir::Up;

    let mut vis = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    loop {
        if !visited[pos.0][pos.1] {
            vis += 1;
            visited[pos.0][pos.1] = true;
        }

        match next {
            Some('#') => {
                dir = rot(dir);
            }
            None => {
                break;
            }
            _ => pos = mov(&dir, pos),
        }

        match dir {
            Dir::Up => {
                if let Some(nextpos) = pos.0.checked_sub(1) {
                    next = map.get(nextpos).map(|r| &r[pos.1]);
                } else {
                    break;
                }
            }
            Dir::Right => {
                next = map[pos.0].get(pos.1 + 1);
            }
            Dir::Down => {
                next = map.get(pos.0 + 1).map(|r| &r[pos.1]);
            }
            Dir::Left => {
                next = map[pos.0].get(pos.1 - 1);
            }
        }
    }

    vis
}
