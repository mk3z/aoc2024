use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/10.txt")
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(tiles: &Vec<Vec<u8>>) -> usize {
    fn search(
        (r, c, nr, nc, tiles): (usize, usize, usize, usize, &Vec<Vec<u8>>),
    ) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::new();

        let h = tiles[r][c];

        if h == 9 {
            visited.insert((r, c));
            return visited;
        }

        let mut nrs: HashSet<(usize, usize)> = HashSet::new();
        if r != 0 && tiles[r - 1][c] == h + 1 {
            nrs.insert((r - 1, c));
        }
        if r != nr - 1 && tiles[r + 1][c] == h + 1 {
            nrs.insert((r + 1, c));
        }
        if c != 0 && tiles[r][c - 1] == h + 1 {
            nrs.insert((r, c - 1));
        }
        if c != nc - 1 && tiles[r][c + 1] == h + 1 {
            nrs.insert((r, c + 1));
        }

        nrs.iter()
            .map(|(r, c)| (*r, *c, nr, nc, tiles))
            .map(search)
            .flatten()
            .collect()
    }

    tiles
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, n)| **n == 0u8)
                .map(move |(j, _)| (i, j))
        })
        .flatten()
        .map(|(r, c)| search((r, c, tiles.len(), tiles[0].len(), tiles)).len())
        .sum()
}

fn part2(tiles: &Vec<Vec<u8>>) -> usize {
    fn search((r, c, nr, nc, tiles): (usize, usize, usize, usize, &Vec<Vec<u8>>)) -> usize {
        let h = tiles[r][c];

        if h == 9 {
            return 1;
        }

        let mut nrs: HashSet<(usize, usize)> = HashSet::new();
        if r != 0 && tiles[r - 1][c] == h + 1 {
            nrs.insert((r - 1, c));
        }
        if r != nr - 1 && tiles[r + 1][c] == h + 1 {
            nrs.insert((r + 1, c));
        }
        if c != 0 && tiles[r][c - 1] == h + 1 {
            nrs.insert((r, c - 1));
        }
        if c != nc - 1 && tiles[r][c + 1] == h + 1 {
            nrs.insert((r, c + 1));
        }

        nrs.iter()
            .map(|(r, c)| (*r, *c, nr, nc, tiles))
            .map(search)
            .sum()
    }

    tiles
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, n)| **n == 0u8)
                .map(move |(j, _)| (i, j))
        })
        .flatten()
        .map(|(r, c)| search((r, c, tiles.len(), tiles[0].len(), tiles)))
        .sum()
}
