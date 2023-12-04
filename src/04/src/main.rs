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
    let mut counts = input
        .lines()
        .map(|l| {
            let (winning, owned) = l.split_once(':').unwrap().1.split_once(" | ").unwrap();
            let (winning, mut owned) = (parse(winning), parse(owned));
            owned.retain(|o| winning.iter().any(|w| w == o));
            (1u32, owned.len() as u32)
        }).collect::<Vec<_>>();
    
    fn recur(ns: &mut [(u32, u32)]) -> u32 {
        match ns.first() {
            Some((n, wins)) => {
                let n = *n;
                for i in 1..=*wins {
                    ns[i as usize].0 += n;
                }
                n + recur(&mut ns[1..])
            }
            None => 0,
        }
    }

    recur(&mut counts)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("1: {}", solve_1(input));
    println!("2: {}", solve_2(input));
}
