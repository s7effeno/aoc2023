use std::ops::RangeInclusive;

fn parse_map(s: &str) -> (&str, &str, Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>) {
    let mut it = s.lines();
    let (from, to) = it
        .next()
        .unwrap()
        .strip_suffix("map:")
        .unwrap()
        .split_once("-to-")
        .unwrap();
    (
        from,
        to.trim_end(),
        it.map(|l| {
            let mut entries = l.splitn(3, ' ');
            let (dest_start, source_start, len): (u64, u64, u64) = (
                entries.next().unwrap().parse().unwrap(),
                entries.next().unwrap().parse().unwrap(),
                entries.next().unwrap().parse().unwrap(),
            );
            (
                source_start..=source_start + len,
                dest_start..=dest_start + len,
            )
        })
        .collect::<Vec<_>>(),
    )
}

fn to_fn(ranges: &[(RangeInclusive<u64>, RangeInclusive<u64>)]) -> impl Fn(u64) -> u64 + '_ {
    move |x| {
        ranges
            .iter()
            .find(|(s, _)| s.contains(&x))
            .map(|(s, d)| d.start() + (x - s.start()))
            .unwrap_or(x)
    }
}

fn mapper<'a, I: Iterator<Item = &'a str> + Clone + 'static>(maps: I) -> Box<dyn Fn(u64) -> u64> {
    let mut maps = maps;
    let binding = maps.next().unwrap();
    let (_, to, ranges) = parse_map(&binding);
    let ranges = Box::new(ranges.clone());
    if to == "location" {
        Box::new(move |x| to_fn(&ranges)(x))
    } else {
        Box::new(move |x| mapper(maps.clone())(to_fn(&ranges)(x)))
    }
}

fn solve_1(input: &'static str) -> u64 {
    fn parse_seeds(s: &str) -> Vec<u64> {
        s.strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    }

    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = parse_seeds(seeds);
    let mapper = mapper(maps.split("\n\n"));
    seeds.into_iter().map(|s| mapper(s)).min().unwrap()
}

fn solve_2(input: &'static str) -> u64 {
    fn _parse_seeds(s: &str) -> Vec<RangeInclusive<u64>> {
        s.strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|arr| {
                let start = arr[0];
                let n = arr[1];
                let start = start.parse().unwrap();
                let n: u64 = n.parse().unwrap();
                start..=(start + n)
            })
            .collect()
    }

    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = _parse_seeds(seeds);
    let mapper = mapper(maps.split("\n\n"));
    seeds
        .into_iter()
        .map(|ss| {
            println!("seed");
            ss.map(|s| mapper(s)).min().unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}
