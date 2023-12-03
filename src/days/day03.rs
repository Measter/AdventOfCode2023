use std::ops::{Add, RangeInclusive};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 3,
    name: "Gear Ratios",
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

struct Schematic<'a> {
    data: Vec<&'a [u8]>,
    width: usize,
}

fn is_symbol(b: u8) -> bool {
    !b.is_ascii_digit() && b != b'.'
}

fn get_number(chunk: &[u8]) -> u32 {
    let mut num = 0;
    for b in chunk.iter().take_while(|b| b.is_ascii_digit()) {
        num = num * 10 + (b - b'0') as u32;
    }
    num
}

fn left_search(line: &[u8]) -> usize {
    let (last_idx, _) = line
        .iter()
        .enumerate()
        .rev()
        .take_while(|(_, b)| b.is_ascii_digit())
        .last()
        .unwrap();
    last_idx
}

impl Schematic<'_> {
    fn is_beside_symbol(&self, y: usize, x_range: RangeInclusive<usize>) -> bool {
        let left_most = x_range.start().saturating_sub(1);
        let right_most = x_range.end().add(1).min(self.width - 1);
        // dbg!(y, left_most, right_most, self.width);

        let same_line = self.data[y];

        let line_up = y.saturating_sub(1);
        let line_down = y.add(1).min(self.data.len() - 1);

        let left_right_symbol = is_symbol(same_line[left_most]) | is_symbol(same_line[right_most]);
        let up_symbol = self.data[line_up][left_most..=right_most]
            .iter()
            .fold(false, |acc, b| acc | is_symbol(*b));
        let down_symbol = self.data[line_down][left_most..=right_most]
            .iter()
            .fold(false, |acc, b| acc | is_symbol(*b));

        left_right_symbol | up_symbol | down_symbol
    }

    fn get_neighbouring_numbers(&self, buff: &mut Vec<u32>, y: usize, x: usize) {
        let same_line = self.data[y];

        // Left and right are easy because they are definitely separate.
        if let Some(b'0'..=b'9') = same_line.get(x + 1) {
            buff.push(get_number(&same_line[x + 1..]));
        }

        if let Some(b'0'..=b'9') = same_line.get(x.wrapping_sub(1)) {
            let start_idx = left_search(&same_line[..x]);
            buff.push(get_number(&same_line[start_idx..]));
        }

        let left_most = x.saturating_sub(1);
        let right_most = x.add(1).min(self.width - 1);
        let line_up = self.data[y.saturating_sub(1)];
        let line_down = self.data[y.add(1).min(self.data.len() - 1)];
        let a = [line_up, line_down];
        let b = [line_down];
        let c = [line_up];
        let lines: &[&[u8]] = if y > 0 && y < self.data.len() {
            &a
        } else if y == 0 {
            &b
        } else {
            &c
        };
        for line in lines {
            let over_chars = &line[left_most..=right_most];

            match over_chars {
                // On a border
                [a, _] | [_, a] if a.is_ascii_digit() => {
                    let idx = left_most + line[1].is_ascii_digit() as usize;
                    let start_idx = left_search(&line[..=idx]);
                    buff.push(get_number(&line[start_idx..]));
                }
                // Definitely only have a single number.
                [_, a, _] if a.is_ascii_digit() => {
                    let start_idx = left_search(&line[..=x]);
                    buff.push(get_number(&line[start_idx..]));
                }
                // Two separated numbers.
                [a, _, b] => {
                    if a.is_ascii_digit() {
                        let start_idx = left_search(&line[..x]);
                        buff.push(get_number(&line[start_idx..]));
                    }
                    if b.is_ascii_digit() {
                        buff.push(get_number(&line[x + 1..]));
                    }
                }
                _ => {}
            }
        }
    }
}

fn parse(input: &str) -> Result<Schematic> {
    let data: Vec<&[u8]> = input
        .trim()
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect();
    let width = data[0].len();
    Ok(Schematic { data, width })
}

fn part1(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for (y, line) in schematic.data.iter().enumerate() {
        let mut line_iter = line.iter().enumerate();
        while let Some((x_start, &b)) = line_iter.next() {
            if !b.is_ascii_digit() {
                continue;
            }

            let mut num = (b - b'0') as u32;
            let mut x_end = x_start;
            while let Some((x, &n @ b'0'..=b'9')) = line_iter.next() {
                num = num * 10 + (n - b'0') as u32;
                x_end = x;
            }

            if schematic.is_beside_symbol(y, x_start..=x_end) {
                sum += num;
            }
        }
    }

    sum
}

fn part2(schematic: &Schematic) -> u32 {
    let mut sum = 0;
    let mut buff = Vec::with_capacity(10);

    for (y, line) in schematic.data.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != b'*' {
                continue;
            }

            buff.clear();
            schematic.get_neighbouring_numbers(&mut buff, y, x);
            if let [a, b] = buff.as_slice() {
                sum += a * b;
            }
        }
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
        let expected = 4361;
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
        let expected = 467_835;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
