use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input: Vec<Vec<char>> = include_str!("../../input/12.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}

fn part1(input: Vec<Vec<char>>) -> usize {
    let nr = input.len();
    let nc = input[0].len();

    let mut price = 0;
    let mut mapped = HashSet::<(usize, usize)>::new();

    for r in 0..nr {
        for c in 0..nc {
            if !mapped.contains(&(r, c)) {
                let char = input[r][c];

                let mut field = HashSet::new();
                let mut edges = 0;
                let mut frontier = HashSet::from([(r, c)]);

                while !frontier.is_empty() {
                    let mut new_frontier = HashSet::new();
                    for (r, c) in frontier.clone() {
                        if r != 0 {
                            let coord = (r - 1, c);
                            if input[r - 1][c] != char {
                                edges += 1;
                            } else {
                                new_frontier.insert(coord);
                            }
                        } else {
                            edges += 1;
                        }
                        if r != nr - 1 {
                            let coord = (r + 1, c);
                            if input[r + 1][c] != char {
                                edges += 1;
                            } else {
                                new_frontier.insert(coord);
                            }
                        } else {
                            edges += 1;
                        }
                        if c != 0 {
                            let coord = (r, c - 1);
                            if input[r][c - 1] != char {
                                edges += 1;
                            } else {
                                new_frontier.insert(coord);
                            }
                        } else {
                            edges += 1;
                        }
                        if c != nc - 1 {
                            let coord = (r, c + 1);
                            if input[r][c + 1] != char {
                                edges += 1;
                            } else {
                                new_frontier.insert(coord);
                            }
                        } else {
                            edges += 1;
                        }
                    }
                    field.extend(frontier);
                    new_frontier.retain(|coord| !field.contains(coord));
                    frontier = new_frontier;
                }

                price += field.len() * edges;
                mapped.extend(field);
            }
        }
    }

    price
}

//TODO
fn part2(input: Vec<Vec<char>>) -> usize {
    let nr = input.len();
    let nc = input[0].len();

    let mut price = 0;
    let mut mapped = HashSet::<(usize, usize)>::new();

    for r in 0..nr {
        for c in 0..nc {
            if !mapped.contains(&(r, c)) {
                let char = input[r][c];

                let mut field = HashSet::new();
                let mut edge: Vec<(isize, isize)> = Vec::new();
                let mut frontier = HashSet::from([(r, c)]);

                while !frontier.is_empty() {
                    let mut new_frontier = HashSet::new();
                    let mut coord;
                    for (r, c) in frontier.clone() {
                        coord = (r as isize - 1, c as isize);
                        if r != 0 {
                            if input[r - 1][c] != char {
                                edge.push(coord);
                            } else {
                                new_frontier.insert((r - 1, c));
                            }
                        } else {
                            edge.push(coord);
                        }

                        coord = (r as isize + 1, c as isize);
                        if r != nr - 1 {
                            if input[r + 1][c] != char {
                                edge.push(coord);
                            } else {
                                new_frontier.insert((r + 1, c));
                            }
                        } else {
                            edge.push(coord);
                        }

                        coord = (r as isize, c as isize - 1);
                        if c != 0 {
                            if input[r][c - 1] != char {
                                edge.push(coord);
                            } else {
                                new_frontier.insert((r, c - 1));
                            }
                        } else {
                            edge.push(coord);
                        }

                        coord = (r as isize, c as isize + 1);
                        if c != nc - 1 {
                            if input[r][c + 1] != char {
                                edge.push(coord);
                            } else {
                                new_frontier.insert((r, c + 1));
                            }
                        } else {
                            edge.push(coord);
                        }
                    }
                    field.extend(frontier);
                    new_frontier.retain(|coord| !field.contains(coord));
                    frontier = new_frontier;
                }

                price += field.len();
                mapped.extend(field);
            }
        }
    }

    price
}
