#![feature(binary_heap_into_iter_sorted)]

use std::collections::BinaryHeap;

fn main() {
    let mut lines = include_str!("../../input/01.txt").lines();

    let mut a = BinaryHeap::<i32>::new();
    let mut b = BinaryHeap::<i32>::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let mut split = line.split_whitespace();

        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", part1(a.clone(), b.clone()));
    println!("{}", part2(a, b));
}

fn part2(mut a: BinaryHeap<i32>, mut b: BinaryHeap<i32>) -> i32 {
    let mut res = 0;

    let Some(mut l) = a.pop() else {
        return res;
    };

    let Some(mut r) = b.pop() else {
        return res;
    };

    loop {
        let mut tmp = 0;

        while l != r {
            while l > r {
                match a.pop() {
                    Some(v) => l = v,
                    None => {
                        return res;
                    }
                }
            }

            while l < r {
                match b.pop() {
                    Some(v) => r = v,
                    None => {
                        return res;
                    }
                }
            }
        }

        let prev = l;
        while l == prev {
            tmp += l;
            match a.pop() {
                Some(v) => l = v,
                None => {
                    break;
                }
            }
        }

        while r == prev {
            res += tmp;
            match b.pop() {
                Some(v) => r = v,
                None => {
                    return res;
                }
            }
        }
    }
}

fn part1(a: BinaryHeap<i32>, b: BinaryHeap<i32>) -> i32 {
    let mut c = Vec::<i32>::new();
    for (a, b) in a.into_iter_sorted().zip(b.into_iter_sorted()) {
        c.push((a - b).abs());
    }

    c.into_iter().sum::<i32>()
}
