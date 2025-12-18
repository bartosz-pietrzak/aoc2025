use anyhow::{Context, Result};
use std::fs::read_to_string;

#[derive(Debug)]
struct Jbox {
    x: usize,
    y: usize,
    z: usize,
}

impl Jbox {
    fn distance_to(&self, other: &Jbox) -> f64 {
        let dx = other.x as f64 - self.x as f64;
        let dy = other.y as f64 - self.y as f64;
        let dz = other.z as f64 - self.z as f64;
        let sum_of_squares = dx * dx + dy * dy + dz * dz;
        sum_of_squares.sqrt()
    }
}

fn main() -> Result<()> {
    let path = "test";
    let file = read_to_string(path).context("Failed to read file to string")?;
    let all_jboxes: Result<Vec<Jbox>> = file
        .lines()
        .map(|line: &str| -> Result<Jbox> {
            let mut coords = line.split(",").map(|coord| -> Result<usize> {
                let parsed_coord = coord.parse::<usize>().context("Invalid coordinate")?;
                Ok(parsed_coord)
            });
            Ok(Jbox {
                x: coords.next().context("Missing x coorfinate")??,
                y: coords.next().context("Missing y coorfinate")??,
                z: coords.next().context("Missing z coorfinate")??,
            })
        })
        .collect();
    println!("{:?}", all_jboxes);

    Ok(())
}
