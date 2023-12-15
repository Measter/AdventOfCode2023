use aoc_lib::{Bench, BenchResult, Day, NoError};

pub const DAY: Day = Day {
    day: 15,
    name: "Lens Library",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(part1(input.trim())))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(part2(input.trim())))
}

fn hash(s: &[u8]) -> u32 {
    s.iter()
        .fold(0u32, |acc, &b| acc.wrapping_add(b as u32).wrapping_mul(17))
        & 0xff
}

fn part1(input: &str) -> u32 {
    input.as_bytes().split(|&b| b == b',').map(hash).sum()
}

#[derive(Debug)]
struct BoxContent<'a> {
    key: &'a [u8],
    value: u8,
}

#[derive(Debug)]
struct Map<'a> {
    boxes: Vec<Vec<BoxContent<'a>>>,
}

impl<'a> Map<'a> {
    fn new() -> Self {
        Self {
            boxes: {
                let mut v = Vec::new();
                v.resize_with(256, Vec::new);
                v
            },
        }
    }

    fn insert(&mut self, key: &'a [u8], value: u8) {
        let box_idx = hash(key) as usize;
        let box_ = &mut self.boxes[box_idx];

        if let Some(slot_idx) = box_.iter().position(|b| b.key == key) {
            // This key already has a value.
            box_[slot_idx].value = value;
        } else {
            box_.push(BoxContent { key, value });
        }
    }

    fn remove(&mut self, key: &'a [u8]) {
        let box_idx = hash(key) as usize;
        let box_ = &mut self.boxes[box_idx];

        let Some(slot_idx) = box_.iter().position(|b| b.key == key) else {
            return;
        };

        box_.remove(slot_idx);
    }
}

fn part2(input: &str) -> u32 {
    let mut map = Map::new();
    input.as_bytes().split(|&b| b == b',').for_each(|v| {
        let op_pos = v.iter().position(|&b| b == b'-' || b == b'=').unwrap();
        let (key, op_val) = v.split_at(op_pos);

        match op_val {
            [b'-'] => map.remove(key),
            [b'=', lens @ ..] => {
                let value = lens.iter().fold(0, |acc, b| (acc * 10) + (b & 0xf));
                map.insert(key, value);
            }
            _ => unreachable!(),
        }
    });

    let mut sum = 0;
    for (box_, box_id) in map.boxes.into_iter().zip(1..) {
        if box_.is_empty() {
            continue;
        }

        for (slot, slot_id) in box_.into_iter().zip(1..) {
            sum += box_id * slot_id * slot.value as u32;
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

        let expected = 1320;
        let actual = part1(data.trim());

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = 145;
        let actual = part2(data.trim());

        assert_eq!(expected, actual);
    }
}
