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
    fn to_degrees(&self) -> u16 {
        match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        }
    }

    fn from_degrees(degrees: u16) -> Self {
        match degrees % 360 {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            degrees => panic!("There is no matching direction for {} degrees.", degrees),
        }
    }

    fn left(&mut self, degrees: u16) {
        *self = Direction::from_degrees(self.to_degrees() + 360 - degrees);
    }

    fn right(&mut self, degrees: u16) {
        *self = Direction::from_degrees(self.to_degrees() + degrees);
    }
}

#[derive(Clone, Debug)]
enum Action {
    Move(Direction, u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.chars();

        let action = match elements.next() {
            Some(character) => character,
            None => return Err(ParseActionError::UnknownAction),
        };
        let number: u16 = elements.collect::<String>().parse()?;

        match action {
            'N' => Ok(Action::Move(Direction::North, number)),
            'E' => Ok(Action::Move(Direction::East, number)),
            'S' => Ok(Action::Move(Direction::South, number)),
            'W' => Ok(Action::Move(Direction::West, number)),
            'L' => Ok(Action::Left(number)),
            'R' => Ok(Action::Right(number)),
            'F' => Ok(Action::Forward(number)),
            _ => Err(ParseActionError::UnknownAction),
        }
    }
}

fn parse_actions(input: &str) -> Result<Vec<Action>, ParseActionError> {
    input.lines().map(str::parse::<Action>).collect()
}

fn manhattan_distance(actions: &Vec<Action>) -> u16 {
    let mut current_view = Direction::East;
    let mut north_distance = 0_i16;
    let mut east_distance = 0_i16;
    for action in actions.iter() {
        let action = match action {
            Action::Forward(steps) => Action::Move(current_view.clone(), *steps),
            action => action.clone(),
        };
        match action {
            Action::Move(Direction::North, steps) => north_distance += steps as i16,
            Action::Move(Direction::East, steps) => east_distance += steps as i16,
            Action::Move(Direction::South, steps) => north_distance -= steps as i16,
            Action::Move(Direction::West, steps) => east_distance -= steps as i16,
            Action::Left(degree) => current_view.left(degree),
            Action::Right(degree) => current_view.right(degree),
            _ => unreachable!(),
        }
    }
    north_distance.abs() as u16 + east_distance.abs() as u16
}

fn solve_part_one(actions: &Vec<Action>) {
    let distance = manhattan_distance(&actions);
    println!(
        "The Manhattan distance between that location and the ship's starting position is {}.",
        distance
    );
}

fn solve_part_two(_actions: &Vec<Action>) {}

fn main() -> Result<(), ParseActionError> {
    let input = include_str!("12_data.txt");

    let actions = parse_actions(&input)?;

    solve_part_one(&actions);
    solve_part_two(&actions);

    Ok(())
}
