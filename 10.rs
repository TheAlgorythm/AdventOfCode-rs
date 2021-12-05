use std::collections::BTreeMap;

fn parse_adapters(input: &str) -> Vec<u64> {
    let mut adapters = input
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .expect("Parsing failed!");

    adapters.sort_unstable();
    adapters
}

fn solve_part_one(adapters: &[u64]) {
    let diff_count = adapters
        .iter()
        .scan(0, |previous, &current| {
            let difference = current - *previous;
            *previous = current;
            Some(difference)
        })
        .fold(BTreeMap::new(), |mut aggregator, difference| {
            aggregator
                .entry(difference)
                .and_modify(|count| *count += 1)
                .or_insert(1_u64);
            aggregator
        });
    println!(
        "The charging product is: {}",
        diff_count.get(&1).unwrap_or(&0) * (diff_count.get(&3).unwrap_or(&0) + 1)
    );
}

fn count_mutations(adapters: &[u64], last_value: u64, cache: &mut BTreeMap<u64, u64>) -> u64 {
    if cache.contains_key(&last_value) {
        return cache[&last_value];
    }

    if last_value + 3 < adapters[0] {
        return 0;
    }
    if adapters.len() == 1 {
        return 1;
    }

    let mutations = count_mutations(&adapters[1..], adapters[0], cache)
        + count_mutations(&adapters[1..], last_value, cache);
    cache.insert(last_value, mutations);
    mutations
}

fn solve_part_two(adapters: &[u64]) {
    let mutations = count_mutations(adapters, 0, &mut BTreeMap::new());
    println!("There are {} valid mutations.", mutations);
}

fn main() {
    let input = include_str!("10_data.list");

    let adapters = parse_adapters(input);

    solve_part_one(&adapters);
    solve_part_two(&adapters);
}
