use itertools::{Itertools, repeat_n};
use rayon::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn main() {
    let input: Vec<(usize, Vec<usize>)> = include_str!("../../input/07.txt")
        .lines()
        .map(|l| {
            let mut s = l.split(':');
            (
                s.next().unwrap().parse().unwrap(),
                s.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect();

    let ops1: Vec<Box<dyn Fn(usize, usize) -> usize + Send + Sync>> =
        vec![Box::new(|a, b| a + b), Box::new(|a, b| a * b)];

    let ops2: Vec<Box<dyn Fn(usize, usize) -> usize + Send + Sync>> = vec![
        Box::new(|a, b| a + b),
        Box::new(|a, b| a * b),
        Box::new(|a, b| a * 10usize.pow(b.ilog10() + 1) + b),
    ];

    println!("{}", solve(&input, &ops1));
    println!("{}", solve(&input, &ops2));
}

fn solve(
    input: &Vec<(usize, Vec<usize>)>,
    ops: &Vec<Box<dyn Fn(usize, usize) -> usize + Send + Sync>>,
) -> usize {
    let sum = Arc::new(AtomicUsize::new(0));

    input.into_par_iter().for_each(|l| {
        let test = l.0;
        let eq = &l.1;

        let first: [Box<dyn Fn(usize, usize) -> usize + Send + Sync>; 1] = [Box::new(|a, b| a + b)];

        if repeat_n(ops.iter(), eq.len() - 1)
            .multi_cartesian_product()
            .map(|p| first.iter().chain(p))
            .map(|op| eq.iter().zip(op).fold(0, |acc, (v, op)| op(acc, *v)))
            .filter(|n| *n == test)
            .peekable()
            .peek()
            .is_some()
        {
            sum.fetch_add(test, Ordering::SeqCst);
        }
    });

    sum.load(Ordering::SeqCst)
}
