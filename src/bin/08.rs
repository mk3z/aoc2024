use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/08.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    // Idk what happened with part 1
    println!("{}", part2(&input));
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let nr = map.len();
    let nc = map[0].len();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut antennae: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    for r in 0..nr {
        for c in 0..nc {
            match map[r][c] {
                '.' => continue,
                a => {
                    if let Some(ants) = antennae.get(&a) {
                        antinodes.extend(
                            ants.iter()
                                .map(|p| {
                                    [mirror(p, &(r, c), (nr, nc)), mirror(&(r, c), p, (nr, nc))]
                                })
                                .flatten()
                                .flatten()
                                .flatten()
                                .collect::<HashSet<(usize, usize)>>(),
                        )
                    }
                    antennae
                        .entry(a)
                        .or_insert_with(HashSet::new)
                        .insert((r, c));
                }
            }
        }
    }

    let res = antinodes.iter().filter(|(r, c)| r < &nr && c < &nc);

    res.count()
}

fn mirror(
    p: &(usize, usize),
    q: &(usize, usize),
    lim: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let mut ps = HashSet::new();

    let mut r = *p;
    let mut s = *q;

    while s.0 < lim.0 && s.1 < lim.1 {
        ps.insert(s);
        let tmp = s;
        s = (
            if let Some(v) = (2 * s.0).checked_sub(r.0) {
                v
            } else {
                break;
            },
            if let Some(v) = (2 * s.1).checked_sub(r.1) {
                v
            } else {
                break;
            },
        );
        r = tmp;
    }

    Some(ps)
}
