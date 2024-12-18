#![feature(binary_heap_into_iter_sorted)]

use itertools::Itertools;
use std::str::SplitWhitespace;

fn main() {
    let (sum1, sum2): (i32, i32) = include_str!("../../input/02.txt")
        .lines()
        .map(|line| {
            (
                part1(line.split_whitespace()),
                part2(line.split_whitespace()),
            )
        })
        .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2));

    println!("{}", sum1);
    println!("{}", sum2);
}

fn check(line: &Vec<i32>) -> bool {
    let tuples = line.iter().tuple_windows();

    let mut inc = false;
    let mut dec = false;

    for (l, r) in tuples {
        if l > r {
            if inc {
                return false;
            }
            let diff = l - r;
            if 1 > diff || diff > 3 {
                return false;
            }
            dec = true;
        } else if l < r {
            if dec {
                return false;
            }
            let diff = r - l;
            if 1 > diff || diff > 3 {
                return false;
            }
            inc = true;
        } else {
            return false;
        }
    }

    true
}

fn part2(line: SplitWhitespace) -> i32 {
    let parsed: Vec<i32> = line.map(|n| n.parse::<i32>().unwrap()).collect();

    if check(&parsed) {
        return 1;
    }

    for i in 0..parsed.len() {
        let mut tmp = parsed.clone();
        tmp.remove(i);

        if check(&tmp) {
            return 1;
        }
    }

    return 0;
}

fn part1(line: SplitWhitespace) -> i32 {
    let mut inc = false;
    let mut dec = false;
    for (l, r) in line.map(|n| n.parse::<i32>().unwrap()).tuple_windows() {
        if l > r {
            if inc {
                return 0;
            }
            let diff = l - r;
            if 1 > diff || diff > 3 {
                return 0;
            }
            dec = true;
        } else if l < r {
            if dec {
                return 0;
            }
            let diff = r - l;
            if 1 > diff || diff > 3 {
                return 0;
            }
            inc = true;
        } else {
            return 0;
        }
    }
    1
}
