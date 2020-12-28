use crate::day::{Day, DynSolver, Solver};

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

struct Day24Solver(Vec<Vec<Direction>>);
impl Solver for Day24Solver {
    fn part1(&self) -> Result<String> {
        let flipped = self
            .0
            .iter()
            .map(|steps| steps.iter().fold((0, 0), |coords, dir| adjacent(coords, *dir)))
            .counts()
            .into_iter()
            .filter(|(_, count)| count % 2 == 1)
            .count();

        Ok(format!("Amount of flipped tiles: {}", flipped))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
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
