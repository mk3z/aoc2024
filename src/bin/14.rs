use itertools::Itertools;
use std::collections::HashSet;
use std::io;
use text_io::scan;

const LX: i32 = 101;
const LY: i32 = 103;
const DX: i32 = LX / 2;
const DY: i32 = LY / 2;
const STEP: i32 = 100;

fn main() {
    let input: Vec<((i32, i32), (i32, i32))> = include_str!("../../input/14.txt")
        .lines()
        .map(|l| {
            let (px, py, vx, vy);
            scan!(l.bytes() => "p={},{} v={},{}", px, py, vx, vy);
            ((px, py), (vx, vy))
        })
        .collect();

    println!("{}", part1(input.clone()));
    part2(input);
}

fn print(robots: &Vec<((i32, i32), (i32, i32))>) -> () {
    let robots = robots
        .iter()
        .map(|(pos, _)| *pos)
        .collect::<HashSet<(i32, i32)>>();

    for y in 0..LY {
        for x in 0..LX {
            if robots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn part2(mut robots: Vec<((i32, i32), (i32, i32))>) -> () {
    let mut input = String::new();
    let mut i = 1;
    println!("Press any key (except power or reset) to advance one frame");
    loop {
        io::stdin().read_line(&mut input).unwrap();

        println!("\n\n{i}");
        robots = robots
            .iter()
            .map(|((px, py), (vx, vy))| {
                let (mut nx, mut ny) = (px + vx, py + vy);
                if *vx < 0 {
                    nx = nx.rem_euclid(LX);
                } else {
                    nx = nx % (LX);
                }
                if *vy < 0 {
                    ny = ny.rem_euclid(LY);
                } else {
                    ny = ny % (LY);
                }
                ((nx, ny), (*vx, *vy))
            })
            .collect_vec();

        print(&robots);
        i += 1;
    }
}

fn part1(robots: Vec<((i32, i32), (i32, i32))>) -> i32 {
    let quadrants = robots
        .iter()
        .map(|((px, py), (vx, vy))| {
            let (mut nx, mut ny) = (px + STEP * vx, py + STEP * vy);
            if *vx < 0 {
                nx = nx.rem_euclid(LX);
            } else {
                nx = nx % (LX);
            }
            if *vy < 0 {
                ny = ny.rem_euclid(LY);
            } else {
                ny = ny % (LY);
            }
            (nx, ny)
        })
        .fold((0, 0, 0, 0), |(ul, ur, ll, lr), (x, y)| {
            if x == DX || y == DY {
                (ul, ur, ll, lr)
            } else {
                match (x < DX, y < DY) {
                    (true, true) => (ul + 1, ur, ll, lr),
                    (false, true) => (ul, ur + 1, ll, lr),
                    (false, false) => (ul, ur, ll, lr + 1),
                    (true, false) => (ul, ur, ll + 1, lr),
                }
            }
        });

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}
