use std::{
    collections::{HashMap, VecDeque},
    ops::{Index, Not},
};

use aoc_lib::{misc::IdGen, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use smallvec::SmallVec;

pub const DAY: Day = Day {
    day: 20,
    name: "Pulse Propagation",
    part_1: run_part1,
    part_2: None,
    // part_2: Some(run_part2),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ModuleId(u8);

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
    kind: ModuleType,
    inputs: SmallVec<[ModuleId; 8]>,
    outputs: SmallVec<[ModuleId; 8]>,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            kind: ModuleType::Broadcast,
            inputs: Default::default(),
            outputs: Default::default(),
        }
    }
}

#[derive(Debug)]
struct ModuleSystem {
    modules: Vec<Module>,
    start: ModuleId,
}

impl Index<ModuleId> for ModuleSystem {
    type Output = Module;

    fn index(&self, index: ModuleId) -> &Self::Output {
        &self.modules[index.0 as usize]
    }
}

fn parse(input: &str) -> Result<ModuleSystem> {
    let mut modules = IdGen::<Module, _, _, _>::new(|id| ModuleId(id as u8), |id| id.0 as usize);

    for line in input.trim().lines().map(str::trim) {
        let (kind_name, outputs) = line.split_once(" -> ").unwrap();
        let c = kind_name.as_bytes()[0];
        let (kind, name) = match c {
            b'&' => (ModuleType::Conjunction, &kind_name[1..]),
            b'%' => (ModuleType::FlipFlop, &kind_name[1..]),
            _ => (ModuleType::Broadcast, kind_name),
        };

        let id = modules.id_of(name);
        modules[id].kind = kind;
        for output in outputs.split(',').map(str::trim) {
            let output_id = modules.id_of(output);
            modules[output_id].inputs.push(id);
            modules[id].outputs.push(output_id);
        }
    }

    let start = modules.id_of("broadcaster");
    Ok(ModuleSystem {
        modules: modules.into_items(),
        start,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

#[derive(Debug, Default)]
struct PulseQueue {
    queue: VecDeque<(ModuleId, Pulse, ModuleId)>,
    num_high: u32,
    num_low: u32,
}

impl PulseQueue {
    fn next(&mut self) -> Option<(ModuleId, Pulse, ModuleId)> {
        self.queue.pop_front()
    }

    fn add(&mut self, src: ModuleId, pulse: Pulse, dst: ModuleId) {
        match pulse {
            Pulse::High => self.num_high += 1,
            Pulse::Low => self.num_low += 1,
        }

        self.queue.push_back((src, pulse, dst))
    }
}

fn next_pulse(
    module: &Module,
    pulse: Pulse,
    flip_flop_state: &mut [Pulse],
    dst: ModuleId,
    inputs: &mut [HashMap<ModuleId, Pulse>],
    src: ModuleId,
) -> Option<Pulse> {
    let new_pulse = match (module.kind, pulse) {
        (ModuleType::FlipFlop, Pulse::High) => return None,
        (ModuleType::FlipFlop, Pulse::Low) => {
            let state = &mut flip_flop_state[dst.0 as usize];
            *state = !*state;
            *state
        }
        (ModuleType::Conjunction, _) => {
            let inputs = &mut inputs[dst.0 as usize];
            inputs.insert(src, pulse);

            if inputs.values().all(|&v| v == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            }
        }
        (ModuleType::Broadcast, _) => pulse,
    };
    Some(new_pulse)
}

fn part1(data: &ModuleSystem) -> u32 {
    let mut pulse_queue = PulseQueue::default();
    let mut flip_flop_state = vec![Pulse::Low; data.modules.len()];
    let mut inputs: Vec<HashMap<_, _>> = data
        .modules
        .iter()
        .map(|m| m.inputs.iter().map(|&m| (m, Pulse::Low)).collect())
        .collect();

    for _ in 0..1000 {
        pulse_queue.add(data.start, Pulse::Low, data.start);
        while let Some((src, pulse, dst)) = pulse_queue.next() {
            let module = &data[dst];
            let Some(new_pulse) =
                next_pulse(module, pulse, &mut flip_flop_state, dst, &mut inputs, src)
            else {
                continue;
            };

            for &output in &module.outputs {
                pulse_queue.add(dst, new_pulse, output);
            }
        }
    }

    pulse_queue.num_high * pulse_queue.num_low
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        for id in [1, 2] {
            let data = aoc_lib::input(DAY.day)
                .example(Example::Part1, id)
                .open()
                .unwrap();

            let (example, expected) = data.split_once("---").unwrap();

            let parsed = parse(example.trim()).unwrap();
            let expected: u32 = expected.trim().parse().unwrap();
            let actual = part1(&parsed);

            assert_eq!(expected, actual);
        }
    }
}
