use std::collections::HashMap;

fn pow_mod(base: u64, exponent: u64, divider: u64) -> u64 {
    let mut base = base as u128;
    let mut exponent = exponent as u128;
    let divider = divider as u128;
    let mut result = 1_u128;
    while exponent != 0 {
        if exponent % 2 != 0 {
            result *= base;
            result %= divider;
        }
        exponent >>= 1;
        base *= base;
        base %= divider;
    }
    result as u64
}

/// Returns the exponent of the formula: base^exponent mod divider = result
/// It uses the Baby-step giant-step algorithm.
fn discrete_logarithm(base: u64, divider: u64, result: u64) -> Option<u64> {
    let base = base % divider;
    let result = result % divider;

    let big_step_size = (divider as f64).sqrt().ceil() as u64;

    let big_values: HashMap<u64, u64> = (1..=big_step_size)
        .map(|big_step| (pow_mod(base, big_step * big_step_size, divider), big_step))
        .collect();

    (1..=big_step_size)
        .map(|baby_step| {
            (
                baby_step,
                ((pow_mod(base, baby_step, divider) as u128 * result as u128) % divider as u128)
                    as u64,
            )
        })
        .filter_map(|(baby_step, baby_value)| {
            big_values
                .get(&baby_value)
                .map(|big_step| big_step * big_step_size - baby_step)
        })
        .filter(|exponent| *exponent < divider)
        .next()
}

/// Break Diffie-Hellman
fn solve_part_one() {
    let device_base = 7;
    let divider = 20201227;

    let card_exponent = discrete_logarithm(device_base, divider, 13233401).expect("Card invalid!");
    let door_exponent = discrete_logarithm(device_base, divider, 6552760).expect("Door invalid!");

    let encryption_key = pow_mod(
        pow_mod(device_base, card_exponent, divider),
        door_exponent,
        divider,
    );
    println!("The encryption key is {}.", encryption_key);
}

fn main() {
    solve_part_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let device_base = 7;
        let divider = 20201227;

        let card_exponent =
            discrete_logarithm(device_base, divider, 13233401).expect("Card invalid!");
        let door_exponent =
            discrete_logarithm(device_base, divider, 6552760).expect("Door invalid!");

        assert_eq!(card_exponent, 16679169);
        assert_eq!(door_exponent, 7725933);
    }

    #[test]
    fn simple_examples() {
        assert_eq!(discrete_logarithm(2, 5, 3).unwrap(), 3);
        assert!(discrete_logarithm(3, 11, 7).is_none());
    }
}
