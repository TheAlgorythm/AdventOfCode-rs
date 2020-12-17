use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Position {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Position { x, y, z, w }
    }

    fn get_block(&self, is_4_dimensional: bool) -> Vec<Self> {
        (-1..=1)
            .map(|x_offset| {
                (-1..=1)
                    .map(|y_offset| {
                        (-1..=1)
                            .map(|z_offset| {
                                (-1 * is_4_dimensional as i64..=1 * is_4_dimensional as i64)
                                    .map(|w_offset| {
                                        Position::new(
                                            self.x + x_offset,
                                            self.y + y_offset,
                                            self.z + z_offset,
                                            self.w + w_offset,
                                        )
                                    })
                                    .collect::<Vec<Self>>()
                            })
                            .flatten()
                            .collect::<Vec<Self>>()
                    })
                    .flatten()
                    .collect::<Vec<Self>>()
            })
            .flatten()
            .collect()
    }

    fn get_neighbors(&self, is_4_dimensional: bool) -> Vec<Self> {
        (-1..=1)
            .map(|x_offset| {
                (-1..=1)
                    .map(|y_offset| {
                        (-1..=1)
                            .map(|z_offset| {
                                (-1 * is_4_dimensional as i64..=1 * is_4_dimensional as i64)
                                    .filter(|w_offset| {
                                        x_offset != 0
                                            || y_offset != 0
                                            || z_offset != 0
                                            || *w_offset != 0
                                    })
                                    .map(|w_offset| {
                                        Position::new(
                                            self.x + x_offset,
                                            self.y + y_offset,
                                            self.z + z_offset,
                                            self.w + w_offset,
                                        )
                                    })
                                    .collect::<Vec<Self>>()
                            })
                            .flatten()
                            .collect::<Vec<Self>>()
                    })
                    .flatten()
                    .collect::<Vec<Self>>()
            })
            .flatten()
            .collect()
    }
}

fn parse_map(input: &str) -> HashSet<Position> {
    input
        .lines()
        .rev()
        .enumerate()
        .map(|(y_index, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x_index, character)| *character == '#')
                .map(|(x_index, _char)| Position::new(x_index as i64, y_index as i64, 0, 0))
                .collect::<HashSet<Position>>()
        })
        .flatten()
        .collect::<HashSet<Position>>()
}

fn cycle(active_cubes: &HashSet<Position>, is_4_dimensional: bool) -> HashSet<Position> {
    active_cubes
        .iter()
        .map(|current_active| current_active.get_block(is_4_dimensional))
        .flatten()
        .filter(|possible_cube| {
            let is_active = active_cubes.contains(possible_cube);
            let neighbors = possible_cube
                .get_neighbors(is_4_dimensional)
                .into_iter()
                .filter(|neighbor| active_cubes.contains(neighbor))
                .count();
            match (is_active, neighbors) {
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            }
        })
        .collect()
}

fn cycles(
    active_cubes: &HashSet<Position>,
    nth: usize,
    is_4_dimensional: bool,
) -> HashSet<Position> {
    let mut current_active = cycle(&active_cubes, is_4_dimensional);
    for _ in 1..nth {
        current_active = cycle(&current_active, is_4_dimensional);
    }
    current_active
}

fn solve_part_one(active_cubes: &HashSet<Position>) {
    let cubes = cycles(active_cubes, 6, false).len();
    println!(
        "There are {} cubes left in the active state after the sixth cycle.",
        cubes
    );
}

fn solve_part_two(active_cubes: &HashSet<Position>) {
    let cubes = cycles(active_cubes, 6, true).len();
    println!(
        "There are {} cubes left in the active state after the sixth cycle in 4 dimensions.",
        cubes
    );
}

fn main() {
    let input = include_str!("17_data.map");

    let active_cubes = parse_map(&input);

    solve_part_one(&active_cubes);
    solve_part_two(&active_cubes);
}
