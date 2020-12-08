use std::collections::{BTreeMap, BTreeSet};

type ReverseTree = BTreeMap<String, Vec<String>>;

struct Unique<I>
where
    I: Iterator,
{
    seen: BTreeSet<I::Item>,
    underlying: I,
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.underlying.next() {
            if !self.seen.contains(&x) {
                self.seen.insert(x.clone());
                return Some(x);
            }
        }
        None
    }
}

trait UniqueExt: Iterator {
    fn unique(self) -> Unique<Self>
    where
        Self::Item: Ord + Clone,
        Self: Sized,
    {
        Unique {
            seen: BTreeSet::new(),
            underlying: self,
        }
    }
}

impl<I: Iterator> UniqueExt for I {}

fn parse_rules(input: &str) -> ReverseTree {
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

fn solve_part_one(rules: &ReverseTree) {
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

fn solve_part_two(rules: &ReverseTree) {
    // println!("The total answers count summed up is {}.", answer_count_sum);
}

fn main() {
    let input = include_str!("07_data.rules");

    let rules = parse_rules(&input);

    solve_part_one(&rules);
    solve_part_two(&rules);
}
