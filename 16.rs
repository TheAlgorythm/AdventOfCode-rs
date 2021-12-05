use std::collections::HashMap;

type Rules = HashMap<String, Vec<(i32, i32)>>;
type Tickets = Vec<Vec<i32>>;
type TicketsSlice = [Vec<i32>];

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

fn value_in_ranges(value: &i32, ranges: &[&(i32, i32)]) -> bool {
    ranges.iter().any(|(min, max)| (min..=max).contains(&value))
}

fn scanning_error_rate(rules: &Rules, tickets: &TicketsSlice) -> i32 {
    let ranges: Vec<&(i32, i32)> = rules.values().flatten().collect();
    tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|value| !value_in_ranges(value, &ranges))
                .sum::<i32>()
        })
        .sum()
}

fn solve_part_one(rules: &Rules, tickets: &TicketsSlice) {
    let sum_invalid = scanning_error_rate(rules, tickets);
    println!("The scanning error rate is {}.", sum_invalid);
}

fn get_labels(rules: &Rules, tickets: &TicketsSlice) -> Vec<String> {
    let ranges: Vec<&(i32, i32)> = rules.values().flatten().collect();
    let valid_tickets: Vec<&Vec<i32>> = tickets
        .iter()
        .skip(1)
        .filter(|ticket| ticket.iter().all(|value| value_in_ranges(value, &ranges)))
        .collect();

    let length_attributes = valid_tickets[0].len();
    let mut possible_attribute_names = vec![Vec::new(); rules.len()];

    rules.iter().for_each(|(attribute_name, attribute_ranges)| {
        (0_usize..length_attributes)
            .filter(|&attribute_index| {
                valid_tickets.iter().all(|ticket| {
                    value_in_ranges(
                        &(**ticket)[attribute_index],
                        &attribute_ranges.iter().collect::<Vec<_>>(),
                    )
                })
            })
            .for_each(|attribute_index| {
                possible_attribute_names[attribute_index].push(attribute_name)
            });
    });

    // determine
    let mut attribute_names = vec![String::new(); rules.len()];
    while attribute_names
        .iter()
        .any(|attribute_name| attribute_name.is_empty())
    {
        possible_attribute_names
            .iter_mut()
            .enumerate()
            .filter(|(_attribute_index, possible_names)| possible_names.len() == 1)
            .for_each(|(attribute_index, possible_names)| {
                attribute_names[attribute_index] = (*possible_names).pop().unwrap().to_string();
            });
        possible_attribute_names
            .iter_mut()
            .for_each(|possible_names| {
                let removing_indexes: Vec<usize> = possible_names
                    .iter()
                    .enumerate()
                    .filter(|(_possible_index, possible_name)| {
                        attribute_names.contains(possible_name)
                    })
                    .map(|(possible_index, _possible_name)| possible_index)
                    .collect();
                removing_indexes.iter().for_each(|possible_index| {
                    possible_names.remove(*possible_index);
                });
            });
    }
    attribute_names
}

fn solve_part_two(rules: &Rules, tickets: &TicketsSlice) {
    let labels = get_labels(rules, tickets);
    let depature_product: i64 = labels
        .iter()
        .enumerate()
        .filter(|(_attribute_index, attribute_name)| attribute_name.starts_with("departure"))
        .map(|(attribute_index, _attribute_name)| tickets[0][attribute_index] as i64)
        .product();
    println!("The departure product is {}.", depature_product);
}

fn main() {
    let input = include_str!("16_data.txt");

    let (rules, tickets) = parse_ticket_notes(input);

    solve_part_one(&rules, &tickets);
    solve_part_two(&rules, &tickets);
}
