fn parse(s: &str) -> Vec<(u32, u32, u32)> {
    fn index(color: &str) -> usize {
        match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => unreachable!(),
        }
    }

    s.split("; ")
        .map(|handful| {
            let mut counts = [0, 0, 0];
            let color = handful.splitn(3, ", ");
            color.for_each(|c| {
                let (count, color) = c.split_once(' ').unwrap();
                counts[index(color)] = count.parse().unwrap();
            });
            (counts[0], counts[1], counts[2])
        })
        .collect::<Vec<_>>()
}

fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse(l.split_once(": ").unwrap().1))
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|(red, green, blue)| *red <= 12 && *green <= 13 && *blue <= 14)
        })
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|l| {
            parse(l.split_once(": ").unwrap().1)
                .into_iter()
                .reduce(|max, (r, g, b)| (max.0.max(r), max.1.max(g), max.2.max(b)))
        })
        .map(|v| v.0 * v.1 * v.2)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("1: {}", solve_1(input));
    println!("2: {}", solve_2(input));
}
