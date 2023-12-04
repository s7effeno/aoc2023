fn solve_1(input: &str) -> u32 {
    fn first_digit<I: Iterator<Item = char>>(mut iter: I) -> u32 {
        iter.find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap()
    }
    input
        .lines()
        .map(|l| 10 * first_digit(l.chars()) + first_digit(l.chars().rev()))
        .sum()
}

fn solve_2(input: &str) -> u32 {
    fn first_digit(
        s: &str,
        single: impl Fn(&str) -> char,
        formatted: impl Fn(&str, &str) -> bool,
        next: impl Fn(&str) -> &str,
    ) -> u32 {
        single(s)
            .to_digit(10)
            .or_else(|| {
                [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .enumerate()
                .find(|(_, t)| formatted(s, t))
                .map(|(n, _)| n as u32 + 1)
            })
            .unwrap_or_else(|| first_digit(next(s), single, formatted, next))
    }

    input
        .lines()
        .map(|l| {
            10 * first_digit(
                l,
                |s| s.chars().next().unwrap(),
                |a, b| a.starts_with(b),
                |s| s.get(1..).unwrap(),
            ) + first_digit(
                l,
                |s| s.chars().last().unwrap(),
                |a, b| a.ends_with(b),
                |s| s.get(0..(s.len() - 1)).unwrap(),
            )
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("1: {}", solve_1(input));
    println!("2: {}", solve_2(input));
}
