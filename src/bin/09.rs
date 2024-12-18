use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/09.txt")
        .trim_end()
        .chars()
        .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn print_disk(disk: &Vec<Option<usize>>) {
    println!(
        "{}",
        &disk
            .iter()
            .map(|o| o.map(|n| format!("{n},")).unwrap_or(".".to_owned()))
            .collect::<String>()
    );
}

fn part2(input: &Vec<usize>) -> usize {
    let mut disk: Vec<Option<usize>> = Vec::new();

    input.iter().enumerate().for_each(|(i, n)| {
        disk.extend(vec![if i % 2 == 0 { Some(i / 2) } else { None }; *n]);
    });

    let indexes = input
        .iter()
        .scan(0, |i, n| {
            let tmp = *i;
            *i += n;
            Some((tmp, n))
        })
        .collect_vec();
    // (start index, number of items)

    let mut files = indexes
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, (i, s))| (*i, **s))
        .collect_vec();

    let mut spaces = indexes
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, (i, s))| (*i, **s))
        .collect_vec();

    let mut s = 0;
    let mut f = files.len() - 1;

    while s < f {
        let start = files[f].0;
        let size = files[f].1;
        let end = start + size;

        if let Some((index, space)) = spaces
            .iter_mut()
            .enumerate()
            .skip(s)
            .find(|(_, (_, size))| size >= &files[f].1)
        {
            if start > space.0 {
                let offset = start - space.0;
                let src = start..end;
                for i in src {
                    disk.swap(i, i - offset);
                }

                files[f].1 = 0;
                (*space).1 -= size;
                (*space).0 += size;

                if space.1 == 0 && index == s {
                    s += 1;
                }
            }
        }

        f -= 1;
    }

    disk.iter()
        .enumerate()
        .map(|(i, n)| i * n.unwrap_or(0))
        .sum()
}

fn part1(input: &Vec<usize>) -> usize {
    let mut disk: Vec<Option<usize>> = Vec::new();

    input.iter().enumerate().for_each(|(i, n)| {
        disk.extend(vec![if i % 2 == 0 { Some(i / 2) } else { None }; *n]);
    });

    let mut i = input[0];
    let mut j = disk.len() - 1;

    while i < j {
        while let Some(_) = disk[i] {
            i += 1;
        }
        while disk[j] == None {
            j -= 1;
        }

        disk.swap(i, j);
        j -= 1;
        i += 1;
    }

    disk.iter().flatten().enumerate().map(|(i, n)| i * n).sum()
}
