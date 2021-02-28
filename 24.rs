use std::collections::{BTreeSet, HashMap};
mod utils;
use utils::unique::*;

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from_directions(directions: &str) -> Self {
        let north_west_steps = directions.matches("nw").count() as i32;
        let north_east_steps = directions.matches("ne").count() as i32;
        let south_west_steps = directions.matches("sw").count() as i32;
        let south_east_steps = directions.matches("se").count() as i32;
        let directions = directions
            .replace("nw", "")
            .replace("ne", "")
            .replace("sw", "")
            .replace("se", "");
        let east_steps = directions.matches('e').count() as i32;
        let west_steps = directions.matches('w').count() as i32;

        Coordinate {
            x: east_steps - west_steps + north_east_steps - south_west_steps,
            y: north_west_steps - south_east_steps + north_east_steps - south_west_steps,
        }
    }

    fn get_block(&self) -> Vec<Self> {
        (-1..=1)
            .map(|x_offset| {
                (-1..=1)
                    .filter(|y_offset| x_offset * y_offset != -1)
                    .map(|y_offset| Coordinate {
                        x: self.x + x_offset,
                        y: self.y + y_offset,
                    })
                    .collect::<Vec<Self>>()
            })
            .flatten()
            .collect()
    }

    fn get_neighbors(&self, active_cubes: &BTreeSet<Self>) -> u32 {
        (-1..=1)
            .map(|x_offset| {
                (-1..=1)
                    .filter(|y_offset| {
                        (x_offset != 0 || *y_offset != 0) && x_offset * y_offset != -1
                    })
                    .map(|y_offset| Coordinate {
                        x: self.x + x_offset,
                        y: self.y + y_offset,
                    })
                    .filter(|neighbor| active_cubes.contains(neighbor))
                    .count() as u32
            })
            .sum()
    }
}

fn parse_tiles(input: &str) -> Vec<Coordinate> {
    input.lines().map(Coordinate::from_directions).collect()
}

fn get_black_tiles(flipped_tiles: Vec<Coordinate>) -> BTreeSet<Coordinate> {
    flipped_tiles
        .into_iter()
        .fold(HashMap::new(), |mut tiles, coordinate| {
            tiles
                .entry(coordinate)
                .and_modify(|flips| *flips += 1)
                .or_insert(1_usize);
            tiles
        })
        .into_iter()
        .filter(|(_coordinate, flips)| flips % 2 == 1)
        .map(|(coordinate, _flips)| coordinate)
        .collect()
}

fn solve_part_one(black_tiles: &BTreeSet<Coordinate>) {
    println!(
        "{} tiles are left with the black side up.",
        black_tiles.len()
    );
}

fn cycle(black_tiles: &BTreeSet<Coordinate>) -> BTreeSet<Coordinate> {
    black_tiles
        .iter()
        .map(|current_black| current_black.get_block())
        .flatten()
        .unique()
        .filter(|possible_black| {
            let is_black = black_tiles.contains(possible_black);
            let neighbors = possible_black.get_neighbors(black_tiles);
            match (is_black, neighbors) {
                (true, 1..=2) => true,
                (false, 2) => true,
                _ => false,
            }
        })
        .collect()
}

fn cycles(black_tiles: &BTreeSet<Coordinate>, nth: usize) -> BTreeSet<Coordinate> {
    let mut current_black = cycle(&black_tiles);
    for _ in 1..nth {
        current_black = cycle(&current_black);
    }
    current_black
}

fn solve_part_two(black_tiles: &BTreeSet<Coordinate>) {
    let progressed_black_tiles = cycles(black_tiles, 100);
    println!(
        "After 100 days, there are {} tiles left with the black side up.",
        progressed_black_tiles.len()
    );
}

fn main() {
    let input = include_str!("24_data.txt");

    let flipped_tiles = parse_tiles(&input);

    let black_tiles = get_black_tiles(flipped_tiles);

    solve_part_one(&black_tiles);
    solve_part_two(&black_tiles);
}
