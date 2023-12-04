use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 4,
    name: "Scratchcards",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone)]
struct Card {
    winning: Vec<u8>,
    have: Vec<u8>,
}

fn parse(input: &str) -> Result<Vec<Card>> {
    let mut cards = Vec::new();

    for line in input.lines().map(str::trim) {
        let (_, numbers) = line["Card ".len()..].split_once(": ").unwrap();
        let (winning, have) = numbers.split_once('|').unwrap();

        let winning = winning
            .split_ascii_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<_, _>>()?;
        let have = have
            .split_ascii_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<_, _>>()?;

        cards.push(Card { winning, have });
    }

    Ok(cards)
}

fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| {
            let num_matchs = card
                .have
                .iter()
                .filter(|&n| card.winning.contains(n))
                .count() as u32;
            if num_matchs > 0 {
                u32::pow(2, num_matchs - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(cards: &[Card]) -> u32 {
    let win_values: Vec<_> = cards
        .iter()
        .map(|card| {
            card.have
                .iter()
                .filter(|&n| card.winning.contains(n))
                .count() as u8
        })
        .collect();

    let mut num_cards = vec![1; cards.len()];

    for (i, &win_count) in win_values.iter().enumerate() {
        if win_count == 0 {
            continue;
        }

        let this_count = num_cards[i];

        num_cards[i + 1..][..win_count as usize]
            .iter_mut()
            .for_each(|c| *c += this_count);
    }

    num_cards.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 13;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 30;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
