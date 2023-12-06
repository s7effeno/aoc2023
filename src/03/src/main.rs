enum Token {
    Symbol,
    Number(u32),
}

fn parse(s: &str) -> Vec<Token> {
    /*let mut it = s.lines()
    .enumerate()
    .flat_map(|(row, l)| l.char_indices().map(|(col, c)| (row, col, c)))
    .peekable();*/

    let mut ret = Vec::new();
    while let Some((row, col, c)) = it.peek() {
        fn is_symbol(c: char) -> bool {
            matches!(c, '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=')
        }
        match c {
            '.' => {
                it.next();
                ()
            }
            c if is_symbol(*c) => {
                it.next();
                ret.push((col, Token::Symbol))
            }
            c if c.is_numeric() => {
                let mut n = String::new();
                while let Some((col, c)) = it.peek() {
                    if c.is() {
                    } else {
                        ret.push((col, Token::Number(s.parse().unwrap())))
                    }
                }
            }
        }
    }
    ret
}

fn solve_1(input: &str) -> u32 {}

fn solve_2(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}
