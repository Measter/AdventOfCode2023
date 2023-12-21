use std::ops::{Index, IndexMut};

use aoc_lib::{
    misc::{IdGen, IdType},
    Bench, BenchResult, Day, NoError, ParseResult, UserError,
};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 19,
    name: "Aplenty",
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

#[derive(Debug, Clone, Copy)]
enum PartField {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
struct Part([u32; 4]);

impl Index<PartField> for Part {
    type Output = u32;
    fn index(&self, index: PartField) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl IndexMut<PartField> for Part {
    fn index_mut(&mut self, index: PartField) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Debug, Clone, Copy)]
enum RuleCondition {
    None,
    Less(PartField, u16),
    Greater(PartField, u16),
}

#[derive(Debug, Clone, Copy)]
struct WorkFlowId(usize);
impl IdType for WorkFlowId {
    fn from_usize(i: usize) -> Self {
        Self(i)
    }

    fn to_usize(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
enum RuleOutput {
    Accept,
    Reject,
    Workflow(WorkFlowId),
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    cond: RuleCondition,
    output: RuleOutput,
}

#[derive(Debug, Default)]
struct WorkFlow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct WorkFlows {
    flows: Vec<WorkFlow>,
}

impl Index<WorkFlowId> for WorkFlows {
    type Output = WorkFlow;

    fn index(&self, index: WorkFlowId) -> &Self::Output {
        &self.flows[index.0]
    }
}

fn parse(input: &str) -> Result<(WorkFlows, Vec<Part>, WorkFlowId)> {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();

    let mut idgen = IdGen::<WorkFlow, _>::new();
    for wf in workflows_str.lines().map(str::trim) {
        let (name, rules) = wf.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap();

        let id = idgen.id_of(name);
        for rule in rules.split(',').map(str::trim) {
            let rule = match rule.split_once(':') {
                Some((condition, output)) => {
                    let output = match output {
                        "R" => RuleOutput::Reject,
                        "A" => RuleOutput::Accept,
                        _ => RuleOutput::Workflow(idgen.id_of(output)),
                    };

                    let int = condition[2..].parse().unwrap();
                    let part_field = match condition.as_bytes()[0] {
                        b'x' => PartField::X,
                        b'm' => PartField::M,
                        b'a' => PartField::A,
                        b's' => PartField::S,
                        _ => unreachable!(),
                    };
                    let cond = match condition.as_bytes()[1] {
                        b'>' => RuleCondition::Greater(part_field, int),
                        b'<' => RuleCondition::Less(part_field, int),
                        _ => unreachable!(),
                    };
                    Rule { cond, output }
                }
                None => {
                    let output = match rule {
                        "R" => RuleOutput::Reject,
                        "A" => RuleOutput::Accept,
                        _ => RuleOutput::Workflow(idgen.id_of(rule)),
                    };
                    Rule {
                        cond: RuleCondition::None,
                        output,
                    }
                }
            };

            idgen[id].rules.push(rule);
        }
    }

    let mut parts = Vec::new();
    for pt in parts_str.lines().map(str::trim) {
        let pt = &pt[1..pt.len() - 1];
        let mut part = Part([0; 4]);

        for field in pt.split(',') {
            let value = field[2..].parse().unwrap();
            match field.as_bytes()[0] {
                b'x' => part.0[0] = value,
                b'm' => part.0[1] = value,
                b'a' => part.0[2] = value,
                b's' => part.0[3] = value,
                _ => unreachable!(),
            }
        }

        parts.push(part);
    }

    let in_id = idgen.id_of("in");

    Ok((
        WorkFlows {
            flows: idgen.into_items(),
        },
        parts,
        in_id,
    ))
}

fn part1((workflows, parts, start): &(WorkFlows, Vec<Part>, WorkFlowId)) -> u32 {
    let mut sum = 0;
    for part in parts {
        let mut cur_wf_id = *start;

        'cond_check: loop {
            let cur_wf = &workflows[cur_wf_id];
            for rule in &cur_wf.rules {
                match rule.cond {
                    RuleCondition::Less(f, i) if part[f] >= i as u32 => {
                        continue;
                    }
                    RuleCondition::Greater(f, i) if part[f] <= i as u32 => continue,
                    _ => {}
                }

                match rule.output {
                    RuleOutput::Accept => {
                        sum += part.0.into_iter().sum::<u32>();
                        break 'cond_check;
                    }
                    RuleOutput::Reject => break 'cond_check,
                    RuleOutput::Workflow(next) => {
                        cur_wf_id = next;
                        continue 'cond_check;
                    }
                }
            }
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
        let expected = 19114;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
