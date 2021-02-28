use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Tile {
    content: Vec<Vec<bool>>,
    borders: Vec<Vec<bool>>,
}

impl Tile {
    fn new(content: Vec<Vec<bool>>) -> Self {
        let mut borders = Vec::new();
        borders.push(content.first().expect("No upper border!").to_vec());
        borders.push(content.last().expect("No lower border!").to_vec());
        borders.push(
            content
                .iter()
                .map(|line| line.first().expect("No left border!").clone())
                .collect(),
        );
        borders.push(
            content
                .iter()
                .map(|line| line.last().expect("No right border!").clone())
                .collect(),
        );

        Tile { content, borders }
    }

    pub fn get_borders(&self) -> Vec<Vec<bool>> {
        self.borders.to_vec()
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        other.get_borders().into_iter().any(|other_border| {
            self.borders.contains(&other_border)
                || self
                    .borders
                    .contains(&other_border.into_iter().rev().collect())
        })
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tile::new(
            s.lines()
                .map(|line| line.chars().map(|character| character == '#').collect())
                .collect(),
        ))
    }
}

type Tiles = HashMap<u16, Tile>;

fn parse_tiles(input: &str) -> Tiles {
    input
        .split("\n\n")
        .map(|tile_block| {
            let tile_elements: Vec<&str> = tile_block.splitn(2, "\n").collect();
            let tile_id = tile_elements[0]
                .chars()
                .skip(5)
                .take(4)
                .collect::<String>()
                .parse()
                .expect("Couldn't parse ID!");
            (
                tile_id,
                tile_elements[1].parse().expect("Couldn't parse Tile"),
            )
        })
        .collect()
}

fn get_corner_tiles(tiles: &Tiles) -> Vec<u16> {
    tiles
        .iter()
        .filter(|(tile_id, tile)| {
            tiles
                .iter()
                .filter(|(other_tile_id, other_tile)| {
                    tile_id != other_tile_id && tile.is_adjacent(other_tile)
                })
                .take(3)
                .count()
                == 2
        })
        .map(|(tile_id, _tile)| *tile_id)
        .take(4)
        .collect()
}

fn solve_part_one(tiles: &Tiles) {
    let corner_product: u64 = get_corner_tiles(&tiles)
        .into_iter()
        .map(|tile_id| tile_id as u64)
        .product();

    println!(
        "The product of the IDs of the four corner tiles is {}.",
        corner_product
    );
}

fn solve_part_two() {}

fn main() {
    let input = include_str!("20_data.raw");

    let tiles = parse_tiles(&input);

    solve_part_one(&tiles);
    solve_part_two();
}
