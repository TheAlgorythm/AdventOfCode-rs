mod utils;
use utils::maths::chinese_remainder;

fn parse_plan(input: &str) -> (u32, Vec<(usize, u32)>) {
    let elements: Vec<&str> = input.splitn(2, '\n').collect();
    (
        elements[0].parse().expect("Couldn't parse arrival!"),
        elements[1]
            .split(',')
            .enumerate()
            .filter(|(_index, id)| *id != "x")
            .map(|(index, id)| (index, str::parse::<u32>(id).expect("Couldn't parse ID!")))
            .collect(),
    )
}

fn get_next_bus(arrival: u32, bus_ids: &[u32]) -> (u32, u32) {
    bus_ids
        .iter()
        .map(|id| (*id, id - (arrival % id)))
        .min_by_key(|(_id, waiting_time)| *waiting_time)
        .expect("No next bus found!")
}

fn solve_part_one(arrival: u32, bus_ids: &[(usize, u32)]) {
    let (bus_id, waiting_time) = get_next_bus(
        arrival,
        &bus_ids.iter().map(|(_index, id)| *id).collect::<Vec<_>>(),
    );
    println!(
        "For the next bus {} you have to wait {} minutes, which is a waiting-product of {}.",
        bus_id,
        waiting_time,
        bus_id * waiting_time
    );
}

fn get_bus_row(bus_ids: &[(usize, u32)]) -> i64 {
    chinese_remainder(
        &bus_ids
            .iter()
            .map(|(index, id)| (*id as i64, *id as i64 - *index as i64))
            .collect::<Vec<_>>(),
    )
}

fn solve_part_two(bus_ids: &[(usize, u32)]) {
    let first_time = get_bus_row(bus_ids);
    println!("The first bus row is at timestamp {}.", first_time);
}

fn main() {
    let input = include_str!("13_data.txt");

    let (arrival, ids) = parse_plan(input);

    solve_part_one(arrival, &ids);
    solve_part_two(&ids);
}
