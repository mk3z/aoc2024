use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/04.txt").lines().collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn rotate45(n: usize, m: usize, matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = Vec::<Vec<char>>::new();

    let mut ctr = 0;
    while ctr < 2 * n - 1 {
        let mut lst = Vec::<char>::new();

        for i in 0..m {
            for j in 0..n {
                if i + j == ctr {
                    lst.push(matrix[i][j]);
                }
            }
        }

        lst.reverse();
        res.push(lst);

        ctr += 1;
    }

    res
}

fn rotate90(n: usize, m: usize, matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec![' '; n]; m];

    for i in 0..m {
        for j in 0..n {
            res[j][n - 1 - i] = matrix[i][j];
        }
    }

    res
}

fn count(strings: &Vec<String>) -> usize {
    strings
        .iter()
        .map(|l| l.matches("XMAS").count() + l.matches("SAMX").count())
        .sum::<usize>()
}

fn part1(strings: &Vec<&str>) -> usize {
    let n = strings[0].len();
    let m = strings.len();

    let rotated0 = strings.iter().map(|l| l.chars().collect()).collect();
    let rotated45 = rotate45(n, m, &rotated0);
    let rotated90 = rotate90(n, m, &rotated0);
    let rotated135 = rotate45(n, m, &rotated90);

    let rotations = [rotated0, rotated45, rotated90, rotated135]
        .into_iter()
        .map(|r| r.iter().map(|l| l.iter().collect()).collect());

    rotations.map(|r| count(&r)).sum()
}

fn find_pattern(matrix: &Vec<Vec<char>>) -> usize {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut sum = 0;

    for i in 1..(m - 1) {
        for j in 1..(n - 1) {
            if matrix[i][j] == 'A' {
                match (
                    (matrix[i - 1][j - 1], matrix[i + 1][j + 1]),
                    (matrix[i - 1][j + 1], matrix[i + 1][j - 1]),
                ) {
                    (('M', 'S'), ('M', 'S'))
                    | (('M', 'S'), ('S', 'M'))
                    | (('S', 'M'), ('M', 'S'))
                    | (('S', 'M'), ('S', 'M')) => sum += 1,
                    _ => continue,
                }
            }
        }
    }

    sum
}

fn part2(strings: &Vec<&str>) -> usize {
    find_pattern(&strings.iter().map(|l| l.chars().collect()).collect())
}
