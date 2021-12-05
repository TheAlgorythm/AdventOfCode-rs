use std::cmp::Ordering;
use std::num::ParseIntError;
use std::ops::Sub;
use std::str::FromStr;

struct SeatPosition {
    pub row: u32,
    pub column: u32,
}

impl SeatPosition {
    fn get_id(&self) -> u32 {
        (self.row * 8) + self.column
    }
}

impl Sub<u32> for &SeatPosition {
    type Output = SeatPosition;

    fn sub(self, other: u32) -> Self::Output {
        let new_id = self.get_id() - other;
        let column = new_id % 8;
        let row = (new_id - column) / 8;
        SeatPosition { row, column }
    }
}

impl FromStr for SeatPosition {
    type Err = ParseIntError;

    fn from_str(seat: &str) -> Result<Self, Self::Err> {
        let converted_seat: String = seat
            .chars()
            .map(|position_specifier| match position_specifier {
                'B' | 'R' => '1',
                'F' | 'L' => '0',
                character => panic!("Character {} not specified!", character),
            })
            .collect();
        let row = u32::from_str_radix(&converted_seat[0..7], 2)?;
        let column = u32::from_str_radix(&converted_seat[7..10], 2)?;
        Ok(SeatPosition { row, column })
    }
}

impl Ord for SeatPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}

impl PartialOrd for SeatPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SeatPosition {}

impl PartialEq for SeatPosition {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

fn parse_seats(input: &str) -> Vec<SeatPosition> {
    let mut seats = input
        .lines()
        .map(str::parse::<SeatPosition>)
        .collect::<Result<Vec<SeatPosition>, ParseIntError>>()
        .expect("Parse error!");
    seats.sort_unstable();
    seats
}

fn solve_part_one(seats: &[SeatPosition]) {
    let highest_seat = seats.last().expect("No highest seat found!");
    println!("The highest seat ID is {}.", highest_seat.get_id());
}

fn solve_part_two(seats: &[SeatPosition]) {
    let my_seat = seats
        .iter()
        .scan(0_u32, |last_id, current| {
            let distance = current.get_id() - *last_id;
            *last_id = current.get_id();
            Some((distance, current))
        })
        .filter(|(distance, _seat)| *distance == 2_u32)
        .map(|(_distance, seat)| seat - 1_u32)
        .next()
        .expect("No free seat!");
    println!(
        "My seat {} is in row {} column {}.",
        my_seat.get_id(),
        my_seat.row,
        my_seat.column
    );
}

fn main() {
    let input = include_str!("05_data.list");

    let seats = parse_seats(input);

    solve_part_one(&seats);
    solve_part_two(&seats);
}
