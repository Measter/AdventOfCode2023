use std::collections::HashSet;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 16,
    name: "The Floor Will Be Lava",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn step(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y.wrapping_sub(1),
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y.saturating_add(1),
            },
            Direction::Left => Self {
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x.saturating_add(1),
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    SplitHorizontal,
    SplitVertical,
    DiagLeft,  // '\'
    DiagRight, // '/'
}

struct Map {
    tiles: Vec<TileType>,
    width: u8,
    height: u8,
}

impl Map {
    fn contains(&self, p: Point) -> bool {
        (p.x < self.width) & (p.y < self.height)
    }

    fn to_idx(&self, p: Point) -> usize {
        p.y as usize * self.width as usize + p.x as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: Point,
    dir: Direction,
}

impl Beam {
    fn new(x: u8, y: u8, dir: Direction) -> Self {
        Self {
            pos: Point { x, y },
            dir,
        }
    }

    fn step(self) -> Self {
        Self {
            pos: self.pos.step(self.dir),
            dir: self.dir,
        }
    }

    fn with_dir(self, dir: Direction) -> Self {
        Self { pos: self.pos, dir }
    }
}

fn parse(input: &str) -> Result<Map> {
    let mut tiles = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (row, y) in input.trim().lines().zip(0..) {
        height = y;
        for (t, x) in row.trim().bytes().zip(0..) {
            width = x;
            let tt = match t {
                b'.' => TileType::Empty,
                b'|' => TileType::SplitVertical,
                b'-' => TileType::SplitHorizontal,
                b'\\' => TileType::DiagLeft,
                b'/' => TileType::DiagRight,
                _ => unreachable!(),
            };

            tiles.push(tt);
        }
    }

    Ok(Map {
        tiles,
        width: width + 1,
        height: height + 1,
    })
}

fn solve(
    map: &Map,
    start: Beam,
    energized: &mut [bool],
    beams: &mut Vec<Beam>,
    seen_beams: &mut HashSet<Beam>,
) -> u32 {
    use Direction::*;
    use TileType::*;

    let mut add_beam = |beam: Beam, beams: &mut Vec<Beam>| {
        if seen_beams.insert(beam) {
            beams.push(beam);
        }
    };

    add_beam(start, beams);

    while let Some(mut beam) = beams.pop() {
        while map.contains(beam.pos) {
            let idx = map.to_idx(beam.pos);
            energized[idx] = true;
            let new_dir = match (map.tiles[idx], beam.dir) {
                (Empty, _) | (SplitHorizontal, Left | Right) | (SplitVertical, Up | Down) => {
                    beam = beam.step();
                    continue;
                }
                (SplitHorizontal, Up | Down) => {
                    add_beam(beam.with_dir(Left), beams);
                    add_beam(beam.with_dir(Right), beams);
                    break;
                }
                (SplitVertical, Left | Right) => {
                    add_beam(beam.with_dir(Up), beams);
                    add_beam(beam.with_dir(Down), beams);
                    break;
                }
                (DiagLeft, Up) => Left,
                (DiagLeft, Down) => Right,
                (DiagLeft, Left) => Up,
                (DiagLeft, Right) => Down,
                (DiagRight, Up) => Right,
                (DiagRight, Down) => Left,
                (DiagRight, Left) => Down,
                (DiagRight, Right) => Up,
            };

            add_beam(beam.with_dir(new_dir).step(), beams);
            break;
        }
    }

    energized.iter().map(|&b| b as u32).sum()
}

fn part1(map: &Map) -> u32 {
    let mut energized = vec![false; map.tiles.len()];
    let mut beams = Vec::new();
    let mut seen_beams = HashSet::new();

    solve(
        map,
        Beam::new(0, 0, Direction::Right),
        &mut energized,
        &mut beams,
        &mut seen_beams,
    )
}

fn part2(map: &Map) -> u32 {
    let mut energized = vec![false; map.tiles.len()];
    let mut beams = Vec::new();
    let mut seen_beams = HashSet::new();

    let mut max_energized = 0;

    for y in 0..map.height {
        energized.fill(false);
        beams.clear();
        seen_beams.clear();
        max_energized = max_energized.max(solve(
            map,
            Beam::new(0, y, Direction::Right),
            &mut energized,
            &mut beams,
            &mut seen_beams,
        ));

        energized.fill(false);
        beams.clear();
        seen_beams.clear();
        max_energized = max_energized.max(solve(
            map,
            Beam::new(map.width - 1, y, Direction::Left),
            &mut energized,
            &mut beams,
            &mut seen_beams,
        ));
    }

    for x in 0..map.width {
        energized.fill(false);
        beams.clear();
        seen_beams.clear();
        max_energized = max_energized.max(solve(
            map,
            Beam::new(x, 0, Direction::Down),
            &mut energized,
            &mut beams,
            &mut seen_beams,
        ));

        energized.fill(false);
        beams.clear();
        seen_beams.clear();
        max_energized = max_energized.max(solve(
            map,
            Beam::new(x, map.height - 1, Direction::Up),
            &mut energized,
            &mut beams,
            &mut seen_beams,
        ));
    }

    max_energized
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
        let expected = 46;
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
        let expected = 51;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
