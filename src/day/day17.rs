use crate::day::{Day, DynSolver, Solver};

use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufRead;
use std::mem;

use anyhow::Result;
use itertools::iproduct;

pub const DAY17: Day = Day {
    title: "Conway Cubes",
    solver_from_input,
};

trait Coords: Sized {
    fn neighbors(&self) -> Vec<Self>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coords3(isize, isize, isize);
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coords4(isize, isize, isize, isize);

impl Coords for Coords3 {
    fn neighbors(&self) -> Vec<Self> {
        let Self(x, y, z) = self;
        iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1), (z - 1)..=(z + 1))
            .map(|(x, y, z)| Self(x, y, z))
            .filter(|c| c != self)
            .collect()
    }
}

impl Coords for Coords4 {
    fn neighbors(&self) -> Vec<Self> {
        let Self(x, y, z, w) = self;
        iproduct!(
            (x - 1)..=(x + 1),
            (y - 1)..=(y + 1),
            (z - 1)..=(z + 1),
            (w - 1)..=(w + 1)
        )
        .map(|(x, y, z, w)| Self(x, y, z, w))
        .filter(|c| c != self)
        .collect()
    }
}

struct PocketUniverse<C> {
    active_cubes: HashSet<C>,
}

impl<C> PocketUniverse<C>
where
    C: Coords,
    C: Copy + Eq + Hash,
{
    fn step(&mut self) {
        let mut new_cubes = HashSet::new();

        for coords in &self.active_cubes {
            self.check(*coords, &mut new_cubes, true);
        }
        mem::swap(&mut new_cubes, &mut self.active_cubes);
    }

    fn check(&self, coords: C, new_cubes: &mut HashSet<C>, check_neighbors: bool) {
        let neighbors = coords.neighbors();
        let active_neighbors = neighbors
            .iter()
            .filter(|coords| self.active_cubes.contains(coords))
            .count();

        let is_active = self.active_cubes.contains(&coords);
        if is_active && (active_neighbors == 2 || active_neighbors == 3) {
            new_cubes.insert(coords);
        } else if !is_active && active_neighbors == 3 {
            new_cubes.insert(coords);
        }

        if check_neighbors {
            for neighbor_coords in neighbors {
                self.check(neighbor_coords, new_cubes, false);
            }
        }
    }
}

struct Day17Solver(HashSet<(isize, isize)>);
impl Solver for Day17Solver {
    fn part1(&self) -> Result<String> {
        let mut pocket_universe = PocketUniverse {
            active_cubes: self.0.iter().map(|(x, y)| Coords3(*x, *y, 0)).collect(),
        };
        for _ in 0..6 {
            pocket_universe.step();
        }

        let active_cubes = pocket_universe.active_cubes.len();

        Ok(format!("Number of active cubes: {}", active_cubes))
    }

    fn part2(&self) -> Result<String> {
        let mut pocket_universe = PocketUniverse {
            active_cubes: self.0.iter().map(|(x, y)| Coords4(*x, *y, 0, 0)).collect(),
        };
        for _ in 0..6 {
            pocket_universe.step();
        }

        let active_cubes = pocket_universe.active_cubes.len();

        Ok(format!("Number of active cubes: {}", active_cubes))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let lines = input.lines();
    let active_cubes = lines
        .enumerate()
        .filter_map(|(y, line)| Some((y, line.ok()?)))
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
                .collect::<Vec<_>>()
        })
        .collect();
    Ok(Box::new(Day17Solver(active_cubes)))
}
