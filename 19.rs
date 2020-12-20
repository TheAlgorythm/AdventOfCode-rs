use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::str::FromStr;
mod utils;
use utils::unique::*;

#[derive(Debug)]
enum Rule {
    /// The outer is a or group, the inner a specific order of rules.
    Meta(Vec<Vec<u32>>),
    Data(char),
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("\"") {
            return Ok(Rule::Data(
                s.trim()
                    .replace("\"", "")
                    .chars()
                    .next()
                    .expect("There is no Data!"),
            ));
        } else {
            return Ok(Rule::Meta(
                s.split("|")
                    .map(|or_group| {
                        or_group
                            .split(" ")
                            .filter(|id| !id.is_empty())
                            .map(|id| id.parse::<u32>())
                            .collect::<Result<_, ParseIntError>>()
                    })
                    .collect::<Result<_, ParseIntError>>()?,
            ));
        }
    }
}

#[derive(Debug)]
struct Rules {
    rules: BTreeMap<u32, Rule>,
}

impl Rules {
    pub fn check(&self, message: &str) -> bool {
        let mut paths = vec![0];

        self.check_internal(message, 0, &mut paths);

        paths.contains(&(message.len() as u32))
    }

    fn check_internal(&self, message: &str, next_rule: u32, paths: &mut Vec<u32>) {
        if paths.is_empty() {
            return;
        }
        match self.rules.get(&next_rule).expect("Rule not found!") {
            Rule::Data(data) => {
                *paths = paths
                    .iter()
                    .filter(|index| match message.chars().nth(**index as usize) {
                        Some(next_character) if *data == next_character => true,
                        _ => false,
                    })
                    .map(|path| *path + 1)
                    .collect()
            }
            Rule::Meta(sub_rules) => {
                *paths = sub_rules
                    .iter()
                    .map(|rule_chain| {
                        let mut next_paths = paths.to_vec();
                        rule_chain
                            .iter()
                            .for_each(|rule| self.check_internal(message, *rule, &mut next_paths));

                        next_paths
                    })
                    .flatten()
                    .unique()
                    .collect();
            }
        }
    }

    fn patch_rules_part_2(&mut self) {
        self.rules
            .entry(8)
            .and_modify(|rule_8| *rule_8 = Rule::Meta(vec![vec![42], vec![42, 8]]));
        self.rules
            .entry(11)
            .and_modify(|rule_11| *rule_11 = Rule::Meta(vec![vec![42, 31], vec![42, 11, 31]]));
    }
}

impl FromStr for Rules {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rules {
            rules: s
                .lines()
                .map(|line| {
                    let elements: Vec<&str> = line.splitn(2, ": ").collect();
                    Ok((elements[0].parse::<u32>()?, elements[1].parse::<Rule>()?))
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?,
        })
    }
}

fn parse_reg_tex(input: &str) -> (Rules, Vec<&str>) {
    let elements: Vec<&str> = input.splitn(2, "\n\n").collect();
    (
        elements[0].parse().expect("Couldn't parse rules!"),
        elements[1].lines().collect(),
    )
}

fn solve_part_one(rules: &Rules, messages: &Vec<&str>) {
    let valid_messages = messages
        .iter()
        .filter(|message| rules.check(message))
        .count();
    println!("{} messages completely match rule 0.", valid_messages);
}

fn solve_part_two(rules: &mut Rules, messages: &Vec<&str>) {
    rules.patch_rules_part_2();
    let valid_messages = messages
        .iter()
        .filter(|message| rules.check(message))
        .count();
    println!(
        "With patched rules, {} messages completely match rule 0.",
        valid_messages
    );
}

fn main() {
    let input = include_str!("19_data.txt");

    let (mut rules, messages) = parse_reg_tex(&input);

    solve_part_one(&rules, &messages);
    solve_part_two(&mut rules, &messages);
}
