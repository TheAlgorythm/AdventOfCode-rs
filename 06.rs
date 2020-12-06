use std::collections::BTreeSet;
use std::hash::Hash;

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
    I::Item: Hash + Eq + std::cmp::Ord + Clone,
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
        Self::Item: Hash + Eq + std::cmp::Ord + Clone,
        Self: Sized,
    {
        Unique {
            seen: BTreeSet::new(),
            underlying: self,
        }
    }
}

impl<I: Iterator> UniqueExt for I {}

fn parse_answers(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn solve_part_one(answers: &Vec<&str>) {
    let answer_count_sum: usize = answers
        .iter()
        .map(|group| {
            group
                .chars()
                .filter(char::is_ascii_alphabetic)
                .unique()
                .count()
        })
        .sum();
    println!("The total answers count summed up is {}.", answer_count_sum);
}

fn count_everyone_answer(group: &str) -> usize {
    let people = group.lines().count();
    group
        .chars()
        .filter(char::is_ascii_alphabetic)
        .unique()
        .filter(|answer_type| {
            group
                .chars()
                .filter(|answers| answers == answer_type)
                .count()
                == people
        })
        .count()
}

fn solve_part_two(answers: &Vec<&str>) {
    let answer_count_sum: usize = answers
        .iter()
        .map(|group| count_everyone_answer(&group))
        .sum();
    println!("The total answers count summed up is {}.", answer_count_sum);
}

fn main() {
    let input = include_str!("06_data.txt");

    let answers = parse_answers(&input);

    solve_part_one(&answers);
    solve_part_two(&answers);
}
