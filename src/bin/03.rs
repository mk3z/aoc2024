use regex::Regex;

fn main() {
    let input = include_str!("../../input/03.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part2(line: &str) -> i32 {
    part1(
        &Regex::new(r"(don't\(\).*?do\(\))|(don't\(\).*?$)")
            .unwrap()
            .replace_all(&line, "")
            .into_owned(),
    )
}

fn part1(string: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(&string)
        .map(|c| c.extract::<2>().1)
        .map(|[a, b]| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum()
}
