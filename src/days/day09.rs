use std::num::ParseIntError;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 9,
    name: "Mirage Maintenance",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<false>(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<true>(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<Vec<i64>>, ParseIntError> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
        })
        .collect()
}

fn solve<const ISP2: bool>(data: &[Vec<i64>]) -> i64 {
    let mut diffs: Vec<i64> = Vec::new();
    let mut last_nums = Vec::new();
    let mut ans = 0;

    for set in data {
        diffs.clear();
        last_nums.clear();

        diffs.extend(set);

        while diffs.iter().any(|&i| i != 0) {
            if ISP2 {
                last_nums.push(diffs[0]);
            }

            for (j, i) in (1..diffs.len()).zip(0..) {
                diffs[i] = diffs[j] - diffs[i];
            }

            if ISP2 {
                diffs.pop();
            } else {
                last_nums.push(diffs.pop().unwrap());
            }
        }

        if ISP2 {
            // Why no rreduce!
            let first = last_nums.pop().unwrap();
            ans += last_nums.iter().rfold(first, |acc, next| next - acc);
        } else {
            ans += last_nums.iter().rfold(0, |acc, next| acc + next);
        }
    }

    ans
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
        let expected = 114;
        let actual = solve::<false>(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 2;
        let actual = solve::<true>(&parsed);

        assert_eq!(expected, actual);
    }
}
