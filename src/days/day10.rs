use std::collections::BinaryHeap;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use smallvec::SmallVec;

pub const DAY: Day = Day {
    day: 10,
    name: "Pipe Maze",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0b0001,
    South = 0b0010,
    East = 0b0100,
    West = 0b1000,
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    dir_map: u8,
}

impl Pipe {
    fn has_dir(self, dir: Direction) -> bool {
        (self.dir_map & dir as u8) != 0
    }

    fn render(self) -> char {
        match self.dir_map {
            0b0011 => '|',
            0b1100 => '-',
            0b0101 => 'L',
            0b1001 => 'J',
            0b1010 => '7',
            0b0110 => 'F',
            _ => '.',
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    pipes: Vec<Pipe>,
    width: u8,
    height: u8,
    start: Point,
}

impl Map {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn neighbours(self, max_width: u8, max_height: u8) -> SmallVec<[Point; 4]> {
        [(0xff, 0), (1, 0), (0, 0xff), (0, 1)]
            .into_iter()
            .filter_map(|(x, y)| {
                Some((
                    if self.x.wrapping_add(x) >= max_width {
                        return None;
                    } else {
                        self.x.wrapping_add(x)
                    },
                    if self.y.wrapping_add(y) >= max_height {
                        return None;
                    } else {
                        self.y.wrapping_add(y)
                    },
                ))
            })
            .map(|(x, y)| Point::new(x, y))
            .collect()
    }
}

impl Map {
    fn connect_dir(&mut self, dir: Direction, point: Point) {
        let idx = self.idx_of(point);
        self.pipes[idx].dir_map |= dir as u8;
    }

    fn idx_of(&self, point: Point) -> usize {
        point.y as usize * self.width as usize + point.x as usize
    }

    fn get(&self, point: Point) -> Pipe {
        self.pipes[self.idx_of(point)]
    }

    fn can_traverse(&self, from: Point, to: Point) -> bool {
        let lr_diff = from.x.max(to.x) - from.x.min(to.x);
        let ud_diff = from.y.max(to.y) - from.y.min(to.y);

        // Not neighbours
        if lr_diff > 1 || ud_diff > 1 || (lr_diff == 1 && ud_diff == 1) {
            return false;
        }

        let [from_dir, to_dir] = if lr_diff == 0 {
            if from.y > to.y {
                [Direction::North, Direction::South]
            } else {
                [Direction::South, Direction::North]
            }
        } else {
            #[allow(clippy::collapsible_else_if)] // Shush!
            if from.x > to.x {
                [Direction::West, Direction::East]
            } else {
                [Direction::East, Direction::West]
            }
        };

        self.get(from).has_dir(from_dir) && self.get(to).has_dir(to_dir)
    }

    #[allow(unused)]
    fn render(&self) {
        for row in self.pipes.chunks_exact(self.width as usize) {
            for pipe in row {
                print!("{}", pipe.render())
            }
            eprintln!()
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct SearchState {
    cost: u16,
    pos: Point,
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Map> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut map = Map {
        pipes: vec![Pipe { dir_map: 0 }; width * height],
        width: width as u8,
        height: width as u8,
        start: Point::new(0, 0),
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            let point = Point::new(x as u8, y as u8);
            match c {
                b'|' => {
                    map.connect_dir(Direction::South, point);
                    map.connect_dir(Direction::North, point);
                }
                b'-' => {
                    map.connect_dir(Direction::East, point);
                    map.connect_dir(Direction::West, point);
                }
                b'L' => {
                    map.connect_dir(Direction::North, point);
                    map.connect_dir(Direction::East, point);
                }
                b'J' => {
                    map.connect_dir(Direction::North, point);
                    map.connect_dir(Direction::West, point);
                }
                b'7' => {
                    map.connect_dir(Direction::South, point);
                    map.connect_dir(Direction::West, point);
                }
                b'F' => {
                    map.connect_dir(Direction::South, point);
                    map.connect_dir(Direction::East, point);
                }
                b'S' => {
                    map.start = point;
                }
                _ => {}
            }
        }
    }

    // Need to connect up the start.
    if map.start.x > 0 {
        let west = Point::new(map.start.x - 1, map.start.y);
        if map.get(west).has_dir(Direction::East) {
            map.connect_dir(Direction::West, map.start);
        }
    }

    if map.start.x < map.width - 1 {
        let east = Point::new(map.start.x + 1, map.start.y);
        if map.get(east).has_dir(Direction::West) {
            map.connect_dir(Direction::East, map.start);
        }
    }

    if map.start.y > 0 {
        let north = Point::new(map.start.x, map.start.y - 1);
        if map.get(north).has_dir(Direction::South) {
            map.connect_dir(Direction::North, map.start);
        }
    }

    if map.start.y < map.height - 1 {
        let south = Point::new(map.start.x, map.start.y + 1);
        if map.get(south).has_dir(Direction::North) {
            map.connect_dir(Direction::South, map.start);
        }
    }

    Ok(map)
}

fn part1(map: &Map) -> u16 {
    let mut queue = BinaryHeap::new();
    let mut dist = vec![u16::MAX; map.width as usize * map.height as usize];

    dist[map.idx_of(map.start)] = 0;
    queue.push(SearchState {
        cost: 0,
        pos: map.start,
    });

    while let Some(next) = queue.pop() {
        for neighbour in next.pos.neighbours(map.width, map.height) {
            if !map.can_traverse(next.pos, neighbour) {
                continue;
            }

            let total_cost = next.cost + 1;
            let n_idx = map.idx_of(neighbour);
            if total_cost < dist[n_idx] {
                dist[n_idx] = total_cost;
                queue.push(SearchState {
                    cost: total_cost,
                    pos: neighbour,
                })
            }
        }
    }

    dist.into_iter().filter(|&i| i != u16::MAX).max().unwrap()
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
        parsed.render();
        let expected = 4;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test2() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 2)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        parsed.render();
        let expected = 8;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
