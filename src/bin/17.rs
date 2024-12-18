use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use text_io::scan;

#[derive(Debug, Clone)]
struct Computer {
    tape: Vec<u32>,
    len: usize,
    output: Vec<u32>,
    ip: usize,
    a: u32,
    b: u32,
    c: u32,
}

impl Computer {
    fn new(tape: Vec<u32>, a: u32, b: u32, c: u32) -> Self {
        let len = tape.len();
        Computer {
            tape,
            len,
            output: Vec::new(),
            ip: 0,
            a,
            b,
            c,
        }
    }

    fn reset(&mut self, a: u32, b: u32, c: u32) {
        self.output = Vec::new();
        self.ip = 0;
        self.a = a;
        self.b = b;
        self.c = c;
    }

    fn part1(&mut self) {
        loop {
            if self.ip >= self.len {
                break;
            }

            self.step();
        }
    }

    // This was solved using z3, burte force was unviable
    fn part2_brute(&mut self) -> Option<u32> {
        // This would have to be u64::MAX
        (0..u32::MAX).into_par_iter().find_first(|a| {
            let mut c = self.clone();
            c.a = *a;

            let mut i = 0;
            loop {
                // If there is output
                if c.step() {
                    // If chars don't match, break
                    if c.output[i] != c.tape[i] {
                        return false;
                    }

                    // If all chars until now have matched and we are at target length, return positive
                    if i == c.len - 1 {
                        dbg!(a, &c.output);
                        return true;
                    }

                    // Increment position in output
                    i += 1;
                }

                // If program ended, exit
                if c.ip >= c.len {
                    return false;
                }
            }
        })
    }

    fn step(&mut self) -> bool {
        let op = self.tape[self.ip];
        let li = self.tape[self.ip + 1];
        let co = match li {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            n => n,
        };
        match op {
            0 => self.a /= 2u32.pow(co),
            1 => self.b ^= li,
            2 => self.b = co % 8,
            3 if self.a != 0 => self.ip = co as usize,
            4 => self.b ^= self.c,
            5 => self.output.push(co % 8),
            6 => self.b = self.a / 2u32.pow(co),
            7 => self.c = self.a / 2u32.pow(co),
            _ => {}
        }

        if !(op == 3 && self.a != 0) {
            self.ip += 2;
        }

        if op == 5 {
            return true;
        }

        false
    }

    fn print(&self) -> String {
        self.tape
            .iter()
            .tuples()
            .map(|(op, li)| {
                let co = match li {
                    4 => "A",
                    5 => "B",
                    6 => "C",
                    n => &n.to_string(),
                };

                match op {
                    0 => format!("A = A / 2^{co}\n"),
                    1 => format!("B = B xor {li}\n"),
                    2 => format!("B = {co} % 8\n"),
                    3 => format!("if A != 0 jmp {li}\n"),
                    4 => format!("B = B xor C\n"),
                    5 => format!("print {co} % 8\n"),
                    6 => format!("B = A / 2^{co}\n"),
                    7 => format!("C = A / 2^{co}\n"),
                    _ => panic!(),
                }
            })
            .collect::<String>()
    }
}

fn main() {
    let input = include_str!("../../input/17.txt")
        .split("\n\n")
        .collect_vec();

    let (a, b, c): (u32, u32, u32);
    scan!(input[0].bytes() => "Register A: {}\nRegister B: {}\nRegister C: {}", a, b, c);

    let tape = input[1]
        .split_whitespace()
        .skip(1)
        .map(|l| l.split(",").map(|c| c.parse().unwrap()))
        .flatten()
        .collect_vec();

    let mut computer = Computer::new(tape, a, b, c);

    computer.part1();
    println!("{}", computer.output.iter().join(","));

    // Reverse engineering help
    // println!("{}", computer.print());
}
