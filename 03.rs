use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct RingAdder {
    next: u64,
    step_width: u64,
    modulus: u64,
}

impl RingAdder {
    fn new(start: u64, step_width: u64, modulus: u64) -> Self {
        RingAdder {
            next: start,
            step_width,
            modulus,
        }
    }
}

impl Iterator for RingAdder {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.next;
        self.next += self.step_width;
        self.next %= self.modulus;
        Some(current)
    }
}

fn solve_part_one(slope_map: &Vec<String>) {
    let mut x_coords_ring = RingAdder::new(0, 3, 31);

    let trees_on_slope = slope_map
        .iter()
        .map(|line| {
            line.chars()
                .nth(x_coords_ring.next().unwrap() as usize)
                .expect("Index out of range")
        })
        .filter(|symbol| *symbol == '#')
        .count();

    println!("There are {} trees on the slope.", trees_on_slope);
}

fn solve_part_two(slope_map: &Vec<String>) {
    let slopes = vec![
        (1, RingAdder::new(0, 1, 31)),
        (1, RingAdder::new(0, 3, 31)),
        (1, RingAdder::new(0, 5, 31)),
        (1, RingAdder::new(0, 7, 31)),
        (2, RingAdder::new(0, 1, 31)),
    ];

    let slope_product: u64 = slopes
        .into_iter()
        .map(|(y_slope, mut x_slope)| {
            slope_map
                .iter()
                .step_by(y_slope)
                .map(|line| {
                    line.chars()
                        .nth(x_slope.next().unwrap() as usize)
                        .expect("Index out of range")
                })
                .filter(|symbol| *symbol == '#')
                .count() as u64
        })
        .product();

    println!("The slope-product is {},", slope_product);
}

fn main() -> std::io::Result<()> {
    let file = File::open("03_data.map")?;
    let reader = BufReader::new(file);
    let slope_map: Vec<String> = reader.lines().collect::<std::io::Result<Vec<String>>>()?;

    solve_part_one(&slope_map);
    solve_part_two(&slope_map);

    Ok(())
}
