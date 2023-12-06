use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 6,
    name: "Wait For It",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse_p1(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse_p2(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse_p1(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u32,
    distance: u64,
}

fn parse_p1(input: &str) -> Result<Vec<Race>> {
    let input = input.trim();
    let split_pos = input.as_bytes().iter().position(|&b| b == b'\n').unwrap();

    let (time, distance) = input.split_at(split_pos);

    let times = time["time: ".len()..]
        .split_ascii_whitespace()
        .map(str::parse::<u32>);
    let distances = distance["distance: ".len()..]
        .split_ascii_whitespace()
        .map(str::parse::<u64>);

    times.zip(distances).try_fold(Vec::new(), |mut v, (t, d)| {
        v.push(Race {
            time: t?,
            distance: d?,
        });
        Ok(v)
    })
}

fn parse_p2(input: &str) -> Result<Race> {
    let input = input.trim();
    let split_pos = input.as_bytes().iter().position(|&b| b == b'\n').unwrap();

    let (time, distance) = input.split_at(split_pos);

    let time = time["time: ".len()..]
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .fold(0, |acc, b| acc * 10 + (b - b'0') as u32);
    let distance = distance["distance: ".len()..]
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .fold(0, |acc, b| acc * 10 + (b - b'0') as u64);

    Ok(Race { time, distance })
}

fn calc_race_distance(hold_time: u32, race_len: u32) -> u64 {
    hold_time as u64 * (race_len - hold_time) as u64
}

fn part1(races: &[Race]) -> usize {
    races
        .iter()
        .map(|r| {
            (1..r.time)
                .map(|i| calc_race_distance(i, r.time))
                .filter(|&l| l > r.distance)
                .count()
        })
        .product()
}

fn part2(race: Race) -> u32 {
    // Let's binary search this bugger! (Thanks Silmeth!)
    let bin_search = |f: fn(u64, u64) -> bool| {
        let mut left = 0;
        let mut right = race.time;
        while (right - left) > 1 {
            let mid = (right - left) / 2 + left;
            let distance = calc_race_distance(mid, race.time);
            if f(distance, race.distance) {
                right = mid;
            } else {
                left = mid;
            }
        }
        (left, right)
    };

    let lower = bin_search(|a, b| a > b).1;
    let higher = bin_search(|a, b| a < b).0;

    higher - lower + 1
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

        let parsed = parse_p1(&data).unwrap();
        let expected = 288;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse_p2(&data).unwrap();
        let expected = 71503;
        let actual = part2(parsed);

        assert_eq!(expected, actual);
    }
}
