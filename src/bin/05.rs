use text_io::scan;

fn main() {
    let mut input = include_str!("../../input/05.txt").lines();

    let rules: Vec<(u32, u32)> = input
        .by_ref()
        .take_while(|l| *l != "")
        .into_iter()
        .map(|line| {
            let (l, r): (u32, u32);
            scan!(line.bytes() => "{}|{}", l, r);
            (l, r)
        })
        .collect();

    let updates: Vec<Vec<u32>> = input
        .skip(1)
        .map(|l| {
            l.split(',')
                .into_iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    println!("{}", part1(&rules, &updates));
    println!("{}", part2(&rules, &mut updates.clone()));
}

fn part2(rules: &Vec<(u32, u32)>, updates: &mut Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for u in updates.iter_mut() {
        let mut swapped = false;
        for i in 0..u.len() {
            for j in i..u.len() {
                if rules.contains(&(u[j], u[i])) {
                    u.swap(i, j);
                    swapped = true;
                }
            }
        }
        if swapped {
            sum += u[u.len() / 2];
        }
    }

    sum
}

fn part1(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    'outer: for u in updates {
        for i in 0..u.len() {
            for j in i..u.len() {
                if rules.contains(&(u[j], u[i])) {
                    continue 'outer;
                }
            }
        }
        sum += u[u.len() / 2];
    }

    sum
}
