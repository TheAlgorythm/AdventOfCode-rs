use std::collections::HashMap;

fn parse_pass_batch(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|pass| {
            pass.split(char::is_whitespace)
                .fold(HashMap::new(), |mut map, entry| {
                    let mut key_value = entry.splitn(2, ":");
                    map.insert(
                        key_value.next().expect("No key!"),
                        key_value.next().expect("No value!"),
                    );
                    map
                })
        })
        .collect()
}

fn check_valid_keys(passport: &HashMap<&str, &str>) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields
        .into_iter()
        .all(|key| passport.contains_key(key))
}

fn solve_part_one(passports: &Vec<HashMap<&str, &str>>) {
    let valid_count = passports
        .iter()
        .filter(|passport| check_valid_keys(&passport))
        .count();
    println!("There are {} valid passports.", valid_count);
}

fn solve_part_two() {}

fn main() {
    let input = include_str!("04_data.batch");

    let passports = parse_pass_batch(&input);

    solve_part_one(&passports);
    solve_part_two();
}
