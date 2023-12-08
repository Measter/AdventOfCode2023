use std::collections::HashMap;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use num::Integer;

pub const DAY: Day = Day {
    day: 8,
    name: "Haunted Wasteland",
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

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeId(u16);

#[derive(Debug, Clone, Copy)]
struct Node {
    id: NodeId,
    end_a: bool,
    end_z: bool,
    left: NodeId,
    right: NodeId,
}

#[derive(Debug, Clone)]
struct Map {
    steps: Vec<Step>,
    start: NodeId,
    end: NodeId,
    nodes: Vec<Node>,
}

impl Map {
    fn get_node_id<'a>(&mut self, name: &'a str, id_map: &mut HashMap<&'a str, NodeId>) -> NodeId {
        if let Some(&id) = id_map.get(name) {
            return id;
        }

        let id = NodeId(self.nodes.len() as u16);
        id_map.insert(name, id);
        self.nodes.push(Node {
            id,
            end_a: name.ends_with('A'),
            end_z: name.ends_with('Z'),
            left: id,
            right: id,
        });

        id
    }

    fn get_node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0 as usize]
    }

    fn get_node(&self, id: NodeId) -> &Node {
        &self.nodes[id.0 as usize]
    }
}

fn parse(input: &str) -> Result<Map> {
    let (steps, graph) = input.split_once('\n').unwrap();

    let steps: Vec<_> = steps
        .trim()
        .bytes()
        .map(|b| if b == b'L' { Step::Left } else { Step::Right })
        .collect();

    let mut map = Map {
        steps,
        start: NodeId(0),
        end: NodeId(0),
        nodes: Vec::new(),
    };

    let mut interner = HashMap::new();

    for line in graph.trim().lines() {
        let (name, next) = line.split_once(" = ").unwrap();

        let name_node = map.get_node_id(name, &mut interner);
        if name == "AAA" {
            map.start = name_node;
        } else if name == "ZZZ" {
            map.end = name_node;
        }

        let (left, right) = next
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        let left_node = map.get_node_id(left.trim(), &mut interner);
        let right_node = map.get_node_id(right.trim(), &mut interner);

        let node = map.get_node_mut(name_node);
        node.left = left_node;
        node.right = right_node;
    }

    Ok(map)
}

fn part1(map: &Map) -> u32 {
    let mut cur_id = map.start;
    let mut step_count = 0;

    let mut steps = map.steps.iter().cycle();

    while cur_id != map.end {
        step_count += 1;
        let node = map.get_node(cur_id);
        cur_id = match steps.next().unwrap() {
            Step::Left => node.left,
            Step::Right => node.right,
        };
    }

    step_count
}

fn part2(map: &Map) -> u64 {
    let start_nodes: Vec<_> = map.nodes.iter().filter(|n| n.end_a).map(|n| n.id).collect();

    let mut cycle_lengths = vec![0u64; start_nodes.len()];

    for (cycle_len, node) in cycle_lengths.iter_mut().zip(&start_nodes) {
        let mut cur_id = *node;

        let mut steps = map.steps.iter().cycle();
        let mut step_count = 0;

        while !map.get_node(cur_id).end_z {
            step_count += 1;
            let node = map.get_node(cur_id);
            cur_id = match steps.next().unwrap() {
                Step::Left => node.left,
                Step::Right => node.right,
            };
        }

        *cycle_len = step_count;
    }

    let [first, rest @ ..] = cycle_lengths.as_slice() else {
        unreachable!()
    };

    rest.iter().fold(*first, |acc, r| acc.lcm(r))
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let tests = [(1, 2), (2, 6)];

        for (id, expected) in tests {
            let data = aoc_lib::input(DAY.day)
                .example(Example::Part1, id)
                .open()
                .unwrap();

            let parsed = parse(&data).unwrap();
            let actual = part1(&parsed);

            assert_eq!(expected, actual, "{id}");
        }
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 6;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
