use std::collections::BTreeSet;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 14,
    name: "Parabolic Reflector Dish",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: u8,
    x: u8,
}

impl Point {
    fn step_north(self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self {
                y: self.y - 1,
                x: self.x,
            })
        }
    }
}

#[derive(Debug)]
struct Map {
    squares: BTreeSet<Point>,
    rounds: Vec<Point>,
    height: u8,
}

fn parse(input: &str) -> Result<Map> {
    let mut squares = BTreeSet::new();
    let mut rounds = Vec::new();
    let mut height = 0;

    for (line, y) in input.trim().lines().map(str::trim).zip(0..) {
        for (b, x) in line.bytes().zip(0..) {
            let p = Point { x, y };
            match b {
                b'#' => {
                    squares.insert(p);
                }
                b'O' => rounds.push(p),
                _ => {}
            }
        }
        height = y;
    }

    Ok(Map {
        squares,
        rounds,
        height,
    })
}

fn part1(map: &Map) -> u32 {
    let mut rounds = BTreeSet::new();
    let mut load = 0;

    for &round in &map.rounds {
        let mut pos = round;
        loop {
            let Some(next) = pos.step_north() else { break };
            if rounds.contains(&next) || map.squares.contains(&next) {
                break;
            }
            pos = next;
        }

        rounds.insert(pos);

        load += (map.height + 1 - pos.y) as u32;
    }

    load
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
        let expected = 136;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
