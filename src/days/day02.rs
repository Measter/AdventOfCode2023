use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 2,
    name: "Cube Conundrum",
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

#[derive(Debug, Clone, Copy, Default)]
struct Play {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    plays: Vec<Play>,
}

fn parse(input: &str) -> Result<Vec<Game>> {
    let mut games = Vec::new();

    for line in input.lines().map(str::trim) {
        let (game, playlist) = line.split_once(':').unwrap();
        let mut game = Game {
            id: game.strip_prefix("Game ").unwrap().parse()?,
            plays: Vec::new(),
        };

        for play_str in playlist.split(';') {
            let mut play = Play::default();
            for dice in play_str.split(',') {
                let (count, colour) = dice.trim().split_once(' ').unwrap();
                let count = count.trim().parse().unwrap();
                match colour.trim() {
                    "red" => play.red = count,
                    "green" => play.green = count,
                    "blue" => play.blue = count,
                    _ => unreachable!(),
                }
            }
            game.plays.push(play);
        }

        games.push(game);
    }

    Ok(games)
}

fn part1(games: &[Game]) -> u16 {
    const MAX_RED: u8 = 12;
    const MAX_GREEN: u8 = 13;
    const MAX_BLUE: u8 = 14;

    let mut total = 0;

    for game in games {
        let total_play = game.plays.iter().fold(Play::default(), |a, p| Play {
            red: a.red.max(p.red),
            green: a.green.max(p.green),
            blue: a.blue.max(p.blue),
        });
        if total_play.red <= MAX_RED && total_play.green <= MAX_GREEN && total_play.blue <= MAX_BLUE
        {
            total += game.id as u16;
        }
    }

    total
}

fn part2(games: &[Game]) -> u16 {
    let mut total = 0;

    for game in games {
        let total_play = game.plays.iter().fold(Play::default(), |a, p| Play {
            red: a.red.max(p.red),
            green: a.green.max(p.green),
            blue: a.blue.max(p.blue),
        });

        total += (total_play.red as u16) * (total_play.green as u16) * (total_play.blue as u16);
    }

    total
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
        let expected = 8;
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
        let expected = 2286;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
