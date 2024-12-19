use itertools::Itertools;
use rayon::prelude::*;
use std::sync::{Arc, atomic::AtomicUsize};

fn main() {
    let input = include_str!("../../input/19.txt")
        .trim()
        .split("\n\n")
        .collect_vec();

    let towels = input[0].split(", ").collect_vec();
    let patterns = input[1].split("\n").collect_vec();

    println!("{}", part1(&towels, &patterns));
    println!("{}", part2(&towels, &patterns));
}

fn part2(towels: &Vec<&str>, patterns: &Vec<&str>) -> usize {
    let sum = Arc::new(AtomicUsize::new(0));

    patterns.par_iter().for_each(|pattern| {
        let len = pattern.len();

        let mut dp: Vec<usize> = Vec::with_capacity(len);
        dp.push(1);
        for _ in 1..=len {
            dp.push(0);
        }

        for i in 1..=len {
            for j in 0..i {
                if dp[j] > 0 && towels.contains(&&pattern[j..i]) {
                    dp[i] += dp[j];
                }
            }
        }
        sum.fetch_add(dp[len], std::sync::atomic::Ordering::Relaxed);
    });

    sum.load(std::sync::atomic::Ordering::Relaxed)
}

fn part1(towels: &Vec<&str>, patterns: &Vec<&str>) -> usize {
    let sum = Arc::new(AtomicUsize::new(0));

    patterns.par_iter().for_each(|pattern| {
        let len = pattern.len();

        let mut dp: Vec<bool> = Vec::with_capacity(len);
        dp.push(true);
        for _ in 1..=len {
            dp.push(false);
        }

        for i in 1..=len {
            for j in 0..i {
                if dp[j] && towels.contains(&&pattern[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        if dp[len] {
            sum.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    });

    sum.load(std::sync::atomic::Ordering::Relaxed)
}
