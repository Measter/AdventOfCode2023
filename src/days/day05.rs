use std::fmt::Debug;
use std::marker::PhantomData;

use aoc_lib::{misc::ArrChunks, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 5,
    name: "If You Give A Seed A Fertilizer",
    part_1: run_part1,
    // Disabled so the bencher doesn't run it.
    // part_2: Some(run_part2),
    part_2: None,
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

#[derive(Debug, Clone, Copy)]
struct Id<T>(u64, PhantomData<T>);

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Id<T> {
    fn new(v: u64) -> Self {
        Self(v, PhantomData)
    }
}

#[derive(Debug, Clone, Copy)]
struct Seed;

#[derive(Debug, Clone, Copy)]
struct Soil;

#[derive(Debug, Clone, Copy)]
struct Fertilizer;

#[derive(Debug, Clone, Copy)]
struct Water;

#[derive(Debug, Clone, Copy)]
struct Light;

#[derive(Debug, Clone, Copy)]
struct Temperature;

#[derive(Debug, Clone, Copy)]
struct Humidity;

#[derive(Debug, Clone, Copy)]
struct Location;

#[derive(Debug, Clone, Copy)]
struct MapInner {
    in_start: u64,
    in_end: u64,
    out_start: u64,
}

impl MapInner {
    fn contains(&self, v: u64) -> bool {
        (self.in_start..self.in_end).contains(&v)
    }
}

#[derive(Debug)]
struct Map<T, U> {
    ranges: Vec<MapInner>,
    _phantom: PhantomData<(T, U)>,
}

impl<T, U> Default for Map<T, U> {
    fn default() -> Self {
        Self {
            ranges: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<T, U> Map<T, U> {
    fn map(&self, src: Id<T>) -> Id<U> {
        let Some(m) = self.ranges.iter().find(|m| m.contains(src.0)) else {
            return Id::new(src.0);
        };

        let new_id = src.0 - m.in_start + m.out_start;
        Id::new(new_id)
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<Id<Seed>>,
    seed_to_soil: Map<Seed, Soil>,
    soil_to_fertilizer: Map<Soil, Fertilizer>,
    fertilizer_to_water: Map<Fertilizer, Water>,
    water_to_light: Map<Water, Light>,
    light_to_temperature: Map<Light, Temperature>,
    temperature_to_humidity: Map<Temperature, Humidity>,
    humidity_to_location: Map<Humidity, Location>,
}

impl Almanac {
    fn seed_to_location(&self, seed: Id<Seed>) -> Id<Location> {
        let soil = self.seed_to_soil.map(seed);
        let fert = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fert);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temp);
        self.humidity_to_location.map(humidity)
    }
}

fn parse_map<T: Debug, U: Debug>(numbers: &str) -> Map<T, U> {
    let mut map = Map {
        ranges: Vec::new(),
        _phantom: PhantomData,
    };

    for line in numbers.lines().map(str::trim) {
        let mut numbers = line
            .split_ascii_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap);
        let dest_start = numbers.next().unwrap();
        let src_start = numbers.next().unwrap();
        let length = numbers.next().unwrap();

        map.ranges.push(MapInner {
            in_start: src_start,
            in_end: src_start + length,
            out_start: dest_start,
        });
    }

    map
}

fn parse(input: &str) -> Result<Almanac> {
    let (seeds, rest) = input["seeds: ".len()..].split_once('\n').unwrap();
    let mut chunks = rest.trim();

    let mut almanec = Almanac {
        seeds: seeds
            .split_ascii_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .map(Id::<Seed>::new)
            .collect(),
        ..Default::default()
    };

    for _ in 0..7 {
        let (name, rest) = chunks.split_once(" map:").unwrap();
        let numbers = match rest.as_bytes().iter().position(|b| b.is_ascii_alphabetic()) {
            Some(idx) => {
                let (a, b) = rest.split_at(idx);
                chunks = b.trim();
                a.trim()
            }
            None => rest.trim(),
        };

        match name {
            "seed-to-soil" => almanec.seed_to_soil = parse_map(numbers),
            "soil-to-fertilizer" => almanec.soil_to_fertilizer = parse_map(numbers),
            "fertilizer-to-water" => almanec.fertilizer_to_water = parse_map(numbers),
            "water-to-light" => almanec.water_to_light = parse_map(numbers),
            "light-to-temperature" => almanec.light_to_temperature = parse_map(numbers),
            "temperature-to-humidity" => almanec.temperature_to_humidity = parse_map(numbers),
            "humidity-to-location" => almanec.humidity_to_location = parse_map(numbers),
            _ => panic!("Unknown chunk {name:?}"),
        }
    }

    Ok(almanec)
}

fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|s| almanac.seed_to_location(*s))
        .min()
        .unwrap()
        .0
}

fn part2(almanac: &Almanac) -> u64 {
    ArrChunks::new(&almanac.seeds)
        .map(|[s, l]| s.0..s.0 + l.0)
        .map(|r| {
            r.map(Id::new)
                .map(|s| almanac.seed_to_location(s))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn map_test() {
        let map: Map<Seed, Soil> = parse_map("50 98 2\n52 50 48");

        // Lower Range
        let expected = Id::new(49);
        let actual = map.map(Id::new(49));
        assert_eq!(expected, actual);

        let expected = Id::new(52);
        let actual = map.map(Id::new(50));
        assert_eq!(expected, actual);

        let expected = Id::new(53);
        let actual = map.map(Id::new(51));
        assert_eq!(expected, actual);

        // Upper range
        let expected = Id::new(99);
        let actual = map.map(Id::new(97));
        assert_eq!(expected, actual);

        let expected = Id::new(50);
        let actual = map.map(Id::new(98));
        assert_eq!(expected, actual);

        let expected = Id::new(51);
        let actual = map.map(Id::new(99));
        assert_eq!(expected, actual);

        let expected = Id::new(100);
        let actual = map.map(Id::new(100));
        assert_eq!(expected, actual);

        // Examples
        let expected = Id::new(81);
        let actual = map.map(Id::new(79));
        assert_eq!(expected, actual);

        let expected = Id::new(14);
        let actual = map.map(Id::new(14));
        assert_eq!(expected, actual);

        let expected = Id::new(57);
        let actual = map.map(Id::new(55));
        assert_eq!(expected, actual);

        let expected = Id::new(13);
        let actual = map.map(Id::new(13));
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 35;
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
        let expected = 46;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
