use itertools::Itertools;
use text_io::scan;

type T = isize;

#[derive(Debug)]
struct Button {
    a: (T, T),
    b: (T, T),
    p: (T, T),
}

fn main() {
    let input = include_str!("../../input/13.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> T {
    solve(
        input
            .split("\n\n")
            .map(|block| {
                let (a0, a1, b0, b1, p0, p1): (T, T, T, T, T, T);
                scan!(block.bytes() =>
            "Button A: X+{}, Y+{}\n\
            Button B: X+{}, Y+{}\n\
            Prize: X={}, Y={}",
            a0, a1, b0, b1, p0, p1 );
                Button {
                    a: (a0, a1),
                    b: (b0, b1),
                    p: (p0, p1),
                }
            })
            .collect_vec(),
    )
}

fn part2(input: &str) -> T {
    solve(
        input
            .split("\n\n")
            .map(|block| {
                let (a0, a1, b0, b1, p0, p1): (T, T, T, T, T, T);
                scan!(block.bytes() =>
            "Button A: X+{}, Y+{}\n\
            Button B: X+{}, Y+{}\n\
            Prize: X={}, Y={}",
            a0, a1, b0, b1, p0, p1 );
                Button {
                    a: (a0, a1),
                    b: (b0, b1),
                    p: (p0 + 10000000000000, p1 + 10000000000000),
                }
            })
            .collect_vec(),
    )
}

fn solve(buttons: Vec<Button>) -> T {
    let mut sum = 0;

    for b in buttons {
        let det_l = b.a.0 * b.b.1 - b.b.0 * b.a.1;
        let det_a = b.p.0 * b.b.1 - b.b.0 * b.p.1;
        let det_b = b.a.0 * b.p.1 - b.p.0 * b.a.1;

        if det_a % det_l == 0 {
            let a = det_a / det_l;
            if det_b % det_l == 0 {
                let b = det_b / det_l;
                // let sol = (a as T, b as T);
                sum += a as T * 3 + b as T;
            }
        }
    }

    sum
}
