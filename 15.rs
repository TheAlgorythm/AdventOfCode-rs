#![feature(min_const_generics)]

use std::collections::HashMap;

fn parse_num_list(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .expect("Couldn't parse input!")
}

struct SplittedMap<V, const STATIC_SIZE: usize> {
    hot_table: [Option<V>; STATIC_SIZE],
    pub cold_map: HashMap<usize, V>,
}

impl<V, const STATIC_SIZE: usize> SplittedMap<V, STATIC_SIZE>
where
    V: Copy,
    Option<V>: Copy,
{
    fn new() -> Self {
        Self {
            hot_table: [None; STATIC_SIZE],
            cold_map: HashMap::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            hot_table: [None; STATIC_SIZE],
            cold_map: HashMap::with_capacity(capacity),
        }
    }

    fn get(&self, key: &usize) -> Option<V> {
        if *key < STATIC_SIZE {
            unsafe {
                return *self.hot_table.get_unchecked(*key);
            }
        } else {
            return self.cold_map.get(key).map(|value| *value);
        }
    }

    fn set(&mut self, key: usize, value: V) {
        if key < STATIC_SIZE {
            unsafe {
                *self.hot_table.get_unchecked_mut(key) = Some(value);
            }
        } else {
            self.cold_map.insert(key, value);
        }
    }
}

fn get_nth_number(starting_values: &Vec<u32>, nth: usize) -> u32 {
    let mut last_values: SplittedMap<u32, { 1024 * 2048 }> =
        SplittedMap::with_capacity(2039983 * (nth > { 1024 * 2048 }) as usize);
    starting_values
        .iter()
        .take(starting_values.len() - 1)
        .enumerate()
        .for_each(|(index, value)| last_values.set(*value as usize, index as u32 + 1));

    let mut last_value = *starting_values.last().expect("No starting value!") as usize;

    (starting_values.len()..nth).for_each(|index| {
        let current_value = last_values
            .get(&last_value)
            .map(|last_occuring_index| index as u32 - last_occuring_index)
            .unwrap_or(0);

        last_values.set(last_value, index as u32);
        last_value = current_value as usize;
    });
    last_value as u32
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
