use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 1,
    name: "Trebuchet!?",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[
        ("Parse", run_parse),
        ("Perfect Hash (Part 2)", run_perf_hash),
    ],
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

fn run_perf_hash(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(perf_hash(input)))
}

fn parse(input: &str) -> Result<Vec<&str>> {
    Ok(input.lines().collect())
}

fn part1(lines: &[&str]) -> u16 {
    lines
        .iter()
        .map(|l| {
            let mut bytes = l.as_bytes().iter();
            let left = bytes
                .clone()
                .position(|b| b.is_ascii_digit())
                .unwrap_or_default();
            let right = bytes.rposition(|b| b.is_ascii_digit()).unwrap_or_default();
            let bytes = l.as_bytes();
            let left = (bytes[left] - b'0') as u16;
            let right = (bytes[right] - b'0') as u16;

            left * 10 + right
        })
        .sum()
}

fn part2(lines: &[&str]) -> u16 {
    lines
        .iter()
        .map(|line| {
            let mut left = None;
            let mut right = None;
            let mut bytes = line.as_bytes();

            while !bytes.is_empty() {
                match bytes {
                    [b @ b'0'..=b'9', ..] => {
                        left = left.or(Some((*b - b'0') as u16));
                        right = Some((*b - b'0') as u16);
                    }
                    [b'o', b'n', b'e', ..] => {
                        left = left.or(Some(1));
                        right = Some(1);
                    }
                    [b't', b'w', b'o', ..] => {
                        left = left.or(Some(2));
                        right = Some(2);
                    }
                    [b't', b'h', b'r', b'e', b'e', ..] => {
                        left = left.or(Some(3));
                        right = Some(3);
                    }
                    [b'f', b'o', b'u', b'r', ..] => {
                        left = left.or(Some(4));
                        right = Some(4);
                    }
                    [b'f', b'i', b'v', b'e', ..] => {
                        left = left.or(Some(5));
                        right = Some(5);
                    }
                    [b's', b'i', b'x', ..] => {
                        left = left.or(Some(6));
                        right = Some(6);
                    }
                    [b's', b'e', b'v', b'e', b'n', ..] => {
                        left = left.or(Some(7));
                        right = Some(7);
                    }
                    [b'e', b'i', b'g', b'h', b't', ..] => {
                        left = left.or(Some(8));
                        right = Some(8);
                    }
                    [b'n', b'i', b'n', b'e', ..] => {
                        left = left.or(Some(9));
                        right = Some(9);
                    }
                    _ => {}
                }
                bytes = &bytes[1..];
            }

            left.unwrap_or_default() * 10 + right.unwrap_or_default()
        })
        .sum()
}

fn perf_hash(input: &str) -> u16 {
    const LOOKUP: [u8; 256] = [
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 4, 5, 0, 31, 31, 31, 31, 31, 31, 15, 15, 31, 31, 10, 0,
        10, 10, 5, 31, 0, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
        31, 31,
    ];

    let mut sum = 0;
    let mut left = None;
    let mut right = 0;
    let mut bytes = input.as_bytes();

    'outer: while !bytes.is_empty() {
        match bytes {
            [b'\n', ..] => {
                sum += left.unwrap_or_default() * 10 + right;
                left = None;
            }
            [b @ b'0'..=b'9', ..] => {
                left = left.or(Some((*b - b'0') as u16));
                right = (*b - b'0') as u16;
            }
            [a, _, b, ..] => {
                const WORDS: [(u16, &str); 31] = [
                    (6, "six"),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (8, "eight"),
                    (7, "seven"),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (5, "five"),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (4, "four"),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (1, "one"),
                    (3, "three"),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (2, "two"),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (0, ""),
                    (9, "nine"),
                ];
                let key = LOOKUP[*a as usize] + LOOKUP[*b as usize];
                'block: {
                    let Some((val, word)) = WORDS.get(key as usize) else {
                        break 'block;
                    };
                    if !word.is_empty() && bytes.starts_with(word.as_bytes()) {
                        left = left.or(Some(*val));
                        right = *val;
                        bytes = &bytes[word.len() - 1..];
                        continue 'outer;
                    }
                }
            }
            _ => {}
        }
        bytes = &bytes[1..];
    }

    sum
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
        let expected = 142;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 281;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
