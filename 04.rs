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

fn check_valid_values(passport: &HashMap<&str, &str>) -> bool {
    (1920..=2002).contains(
        &passport
            .get("byr")
            .expect("No byr-field!")
            .parse::<i32>()
            .unwrap_or(0),
    ) && (2010..=2020).contains(
        &passport
            .get("iyr")
            .expect("No iyr-field!")
            .parse::<i32>()
            .unwrap_or(0),
    ) && (2020..=2030).contains(
        &passport
            .get("eyr")
            .expect("No eyr-field!")
            .parse::<i32>()
            .unwrap_or(0),
    ) && {
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
    } && {
        let hair_color_item = passport.get("hcl").expect("No hcl-field!");
        hair_color_item.len() == 7
            && hair_color_item.starts_with('#')
            && hair_color_item
                .chars()
                .skip(1)
                .all(|digit| char::is_ascii_hexdigit(&digit))
    } && match *passport.get("ecl").expect("No ecl-field!") {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    } && {
        let pass_id_item = passport.get("pid").expect("No pid-field!");
        pass_id_item.len() == 9
            && pass_id_item
                .chars()
                .all(|digit| char::is_ascii_digit(&digit))
    }
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
