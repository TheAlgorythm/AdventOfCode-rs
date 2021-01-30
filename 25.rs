use std::collections::HashMap;

fn pow_mod(base: u64, exponent: u64, divider: u64) -> u64 {
    (0..exponent).fold(1_u64, |value, _| (value as u128 * base as u128 % divider as u128) as u64)
}

fn pow_mod_cached(base: u64, exponent: u64, divider: u64, mut exponent_cache: &mut HashMap<u64, u64>) -> u64 {
    if exponent == 0 {
        return 1;
    } 
    let highest_cached_exponent = exponent_cache.keys().filter(|cached_exponent| cached_exponent <= &&exponent).max();
    let result = match highest_cached_exponent {
        None => pow_mod(base, exponent, divider), 
        Some(highest_cached_exponent) => (exponent_cache[highest_cached_exponent] as u128 * pow_mod_cached(base, exponent - highest_cached_exponent, divider, &mut exponent_cache) as u128 % divider as u128) as u64, 
    };

    exponent_cache.insert(exponent, result);
    result
}

fn discrete_logarithm(base: u64, divider: u64, result: u64) -> Option<u64> {
    let base = base % divider;
    let result = result % divider;
    
    let big_step_size = (divider as f64).sqrt().ceil() as u64;
    let mut exponent_cache = HashMap::new(); 

    let big_values: HashMap<u64, u64> = (1..=big_step_size).map(|big_step| (pow_mod_cached(base, big_step * big_step_size, divider, &mut exponent_cache), big_step)).collect();
    
    (1..=big_step_size)
        .map(|baby_step| (baby_step, ((pow_mod_cached(base, baby_step, divider, &mut exponent_cache) as u128 * result as u128) % divider as u128) as u64))
        .filter(|(_baby_step, baby_value)| big_values.contains_key(baby_value))
        .map(|(baby_step, baby_value)| big_values[&baby_value] * big_step_size - baby_step)
        .min()
} 

fn solve_part_one() {
    let device_base = 7;
    let divider = 20201227;

    let card_loop_size = discrete_logarithm(device_base, divider, 13233401).expect("Card invalid!");
    let door_loop_size = discrete_logarithm(device_base, divider, 6552760).expect("Door invalid!");

    let encryption_key = pow_mod(pow_mod(device_base, card_loop_size, divider), door_loop_size, divider);
    println!("The encryption key is {}.", encryption_key);
}

fn main() {
    solve_part_one();
}
