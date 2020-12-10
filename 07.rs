use std::collections::BTreeMap;
mod utils;
use utils::unique::*;

type Tree = BTreeMap<String, Vec<(u32, String)>>;
type ReverseTree = BTreeMap<String, Vec<String>>;

fn parse_reverse_rules(input: &str) -> ReverseTree {
    input.lines().fold(BTreeMap::new(), |mut rules, rule| {
        let mut rule_parts = rule.splitn(2, " contain ");
        let parent = rule_parts
            .next()
            .expect("No parent!")
            .to_string()
            .replace(" bags", "");
        let children = rule_parts.next().expect("No children!");
        children
            .split(", ")
            .filter(|child| child != &"no other bags.")
            .for_each(|child| {
                rules
                    .entry(
                        child
                            .chars()
                            .filter(|character| {
                                character.is_ascii_alphabetic() || character.is_ascii_whitespace()
                            })
                            .skip(1)
                            .collect::<String>()
                            .replace(" bags", "")
                            .replace(" bag", ""),
                    )
                    .and_modify(|parents| parents.push(parent.clone()))
                    .or_insert(vec![parent.clone()]);
            });
        rules
    })
}

fn parse_rules(input: &str) -> Tree {
    input.lines().fold(BTreeMap::new(), |mut rules, rule| {
        let mut rule_parts = rule.splitn(2, " contain ");
        let parent = rule_parts
            .next()
            .expect("No parent!")
            .to_string()
            .replace(" bags", "");
        let children = rule_parts.next().expect("No children!");
        children
            .split(", ")
            .filter(|child| child != &"no other bags.")
            .for_each(|child| {
                let child_name = child
                    .chars()
                    .filter(|character| {
                        character.is_ascii_alphabetic() || character.is_ascii_whitespace()
                    })
                    .skip(1)
                    .collect::<String>()
                    .replace(" bags", "")
                    .replace(" bag", "");
                let child_quantity = child
                    .chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("Couldn't parse number!");
                rules
                    .entry(parent.clone())
                    .and_modify(|children| children.push((child_quantity, child_name.clone())))
                    .or_insert(vec![(child_quantity, child_name.clone())]);
            });
        rules
    })
}

fn count_distinct_outer_layers(
    rules: &ReverseTree,
    pattern: &String,
    mut outers: &mut Vec<String>,
) {
    rules.get(pattern).map(|parents| {
        parents.iter().for_each(|parent| {
            outers.push(parent.to_string());
            count_distinct_outer_layers(&rules, &parent, &mut outers)
        });
    });
}

fn solve_part_one(input: &str) {
    let rules = parse_reverse_rules(&input);

    let mut shiny_gold_possibilities = Vec::new();
    count_distinct_outer_layers(
        &rules,
        &"shiny gold".to_string(),
        &mut shiny_gold_possibilities,
    );
    println!(
        "The number of bag colors that can eventually contain at least one shiny gold bag is {}.",
        shiny_gold_possibilities.into_iter().unique().count()
    );
}

fn count_inner_bags(rules: &Tree, pattern: &String) -> u32 {
    match rules.get(pattern) {
        None => 0,
        Some(rule) => rule
            .iter()
            .map(|(quantity, name)| quantity + (quantity * count_inner_bags(&rules, name)))
            .sum(),
    }
}

fn solve_part_two(input: &str) {
    let rules = parse_rules(&input);
    let inner_bags = count_inner_bags(&rules, &"shiny gold".to_string());
    println!("The shiny gold bag has to contain {} bags.", inner_bags);
}

fn main() {
    let input = include_str!("07_data.rules");

    solve_part_one(&input);
    solve_part_two(&input);
}
