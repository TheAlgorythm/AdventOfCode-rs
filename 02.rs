use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParsePolicyError {
    ParseInt(ParseIntError),
    NoSearchCharacter,
}

impl From<ParseIntError> for ParsePolicyError {
    fn from(err: ParseIntError) -> ParsePolicyError {
        ParsePolicyError::ParseInt(err)
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    first_constraint: usize,
    second_constraint: usize,
    search_character: char,
    suggestion: String,
}

impl PasswordPolicy {
    fn check_suggestion_part_one(&self) -> bool {
        let search_character_count = self
            .suggestion
            .chars()
            .filter(|character| *character == self.search_character)
            .count();

        (self.first_constraint..=self.second_constraint).contains(&search_character_count)
    }

    fn check_suggestion_part_two(&self) -> bool {
        (self.suggestion.chars().nth(self.first_constraint - 1) == Some(self.search_character))
            != (self.suggestion.chars().nth(self.second_constraint - 1)
                == Some(self.search_character))
    }
}

impl FromStr for PasswordPolicy {
    type Err = ParsePolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements: Vec<&str> = s.split(' ').collect();

        let occurrences: Vec<&str> = elements[0].split('-').collect();

        let first_constraint = occurrences[0].parse::<usize>()?;

        let second_constraint = occurrences[1].parse::<usize>()?;

        let search_character: char = match elements[1].chars().next() {
            Some(character) => character,
            None => return Err(ParsePolicyError::NoSearchCharacter),
        };

        let suggestion = elements[2].to_string();

        Ok(PasswordPolicy {
            first_constraint,
            second_constraint,
            search_character,
            suggestion,
        })
    }
}

#[derive(Debug)]
pub enum MainError {
    IO(std::io::Error),
    ParsePolicy(ParsePolicyError),
}

impl From<ParsePolicyError> for MainError {
    fn from(err: ParsePolicyError) -> MainError {
        MainError::ParsePolicy(err)
    }
}

impl From<std::io::Error> for MainError {
    fn from(err: std::io::Error) -> MainError {
        MainError::IO(err)
    }
}

fn main() -> Result<(), MainError> {
    let file = File::open("02_data.list")?;
    let reader = BufReader::new(file);

    let policies: Vec<PasswordPolicy> = reader
        .lines()
        .map(|line| line.unwrap().parse::<PasswordPolicy>())
        .collect::<Result<Vec<PasswordPolicy>, ParsePolicyError>>()?;

    let part_one_valid_count = policies
        .iter()
        .filter(|policy| policy.check_suggestion_part_one())
        .count();

    let part_two_valid_count = policies
        .into_iter()
        .filter(|policy| policy.check_suggestion_part_two())
        .count();

    println!(
        "There are {} valid suggestions. ({} according to the old scheme)",
        part_two_valid_count, part_one_valid_count,
    );

    Ok(())
}
