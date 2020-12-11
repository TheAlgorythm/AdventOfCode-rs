#[derive(Clone, Debug)]
enum PositionState {
    Floor,
    Empty,
    Occupied,
}

impl PositionState {
    pub fn is_occupied(&self) -> bool {
        match self {
            PositionState::Occupied => true,
            _ => false,
        }
    }

    pub fn calculate_new(
        &self,
        map: &Map,
        line_index: usize,
        column_index: usize,
        value_changed: &mut bool,
    ) -> Self {
        match self {
            PositionState::Floor => return PositionState::Floor,
            _ => {}
        };
        let neighborhood_occupied_count: usize = map
            .iter()
            .skip(get_first_border(line_index))
            .take(get_width(line_index, map.len()))
            .map(|line| {
                line.iter()
                    .skip(get_first_border(column_index))
                    .take(get_width(column_index, line.len()))
                    .filter(|state| state.is_occupied())
                    .count()
            })
            .sum();
        match self {
            PositionState::Empty => {
                if neighborhood_occupied_count == 0 {
                    *value_changed |= true;
                    PositionState::Occupied
                } else {
                    PositionState::Empty
                }
            }
            PositionState::Occupied => {
                if neighborhood_occupied_count >= 5 {
                    *value_changed |= true;
                    PositionState::Empty
                } else {
                    PositionState::Occupied
                }
            }
            _ => unreachable!(),
        }
    }
}

fn get_first_border(center: usize) -> usize {
    if center == 0 {
        return 0;
    }
    center - 1
}

fn get_width(center: usize, size: usize) -> usize {
    let mut default_width = 3;
    if center == 0 {
        default_width -= 1;
    }
    if center == size - 1 {
        default_width -= 1;
    }
    default_width
}

impl From<char> for PositionState {
    fn from(character: char) -> Self {
        match character {
            '.' => PositionState::Floor,
            'L' => PositionState::Empty,
            '#' => PositionState::Occupied,
            _ => panic!("Character not valid!"),
        }
    }
}

type Map = Vec<Vec<PositionState>>;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(PositionState::from).collect())
        .collect()
}

fn seat_behaviour_engine(map: &Map) -> (Map, bool) {
    let mut value_changed = false;
    (
        map.iter()
            .enumerate()
            .map(|(line_number, line)| {
                line.iter()
                    .enumerate()
                    .map(|(column, state)| {
                        state.calculate_new(&map, line_number, column, &mut value_changed)
                    })
                    .collect::<Vec<PositionState>>()
            })
            .collect(),
        value_changed,
    )
}

fn solve_part_one(map: &Map) {
    let mut old_map = map.to_vec();
    loop {
        let (updated_map, value_changed) = seat_behaviour_engine(&old_map);
        if !value_changed {
            break;
        }
        old_map = updated_map;
    }
    let occupied_seats: usize = old_map
        .iter()
        .map(|line| line.iter().filter(|state| state.is_occupied()).count())
        .sum();
    println!("There end up {} seats occupied.", occupied_seats);
}

fn solve_part_two(_map: &Map) {}

fn main() {
    let input = include_str!("11_data.map");

    let map = parse_map(&input);

    solve_part_one(&map);
    solve_part_two(&map);
}
