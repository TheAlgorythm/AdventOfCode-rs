use std::collections::HashMap;

fn parse_num_list(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .expect("Couldn't parse input!")
}

fn get_nth_number(starting_values: &Vec<u32>, nth: usize) -> u32 {
    let mut last_values: HashMap<u32, u32> = starting_values
        .iter()
        .take(starting_values.len() - 1)
        .enumerate()
        .map(|(index, value)| (*value, index as u32 + 1))
        .collect();

    let mut last_value = *starting_values.last().expect("No starting value!");

    (starting_values.len()..nth).for_each(|index| {
        let current_value = match last_values.get(&last_value) {
            Some(last_occuring_index) => index as u32 - last_occuring_index,
            None => 0,
        };
        last_values
            .entry(last_value)
            .and_modify(|num| *num = index as u32)
            .or_insert(index as u32);
        last_value = current_value;
    });
    last_value
}

fn solve_part_one(starting_values: &Vec<u32>) {
    let last_spoken_number = get_nth_number(&starting_values, 2020);
    println!("{} will be the 2020th number spoken.", last_spoken_number);
}

fn solve_part_two(starting_values: &Vec<u32>) {
    let last_spoken_number = get_nth_number(&starting_values, 30000000);
    println!(
        "{} will be the 30000000th number spoken.",
        last_spoken_number
    );
}

fn main() {
    let input = "18,8,0,5,4,1,20";

    let starting_values = parse_num_list(&input);

    solve_part_one(&starting_values);
    solve_part_two(&starting_values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(get_nth_number(&parse_num_list(&"0,3,6"), 2020), 436);
        assert_eq!(get_nth_number(&parse_num_list(&"1,3,2"), 2020), 1);
        assert_eq!(get_nth_number(&parse_num_list(&"2,1,3"), 2020), 10);
        assert_eq!(get_nth_number(&parse_num_list(&"1,2,3"), 2020), 27);
        assert_eq!(get_nth_number(&parse_num_list(&"2,3,1"), 2020), 78);
        assert_eq!(get_nth_number(&parse_num_list(&"3,2,1"), 2020), 438);
        assert_eq!(get_nth_number(&parse_num_list(&"3,1,2"), 2020), 1836);
    }
}
