use std::ops::RangeInclusive;

fn solve(a: f64, b: f64, c: f64) -> RangeInclusive<f64> {
    let delta = (b * b - 4. * a * c) as f64;
    let delta_sqrt = delta.sqrt();
    (((-b - delta_sqrt) / (2. * a)) + 1.)..=(((-b + delta_sqrt) / (2. * a)) - 1.)
}

fn solve_i64(a: i64, b: i64, c: i64) -> RangeInclusive<i64> {
    let (lower, upper) = solve(a as f64, b as f64, c as f64).into_inner();
    lower.floor() as i64..=upper.ceil() as i64
}

fn solve_1(input: &str) -> u32 {
    fn parse(s: &str) -> Vec<(u32, u32)> {
        fn parse(s: &str) -> impl Iterator<Item = u32> + '_ {
            s.trim().split_whitespace().map(|n| n.parse().unwrap())
        }
        let (times, distances) = s.split_once('\n').unwrap();
        let times = parse(times.strip_prefix("Time:").unwrap());
        let distances = parse(distances.strip_prefix("Distance:").unwrap());
        times.zip(distances).collect()
    }

    parse(input)
        .into_iter()
        .map(|(t, d)| solve_i64(1, -(t as i64), d as i64).count() as u32)
        .product()
}

fn solve_2(input: &str) -> u32 {
    fn parse(s: &str) -> (u64, u64) {
        fn parse(s: &str) -> u64 {
            s.replace(' ', "").parse().unwrap()
        }
        let (time, distance) = s.split_once('\n').unwrap();
        let time = parse(time.strip_prefix("Time:").unwrap());
        let distance = parse(distance.strip_prefix("Distance:").unwrap().trim_end());
        (time, distance)
    }
    let (time, distance) = parse(input);
    solve_i64(1, -(time as i64), distance as i64).count() as u32
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_1(&input));
    println!("{}", solve_2(&input));
}
