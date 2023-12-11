use std::cmp::{Ord, Ordering};
use std::convert::From;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand([Card; 5]);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<Hand> for Kind {
    fn from(value: Hand) -> Self {
        let mut hand = value.0;
        hand.sort();
        if hand[0] == hand[4] {
            Self::FiveOfAKind
        } else if hand[0] == hand[3] || hand[1] == hand[4] {
            Self::FourOfAKind
        } else if hand[0] == hand[2] && hand[3] == hand[4]
            || hand[0] == hand[1] && hand[2] == hand[4]
        {
            Self::FullHouse
        } else if hand[0] == hand[2] || hand[1] == hand[3] || hand[2] == hand[4] {
            Self::ThreeOfAKind
        } else if hand[0] == hand[1] && (hand[2] == hand[3] || hand[3] == hand[4])
            || hand[1] == hand[2] && hand[3] == hand[4]
        {
            Self::TwoPair
        } else if hand[0] == hand[1]
            || hand[1] == hand[2]
            || hand[2] == hand[3]
            || hand[3] == hand[4]
        {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            let kind1: Kind = self.clone().into();
            let kind2: Kind = other.clone().into();
            if kind1 < kind2 {
                Some(Ordering::Less)
            } else if kind1 > kind2 {
                Some(Ordering::Greater)
            } else {
                Some(self.0.cmp(&other.0))
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let kind1: Kind = self.clone().into();
        let kind2: Kind = other.clone().into();
        kind1 == kind2 && self.0 == other.0
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_hand(s: &str) -> Hand {
    let mut it = s.chars().map(|c| c.into());
    Hand(std::array::from_fn(|_| it.next().unwrap()))
}

fn parse(s: &str) -> (Hand, u32) {
    let (hand, bid) = s.split_once(' ').unwrap();
    (parse_hand(hand), bid.parse().unwrap())
}

fn solve_1(input: &str) -> u32 {
    let mut sorted = input.lines().map(|l| parse(l)).collect::<Vec<_>>();
    sorted.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
    sorted
        .into_iter()
        .enumerate()
        .map(|(r, (_, b))| ((r + 1) as u32) * b)
        .sum()
}

fn solve_2(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_1(&input));
    println!("{}", solve_2(&input));
}
