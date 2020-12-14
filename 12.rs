use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseActionError {
    UnknownAction,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseActionError {
    fn from(err: ParseIntError) -> ParseActionError {
        ParseActionError::ParseInt(err)
    }
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_degrees(&self) -> u32 {
        match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        }
    }

    fn from_degrees(degrees: u32) -> Self {
        match degrees % 360 {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            degrees => panic!("There is no matching direction for {} degrees.", degrees),
        }
    }

    fn left(&mut self, degrees: u32) {
        *self = Direction::from_degrees(self.to_degrees() + 360 - degrees);
    }

    fn right(&mut self, degrees: u32) {
        *self = Direction::from_degrees(self.to_degrees() + degrees);
    }
}

#[derive(Clone, Debug)]
enum Action {
    Move(Direction, u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.chars();

        let action = match elements.next() {
            Some(character) => character,
            None => return Err(ParseActionError::UnknownAction),
        };
        let number: u32 = elements.collect::<String>().parse()?;

        match action {
            'N' => Ok(Action::Move(Direction::North, number)),
            'E' => Ok(Action::Move(Direction::East, number)),
            'S' => Ok(Action::Move(Direction::South, number)),
            'W' => Ok(Action::Move(Direction::West, number)),
            'L' => Ok(Action::Left(number % 360)),
            'R' => Ok(Action::Right(number % 360)),
            'F' => Ok(Action::Forward(number)),
            _ => Err(ParseActionError::UnknownAction),
        }
    }
}

fn parse_actions(input: &str) -> Result<Vec<Action>, ParseActionError> {
    input.lines().map(str::parse::<Action>).collect()
}

fn manhattan_distance(actions: &Vec<Action>, mut current_view: Direction) -> u32 {
    let mut north_distance = 0_i32;
    let mut east_distance = 0_i32;
    for action in actions.iter() {
        let action = match action {
            Action::Forward(steps) => Action::Move(current_view.clone(), *steps),
            action => action.clone(),
        };
        match action {
            Action::Move(Direction::North, steps) => north_distance += steps as i32,
            Action::Move(Direction::East, steps) => east_distance += steps as i32,
            Action::Move(Direction::South, steps) => north_distance -= steps as i32,
            Action::Move(Direction::West, steps) => east_distance -= steps as i32,
            Action::Left(degree) => current_view.left(degree),
            Action::Right(degree) => current_view.right(degree),
            _ => unreachable!(),
        }
    }
    north_distance.abs() as u32 + east_distance.abs() as u32
}

fn solve_part_one(actions: &Vec<Action>) {
    let distance = manhattan_distance(&actions, Direction::East);
    println!(
        "The Manhattan distance between that location and the ship's starting position is {}.",
        distance
    );
}

fn manhattan_distance_with_waypoints(
    actions: &Vec<Action>,
    mut waypoint_north_coordinate: i32,
    mut waypoint_east_coordinate: i32,
) -> u32 {
    let mut north_distance = 0_i32;
    let mut east_distance = 0_i32;
    for action in actions.iter() {
        match action {
            Action::Forward(steps) => {
                north_distance += waypoint_north_coordinate * *steps as i32;
                east_distance += waypoint_east_coordinate * *steps as i32;
            }
            Action::Move(Direction::North, steps) => waypoint_north_coordinate += *steps as i32,
            Action::Move(Direction::East, steps) => waypoint_east_coordinate += *steps as i32,
            Action::Move(Direction::South, steps) => waypoint_north_coordinate -= *steps as i32,
            Action::Move(Direction::West, steps) => waypoint_east_coordinate -= *steps as i32,
            Action::Left(degree) => match degree {
                0 => {}
                90 => {
                    let new_north = waypoint_east_coordinate;
                    waypoint_east_coordinate = -waypoint_north_coordinate;
                    waypoint_north_coordinate = new_north;
                }
                180 => {
                    waypoint_north_coordinate *= -1;
                    waypoint_east_coordinate *= -1;
                }
                270 => {
                    let new_north = -waypoint_east_coordinate;
                    waypoint_east_coordinate = waypoint_north_coordinate;
                    waypoint_north_coordinate = new_north;
                }
                degrees => panic!("There is no matching direction for {} degrees.", degrees),
            },
            Action::Right(degree) => match degree {
                0 => {}
                90 => {
                    let new_north = -waypoint_east_coordinate;
                    waypoint_east_coordinate = waypoint_north_coordinate;
                    waypoint_north_coordinate = new_north;
                }
                180 => {
                    waypoint_north_coordinate *= -1;
                    waypoint_east_coordinate *= -1;
                }
                270 => {
                    let new_north = waypoint_east_coordinate;
                    waypoint_east_coordinate = -waypoint_north_coordinate;
                    waypoint_north_coordinate = new_north;
                }
                degrees => panic!("There is no matching direction for {} degrees.", degrees),
            },
        }
    }
    north_distance.abs() as u32 + east_distance.abs() as u32
}

fn solve_part_two(actions: &Vec<Action>) {
    let distance = manhattan_distance_with_waypoints(&actions, 1, 10);
    println!(
        "The Manhattan distance between that location and the ship's starting position is {} using waypoints.",
        distance
    );
}

fn main() -> Result<(), ParseActionError> {
    let input = include_str!("12_data.txt");

    let actions = parse_actions(&input)?;

    solve_part_one(&actions);
    solve_part_two(&actions);

    Ok(())
}
