type Map = Vec<Vec<PositionState>>;
type MapSlice = [Vec<PositionState>];

#[derive(Clone, Debug)]
enum PositionState {
    Floor,
    Empty,
    Occupied,
}

impl PositionState {
    pub fn is_occupied(&self) -> bool {
        matches!(self, PositionState::Occupied)
    }

    pub fn is_seat(&self) -> bool {
        !matches!(self, PositionState::Floor)
    }

    pub fn calculate_new<'a>(
        &self,
        count_occupied: &'a dyn Fn(&MapSlice, usize, usize) -> u32,
        map: &MapSlice,
        line_index: usize,
        column_index: usize,
        disallowed_occupied: u32,
        stabilized: &mut bool,
    ) -> Self {
        match self {
            PositionState::Floor => PositionState::Floor,
            PositionState::Empty => {
                if (count_occupied)(map, line_index, column_index) == 0 {
                    *stabilized = false;
                    PositionState::Occupied
                } else {
                    PositionState::Empty
                }
            }
            PositionState::Occupied => {
                if (count_occupied)(map, line_index, column_index) >= disallowed_occupied {
                    *stabilized = false;
                    PositionState::Empty
                } else {
                    PositionState::Occupied
                }
            }
        }
    }
}

fn get_first_border(center: usize) -> usize {
    center - (center != 0) as usize
}

fn get_width(center: usize, size: usize) -> usize {
    let mut default_width = 3;
    default_width -= (center == 0) as usize;
    default_width -= (center == size - 1) as usize;
    default_width
}

fn count_occupied_neighbors(map: &MapSlice, line_index: usize, column_index: usize) -> u32 {
    map.iter()
        .skip(get_first_border(line_index))
        .take(get_width(line_index, map.len()))
        .map(|line| {
            line.iter()
                .skip(get_first_border(column_index))
                .take(get_width(column_index, line.len()))
                .filter(|state| state.is_occupied())
                .count() as u32
        })
        .sum::<u32>()
        - (map[line_index][column_index].is_occupied() as u32)
}

fn count_occupied_axis(map: &MapSlice, line_index: usize, column_index: usize) -> u32 {
    static DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    DIRECTIONS
        .iter()
        .map(|vector| get_occupied_axis(map, line_index, column_index, *vector) as u32)
        .sum()
}

fn get_occupied_axis(
    map: &MapSlice,
    line_index: usize,
    column_index: usize,
    vector: (isize, isize),
) -> bool {
    !matches!(
        (1..)
            .map(|n| {
                (
                    line_index as isize + (vector.0 * n),
                    column_index as isize + (vector.1 * n),
                )
            })
            .take_while(|(line_vector, column_vector)| {
                (0..map.len() as isize).contains(line_vector)
                    && (0..map[*line_vector as usize].len() as isize).contains(column_vector)
            })
            .map(|(line_vector, column_vector)| {
                map[line_vector as usize][column_vector as usize].clone()
            })
            .find(|state| state.is_seat()),
        None | Some(PositionState::Empty)
    )
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

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(PositionState::from).collect())
        .collect()
}

struct BehaviourEngine<'a> {
    map: Map,
    count_occupied: &'a dyn Fn(&MapSlice, usize, usize) -> u32,
    disallowed_occupied: u32,
    stabilized: bool,
}

impl<'a> BehaviourEngine<'a> {
    pub fn run_to_stabilized(&mut self) {
        while !self.stabilized {
            self.step();
        }
    }

    fn step(&mut self) {
        self.stabilized = true;
        self.map = self
            .map
            .to_vec()
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                line.iter()
                    .enumerate()
                    .map(|(column_index, state)| {
                        state.calculate_new(
                            self.count_occupied,
                            &self.map,
                            line_index,
                            column_index,
                            self.disallowed_occupied,
                            &mut self.stabilized,
                        )
                    })
                    .collect::<Vec<PositionState>>()
            })
            .collect();
    }

    pub fn get_all_occupied(&self) -> u32 {
        self.map
            .iter()
            .map(|line| line.iter().filter(|state| state.is_occupied()).count() as u32)
            .sum()
    }
}

fn solve_part_one(map: &MapSlice) {
    let mut engine = BehaviourEngine {
        map: map.to_vec(),
        count_occupied: &count_occupied_neighbors,
        disallowed_occupied: 4,
        stabilized: false,
    };
    engine.run_to_stabilized();
    let occupied_seats = engine.get_all_occupied();
    println!(
        "There end up {} seats occupied (neighbor model).",
        occupied_seats
    );
}

fn solve_part_two(map: Map) {
    let mut engine = BehaviourEngine {
        map,
        count_occupied: &count_occupied_axis,
        disallowed_occupied: 5,
        stabilized: false,
    };
    engine.run_to_stabilized();
    let occupied_seats = engine.get_all_occupied();
    println!(
        "There end up {} seats occupied (axis model).",
        occupied_seats
    );
}

fn main() {
    let input = include_str!("11_data.map");

    let map = parse_map(input);

    solve_part_one(&map);
    solve_part_two(map);
}
