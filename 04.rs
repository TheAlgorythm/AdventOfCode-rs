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

fn check_range(passport: &HashMap<&str, &str>, field: &str, from: i32, to: i32) -> bool {
    (from..=to).contains(
        &passport
            .get(field)
            .expect(format!("No {}-field!", field).as_str())
            .parse::<i32>()
            .unwrap_or(0),
    )
}

fn check_height(passport: &HashMap<&str, &str>) -> bool {
    let height_item = passport.get("hgt").expect("No hgt-field!");
    match &height_item[height_item.len() - 2..] {
        "cm" => (150..=193).contains(
            &height_item[..height_item.len() - 2]
                .parse::<i32>()
                .unwrap_or(0),
        ),
        "in" => (59..=76).contains(
            &height_item[..height_item.len() - 2]
                .parse::<i32>()
                .unwrap_or(0),
        ),
        _ => false,
    }
}

fn check_hair_color(passport: &HashMap<&str, &str>) -> bool {
    let hair_color_item = passport.get("hcl").expect("No hcl-field!");
    hair_color_item.len() == 7
        && hair_color_item.starts_with('#')
        && hair_color_item
            .chars()
            .skip(1)
            .all(|digit| char::is_ascii_hexdigit(&digit))
}

fn check_eye_color(passport: &HashMap<&str, &str>) -> bool {
    match *passport.get("ecl").expect("No ecl-field!") {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn check_pass_id(passport: &HashMap<&str, &str>) -> bool {
    let pass_id_item = passport.get("pid").expect("No pid-field!");
    pass_id_item.len() == 9
        && pass_id_item
            .chars()
            .all(|digit| char::is_ascii_digit(&digit))
}

fn check_valid_values(passport: &HashMap<&str, &str>) -> bool {
    check_range(&passport, "byr", 1920, 2002)
        && check_range(&passport, "iyr", 2010, 2020)
        && check_range(&passport, "eyr", 2020, 2030)
        && check_height(&passport)
        && check_hair_color(&passport)
        && check_eye_color(&passport)
        && check_pass_id(&passport)
}

fn solve_part_one(passports: &Vec<HashMap<&str, &str>>) {
    let valid_count = passports
        .iter()
        .filter(|passport| check_valid_keys(&passport))
        .count();
    println!("There are {} key-valid passports.", valid_count);
}

fn solve_part_two(passports: &Vec<HashMap<&str, &str>>) {
    let valid_count = passports
        .iter()
        .filter(|passport| check_valid_keys(&passport))
        .filter(|passport| check_valid_values(&passport))
        .count();
    println!("There are {} valid passports.", valid_count);
}

fn main() {
    let input = include_str!("04_data.batch");

    let passports = parse_pass_batch(&input);

    solve_part_one(&passports);
    solve_part_two(&passports);
}
