use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 13,
    name: "Point of Incidence",
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

struct Block {
    by_row: Vec<u32>,
    // This is by_row rotated 90 degrees anticlockwise then reversed
    by_col: Vec<u32>,
}

fn parse(input: &str) -> Result<Vec<Block>> {
    let mut blocks = Vec::new();

    for block in input.split("\n\n").map(str::trim) {
        let mut by_row = Vec::new();
        let mut width = 0;
        for line in block.lines() {
            let v = line
                .as_bytes()
                .iter()
                .fold(0, |acc, b| (acc << 1) | (*b == b'.') as u32);

            width = line.len();

            by_row.push(v);
        }

        let mut by_col = Vec::new();
        for bit in 0..width {
            let v = by_row.iter().fold(0, |acc, r| {
                let bit = r & (1 << bit);
                (acc << 1) | (bit != 0) as u32
            });

            by_col.push(v);
        }
        by_col.reverse();

        blocks.push(Block { by_row, by_col });
    }

    Ok(blocks)
}

fn mirror_search_p1(vals: &[u32]) -> Option<usize> {
    for row in 1..vals.len() {
        // There'll always been at least one in each group.
        let (upper, lower) = vals.split_at(row);

        // Trim to the same length
        let len = upper.len().min(lower.len());
        let upper = &upper[upper.len() - len..];
        let lower = &lower[..len];

        if upper.iter().rev().eq(lower) {
            return Some(row);
        }
    }
    None
}

fn part1(blocks: &[Block]) -> usize {
    let mut sum = 0;

    for block in blocks {
        if let Some(r) = mirror_search_p1(&block.by_row) {
            sum += r * 100;
        }

        if let Some(c) = mirror_search_p1(&block.by_col) {
            sum += c;
        }
    }

    sum
}

fn mirror_search_p2(vals: &[u32]) -> Option<usize> {
    for row in 1..vals.len() {
        // There'll always been at least one in each group.
        let (upper, lower) = vals.split_at(row);

        // Trim to the same length
        let len = upper.len().min(lower.len());
        let upper = &upper[upper.len() - len..];
        let lower = &lower[..len];

        let diff: u32 = upper
            .iter()
            .rev()
            .zip(lower)
            .map(|(u, l)| (u ^ l).count_ones())
            .sum();

        if diff == 1 {
            return Some(row);
        }
    }
    None
}

fn part2(blocks: &[Block]) -> usize {
    let mut sum = 0;

    for block in blocks {
        if let Some(r) = mirror_search_p2(&block.by_row) {
            sum += r * 100;
        }

        if let Some(c) = mirror_search_p2(&block.by_col) {
            sum += c;
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
        let expected = 405;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test_5nnn() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, "5nnn")
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 3;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test_sinsworth() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, "sinsworth")
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 709;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test_striped_monkey() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, "stripedmonkey")
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 6;
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
        let expected = 400;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
