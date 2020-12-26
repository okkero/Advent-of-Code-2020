use crate::day::{Day, DynSolver, Solver};

use std::collections::HashMap;
use std::io::{self, BufRead};

use anyhow::{anyhow, bail, Result};

pub const DAY20: Day = Day {
    title: "Jurassic Jigsaw",
    solver_from_input,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Pixel {
    On,
    Off,
}

#[derive(Debug)]
struct Tile {
    id: u32,
    edges: Vec<Vec<Pixel>>,
}

struct Day20Solver(Vec<Tile>);
impl Solver for Day20Solver {
    fn part1(&self) -> Result<String> {
        let mut matches = HashMap::new();
        for tile in &self.0 {
            for edge in &tile.edges {
                matches.entry(edge.clone()).or_insert(vec![]).push(tile.id);
                matches
                    .entry({
                        let mut v = edge.clone();
                        v.reverse();
                        v
                    })
                    .or_insert(vec![])
                    .push(tile.id);
            }
        }
        let mut frequencies = HashMap::new();
        for id in matches
            .into_iter()
            .filter(|(_, v)| v.len() == 2)
            .flat_map(|(_, v)| v)
        {
            *frequencies.entry(id).or_insert(0) += 1;
        }
        let result: u64 = frequencies
            .into_iter()
            .filter(|(_, count)| *count == 4)
            .map(|(id, _)| id as u64)
            .product();

        Ok(format!("Product of corner IDs: {}", result))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut tiles = Vec::new();
    loop {
        let mut lines = input.lines().peekable();
        if let None = lines.peek() {
            break;
        }

        let tile = parse_tile(lines)?;
        tiles.push(tile);
    }

    Ok(Box::new(Day20Solver(tiles)))
}

fn parse_tile(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<Tile> {
    let header = lines.next().ok_or(anyhow!("No header"))??;
    let id = header[5..(header.len() - 1)].parse()?;
    let image_data = lines
        .take_while(|line| {
            if let Ok("") = line.as_ref().map(|s| s.as_str()) {
                false
            } else {
                true
            }
        })
        .map(|line| {
            Ok(line?
                .chars()
                .map(|c| if c == '#' { Pixel::On } else { Pixel::Off })
                .collect::<Vec<_>>())
        })
        .collect::<Result<Vec<_>>>()?;

    let top = image_data[0].clone();
    let left = image_data
        .iter()
        .map(|row| &row[0])
        .rev()
        .cloned()
        .collect();
    let right = image_data
        .iter()
        .map(|row| &row[row.len() - 1])
        .cloned()
        .collect();
    let bottom = image_data[image_data.len() - 1]
        .iter()
        .rev()
        .cloned()
        .collect();

    let edges = vec![top, left, right, bottom];

    Ok(Tile { id, edges })
}
