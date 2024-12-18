use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Tile {
    Space,
    Wall,
    Box,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut robot_pos = (0, 0);
    let input = include_str!("../../input/15.txt").lines().collect_vec();

    let mut map: Vec<Vec<Tile>> = Vec::new();
    for (r, l) in input.iter().enumerate() {
        if *l == "" {
            break;
        }

        map.push(
            l.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Tile::Space,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => {
                        robot_pos = (r, c);
                        return Tile::Space;
                    }
                    _ => panic!("Invalid tile {}", ch),
                })
                .collect_vec(),
        );
    }

    let moves = input
        .iter()
        .skip_while(|&&l| l != "")
        .skip(1)
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => panic!("Invalid move {}", c),
            })
        })
        .collect_vec();

    println!("{}", part1(map, &moves, robot_pos));
}

fn next(r: (usize, usize), mv: &Move) -> (usize, usize) {
    match mv {
        Move::Up => (r.0 - 1, r.1),
        Move::Down => (r.0 + 1, r.1),
        Move::Left => (r.0, r.1 - 1),
        Move::Right => (r.0, r.1 + 1),
    }
}

fn move_box(b: (usize, usize), dir: &Move, map: &mut Vec<Vec<Tile>>) -> bool {
    match dir {
        Move::Up => {
            for r in (0..b.0).rev() {
                match map[r][b.1] {
                    Tile::Box => {}
                    Tile::Wall => {
                        return false;
                    }
                    Tile::Space => {
                        map[b.0][b.1] = Tile::Space;
                        map[r][b.1] = Tile::Box;
                        return true;
                    }
                }
            }
        }
        Move::Down => {
            for r in b.0 + 1..map.len() {
                match map[r][b.1] {
                    Tile::Box => {}
                    Tile::Wall => {
                        return false;
                    }
                    Tile::Space => {
                        map[b.0][b.1] = Tile::Space;
                        map[r][b.1] = Tile::Box;
                        return true;
                    }
                }
            }
        }
        Move::Left => {
            for c in (0..b.1).rev() {
                match map[b.0][c] {
                    Tile::Box => {}
                    Tile::Wall => {
                        return false;
                    }
                    Tile::Space => {
                        map[b.0][b.1] = Tile::Space;
                        map[b.0][c] = Tile::Box;
                        return true;
                    }
                }
            }
        }
        Move::Right => {
            for c in b.1 + 1..map[0].len() {
                match map[b.0][c] {
                    Tile::Box => {}
                    Tile::Wall => {
                        return false;
                    }
                    Tile::Space => {
                        map[b.0][b.1] = Tile::Space;
                        map[b.0][c] = Tile::Box;
                        return true;
                    }
                }
            }
        }
    };
    return false;
}

fn part1(mut map: Vec<Vec<Tile>>, moves: &Vec<Move>, mut r: (usize, usize)) -> usize {
    for mv in moves {
        let next = next(r, &mv);
        match &map[next.0][next.1] {
            Tile::Space => r = next,
            Tile::Wall => {}
            Tile::Box => {
                if move_box(next, &mv, &mut map) {
                    r = next;
                }
            }
        }
    }

    map.iter()
        .enumerate()
        .map(|(r, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, t)| **t == Tile::Box)
                .map(move |(c, _)| 100 * r + c)
        })
        .flatten()
        .sum()
}

fn print_map(map: &Vec<Vec<Tile>>, robot: (usize, usize)) {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if robot == (r, c) {
                print!("@");
            } else {
                match &map[r][c] {
                    Tile::Space => print!("."),
                    Tile::Wall => print!("#"),
                    Tile::Box => print!("O"),
                }
            }
        }
        print!("\n");
    }
    print!("\n");
}
