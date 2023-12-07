use std::cmp::Ordering;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 7,
    name: "Camel Cards",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(&data, Hand::p1_cmp)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(&data, Hand::p2_cmp)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl Card {
    fn p1_cmp(self, other: Self) -> Ordering {
        (self as u8).cmp(&(other as u8))
    }

    fn p2_cmp(self, other: Self) -> Ordering {
        match (self, other) {
            (Card::Joker, Card::Joker) => Ordering::Equal,
            (_, Card::Joker) => Ordering::Greater,
            (Card::Joker, _) => Ordering::Less,
            _ => (self as usize).cmp(&(other as usize)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
    fn get_type_p1(self) -> Type {
        let mut counts = [0; 13];
        self.0
            .into_iter()
            .map(|c| c as usize)
            .for_each(|i| counts[i] += 1);

        counts.sort_unstable();
        count_to_type(counts)
    }

    fn p1_cmp(self, other: Self) -> Ordering {
        self.get_type_p1().cmp(&other.get_type_p1()).then_with(|| {
            self.0
                .into_iter()
                .zip(other.0)
                .fold(Ordering::Equal, |acc, (a, b)| acc.then(a.p1_cmp(b)))
        })
    }

    fn get_type_p2(&self) -> Type {
        let mut counts = [0; 13];
        self.0
            .into_iter()
            .map(|c| c as usize)
            .for_each(|i| counts[i] += 1);

        let count_joker = counts[Card::Joker as usize];
        counts[Card::Joker as usize] = 0;

        (0..counts.len())
            .filter(|i| *i != Card::Joker as usize)
            .map(|i| {
                let mut local_counts = counts;
                local_counts[i] += count_joker;
                local_counts.sort_unstable();
                count_to_type(local_counts)
            })
            .max()
            .unwrap()
    }

    fn p2_cmp(self, other: Self) -> Ordering {
        self.get_type_p2().cmp(&other.get_type_p2()).then_with(|| {
            self.0
                .into_iter()
                .zip(other.0)
                .fold(Ordering::Equal, |acc, (a, b)| acc.then(a.p2_cmp(b)))
        })
    }
}

fn count_to_type(counts: [i32; 13]) -> Type {
    match counts {
        [.., 5] => Type::FiveOfAKind,
        [.., 4] => Type::FourOfAKind,
        [.., 2, 3] => Type::FullHouse,
        [.., 3] => Type::ThreeOfAKind,
        [.., 2, 2] => Type::TwoPair,
        [.., 2] => Type::OnePair,
        [..] => Type::HighCard,
    }
}

#[derive(Debug, Clone)]
struct Play {
    hand: Hand,
    bid: u16,
}

fn parse_card(b: u8) -> Card {
    match b {
        b'A' => Card::Ace,
        b'K' => Card::King,
        b'Q' => Card::Queen,
        b'J' => Card::Joker,
        b'T' => Card::Ten,
        b'9' => Card::Nine,
        b'8' => Card::Eight,
        b'7' => Card::Seven,
        b'6' => Card::Six,
        b'5' => Card::Five,
        b'4' => Card::Four,
        b'3' => Card::Three,
        b'2' => Card::Two,
        _ => unreachable!(),
    }
}

fn parse_hand(h: &str) -> Hand {
    Hand(<[u8; 5]>::try_from(h.as_bytes()).unwrap().map(parse_card))
}

fn parse(input: &str) -> Result<Vec<Play>> {
    input
        .lines()
        .map(|l| -> Result<Play> {
            let (hand, bid) = l.trim().split_once(' ').unwrap();

            Ok(Play {
                hand: parse_hand(hand),
                bid: bid.parse().unwrap(),
            })
        })
        .collect()
}

fn solve(plays: &[Play], sortfn: fn(Hand, Hand) -> Ordering) -> u32 {
    let mut plays = plays.to_owned();
    plays.sort_unstable_by(|a, b| sortfn(a.hand, b.hand));
    let len = plays.len() as u32;

    plays
        .into_iter()
        .zip(1..=len)
        .map(|(play, mult)| play.bid as u32 * mult)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_type_test() {
        let tests = [
            ("AAAAA", Type::FiveOfAKind),
            ("AA8AA", Type::FourOfAKind),
            ("23332", Type::FullHouse),
            ("TTT98", Type::ThreeOfAKind),
            ("23423", Type::TwoPair),
            ("A23A4", Type::OnePair),
            ("23456", Type::HighCard),
        ];

        for (hand, expected_type) in tests {
            let parsed = parse_hand(hand);
            let actual = parsed.get_type_p1();
            assert_eq!(expected_type, actual, "hand");
        }
    }

    #[test]
    fn part1_strength_test() {
        let tests = [("33332", "2AAAA"), ("77888", "77788")];
        for (a, b) in tests {
            let a_hand = parse_hand(a);
            let b_hand = parse_hand(b);
            assert!(a_hand.p1_cmp(b_hand).is_gt(), "{a} > {b}");
        }
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 6440;
        let actual = solve(&parsed, Hand::p1_cmp);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_type_test() {
        let tests = [
            ("QJJQ2", Type::FourOfAKind),
            ("32T3K", Type::OnePair),
            ("KK677", Type::TwoPair),
            ("T55J5", Type::FourOfAKind),
            ("KTJJT", Type::FourOfAKind),
            ("QQQJA", Type::FourOfAKind),
        ];

        for (hand, expected_type) in tests {
            let parsed = parse_hand(hand);
            let actual = parsed.get_type_p2();
            assert_eq!(expected_type, actual, "{hand}");
        }
    }

    #[test]
    fn part2_strength_test() {
        let tests = [("QQQQ2", "JKKK2")];
        for (a, b) in tests {
            let a_hand = parse_hand(a);
            let b_hand = parse_hand(b);
            assert!(a_hand.p2_cmp(b_hand).is_gt(), "{a} > {b}");
        }
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 5905;
        let actual = solve(&parsed, Hand::p2_cmp);

        assert_eq!(expected, actual);
    }
}
