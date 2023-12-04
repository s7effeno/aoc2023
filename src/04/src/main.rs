fn parse(s: &str) -> Vec<u32> {
    s.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (winning, owned) = l.split_once(':').unwrap().1.split_once(" | ").unwrap();
            let (winning, mut owned) = (parse(winning), parse(owned));
            owned.retain(|o| winning.iter().any(|w| w == o));
            match owned.len() {
                0 => 0,
                n => 2u32.pow(n as u32 - 1),
            }
        })
        .sum()
}

fn solve_2(input: &str) -> u32 {
    let mut counts = Vec::new();
    for (i, n) in input
        .lines()
        .map(|l| {
            let (winning, owned) = l.split_once(':').unwrap().1.split_once(" | ").unwrap();
            let (winning, mut owned) = (parse(winning), parse(owned));
            owned.retain(|o| winning.iter().any(|w| w == o));
            owned.len()
        })
        .enumerate()
    {
        if counts.get(i).is_none() {
            counts.push(0);
        }
        counts[i] += 1;
        for j in 1..=n {
            if counts.get(i + j).is_none() {
                counts.push(0);
            }
            counts[i + j] += counts[i];
        }
    }
    counts.into_iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("1: {}", solve_1(input));
    println!("2: {}", solve_2(input));
}
