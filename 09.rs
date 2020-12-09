fn parse_encrypted(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .expect("Parsing failed!")
}

fn get_first_invalid(encrypted: &Vec<u64>) -> Option<(usize, u64)> {
    let preamble_size = 25;
    for (step, num) in encrypted.iter().enumerate().skip(preamble_size) {
        let leading_numbers = &encrypted[step - preamble_size..step];
        if !leading_numbers
            .iter()
            .filter(|previous_number| *previous_number <= num)
            .any(|previous_number| leading_numbers.contains(&(num - previous_number)))
        {
            return Some((step, *num));
        }
    }
    None
}

fn solve_part_one(first_invalid: &Option<(usize, u64)>) {
    match first_invalid {
        Some((position, invalid)) => println!(
            "The scheme is not fulfilled on position {} with value {}.",
            position, invalid
        ),
        None => println!("There is no invalid position."),
    }
}

fn find_weakness(encrypted: &Vec<u64>, invalid: u64) -> Option<u64> {
    let mut sequence: std::collections::VecDeque<u64> = std::collections::VecDeque::new();
    for num in encrypted.iter() {
        sequence.push_back(*num);
        let mut sequence_sum = sequence.iter().sum();
        while invalid < sequence_sum {
            sequence.pop_front();
            sequence_sum = sequence.iter().sum();
        }
        if invalid == sequence_sum {
            return Some(
                sequence.iter().min().expect("Empty sequence!")
                    + sequence.iter().max().expect("Empty sequence!"),
            );
        }
    }
    None
}

fn solve_part_two(encrypted: &Vec<u64>, first_invalid: &Option<(usize, u64)>) {
    let first_invalid = match first_invalid {
        Some((_position, invalid_number)) => *invalid_number,
        None => {
            println!("There is no invalid position.");
            return;
        }
    };
    match find_weakness(&encrypted, first_invalid) {
        Some(weakness) => println!("The weakness is {}.", weakness),
        None => println!("There is no weakness."),
    }
}

fn main() {
    let input = include_str!("09_data.enc");

    let encrypted = parse_encrypted(&input);

    let first_invalid = get_first_invalid(&encrypted);

    solve_part_one(&first_invalid);
    solve_part_two(&encrypted, &first_invalid);
}
