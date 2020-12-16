use std::collections::HashMap;

type Rules = HashMap<String, Vec<(i32, i32)>>;
type Tickets = Vec<Vec<i32>>;

fn parse_ticket_notes(input: &str) -> (Rules, Tickets) {
    let elements: Vec<&str> = input.splitn(2, "\n\n").collect();

    let rules = elements[0]
        .lines()
        .map(|line| {
            let rule_elements: Vec<&str> = line.splitn(2, ": ").collect();
            let rule_name = rule_elements[0].to_string();
            let rule_body = rule_elements[1]
                .split(" or ")
                .map(|rule_range| {
                    let rule_range_elements: Vec<&str> = rule_range.splitn(2, '-').collect();
                    (
                        rule_range_elements[0]
                            .parse::<i32>()
                            .expect("Couldn't parse range!"),
                        rule_range_elements[1]
                            .parse::<i32>()
                            .expect("Couldn't parse range!"),
                    )
                })
                .collect();
            (rule_name, rule_body)
        })
        .collect();

    let tickets = elements[1]
        .replace("your ticket:", "")
        .replace("nearby tickets:", "")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|ticket| {
            ticket
                .split(',')
                .map(|value| value.parse().expect("Couldn't parse ticket-value!"))
                .collect()
        })
        .collect();

    (rules, tickets)
}

fn scanning_error_rate(rules: &Rules, tickets: &Tickets) -> i32 {
    let ranges: Vec<&(i32, i32)> = rules.values().flatten().collect();
    tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|value| !ranges.iter().any(|(min, max)| (min..=max).contains(value)))
                .sum::<i32>()
        })
        .sum()
}

fn solve_part_one(rules: &Rules, tickets: &Tickets) {
    let sum_invalid = scanning_error_rate(&rules, &tickets);
    println!("The scanning error rate is {}.", sum_invalid);
}

fn solve_part_two() {}

fn main() {
    let input = include_str!("16_data.txt");

    let (rules, tickets) = parse_ticket_notes(&input);

    solve_part_one(&rules, &tickets);
    solve_part_two();
}
