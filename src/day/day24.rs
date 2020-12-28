use crate::day::{Day, DynSolver, Solver};

use std::collections::HashSet;
use std::io::BufRead;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

pub const DAY24: Day = Day {
    title: "Lobby Layout",
    solver_from_input,
};

#[derive(Clone, Copy)]
enum Direction {
    W,
    E,
    NW,
    NE,
    SW,
    SE,
}

fn setup(instructions: &Vec<Vec<Direction>>) -> HashSet<(i32, i32)> {
    instructions
        .iter()
        .map(|steps| {
            steps
                .iter()
                .fold((0, 0), |coords, dir| adjacent(coords, *dir))
        })
        .counts()
        .into_iter()
        .filter(|(_, count)| count % 2 == 1)
        .map(|(coords, _)| coords)
        .collect()
}

fn adjacent((x, y): (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::W => (x - 1, y),
        Direction::E => (x + 1, y),
        Direction::NW => (x, y + 1),
        Direction::NE => (x + 1, y + 1),
        Direction::SW => (x - 1, y - 1),
        Direction::SE => (x, y - 1),
    }
}

fn adjacents(coords: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    use Direction::*;

    [W, E, NW, NE, SW, SE]
        .iter()
        .map(move |dir| adjacent(coords, *dir))
}

struct Day24Solver(Vec<Vec<Direction>>);
impl Solver for Day24Solver {
    fn part1(&self) -> Result<String> {
        let black_count = setup(&self.0).len();

        Ok(format!("Amount of flipped tiles: {}", black_count))
    }

    fn part2(&self) -> Result<String> {
        let mut tiles = setup(&self.0);

        for _ in 0..100 {
            let mut visited_white = Vec::new();
            let mut new_tiles = tiles
                .iter()
                .copied()
                .filter(|coords| {
                    let (adjacent_black, adjacent_white): (Vec<_>, _) =
                        adjacents(*coords).partition(|adjacent| tiles.contains(&adjacent));
                    let adjacent_black_count = adjacent_black.len();
                    visited_white.extend(adjacent_white);

                    adjacent_black_count == 1 || adjacent_black_count == 2
                })
                .collect::<HashSet<_>>();
            new_tiles.extend(
                visited_white
                    .into_iter()
                    .counts()
                    .into_iter()
                    .filter(|(_, count)| *count == 2)
                    .map(|(coords, _)| coords),
            );

            tiles = new_tiles;
        }

        let black_count = tiles.len();

        Ok(format!("Amount of black tiles: {}", black_count))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let paths = input
        .lines()
        .map(|line| {
            let line = line?;
            let mut chars = line.chars();

            let mut dirs = Vec::new();
            while let Some(c) = chars.next() {
                let dir = match c {
                    'w' => Direction::W,
                    'e' => Direction::E,
                    'n' => match chars.next().ok_or(anyhow!("Expected char after 'n'"))? {
                        'w' => Direction::NW,
                        'e' => Direction::NE,
                        c => bail!("Unexpected '{}' after 'n'", c),
                    },
                    's' => match chars.next().ok_or(anyhow!("Expected char after 's'"))? {
                        'w' => Direction::SW,
                        'e' => Direction::SE,
                        c => bail!("Unexpected '{}' after 's'", c),
                    },
                    c => bail!("Unexpected '{}'", c),
                };
                dirs.push(dir);
            }

            Ok(dirs)
        })
        .collect::<Result<_>>()?;

    Ok(Box::new(Day24Solver(paths)))
}
