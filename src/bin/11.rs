use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/11.txt")
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .counts();

    println!("{}", solve(input.clone(), 25));
    println!("{}", solve(input, 75));
}

#[inline(always)]
fn digits(a: usize) -> usize {
    (a.ilog10() + 1).try_into().unwrap()
}

fn solve(mut stones: HashMap<usize, usize>, seconds: usize) -> usize {
    for _ in 0..seconds {
        let mut new = HashMap::new();
        for (s, n) in stones {
            if s == 0usize {
                new.entry(1).and_modify(|m| *m += n).or_insert(n);
            } else if digits(s) % 2 == 0 {
                let a = 10usize.pow((digits(s) / 2).try_into().unwrap());
                new.entry(s / a).and_modify(|m| *m += n).or_insert(n);
                new.entry(s % a).and_modify(|m| *m += n).or_insert(n);
            } else {
                new.entry(s * 2024).and_modify(|m| *m += n).or_insert(n);
            }
        }
        stones = new;
    }

    stones.into_values().sum()
}
