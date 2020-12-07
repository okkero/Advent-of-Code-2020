use anyhow::{anyhow, Result};
use std::io::BufRead;

use crate::day::{Day, DynSolver, Solver};
use std::iter;

pub const DAY3: Day = Day {
    title: "Toboggan Trajectory",
    solver_from_input,
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Tree,
    Empty,
}

struct Forest {
    map: Vec<Tile>,
    width: usize,
}

impl Forest {
    fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        let index = y * self.width + x;
        self.map.get(index).copied()
    }
}

struct Toboggan<'a> {
    forest: &'a Forest,
    right: usize,
    down: usize,
    x: usize,
    y: usize,
}

impl<'a> Toboggan<'a> {
    fn new(forest: &'a Forest, right: usize, down: usize) -> Self {
        Self {
            forest,
            right,
            down,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for Toboggan<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.forest.tile_at(self.x % self.forest.width, self.y)?;
        self.x += self.right;
        self.y += self.down;
        Some(tile)
    }
}

struct Day3Solver(Forest);
impl Solver for Day3Solver {
    fn part1(&self) -> Result<String> {
        let forest = &self.0;
        let toboggan = Toboggan::new(forest, 3, 1);
        let trees = toboggan.filter(|tile| *tile == Tile::Tree).count();

        Ok(format!("Trees hit: {}", trees))
    }

    fn part2(&self) -> Result<String> {
        let forest = &self.0;
        let mut sleds = [
            Toboggan::new(forest, 1, 1),
            Toboggan::new(forest, 3, 1),
            Toboggan::new(forest, 5, 1),
            Toboggan::new(forest, 7, 1),
            Toboggan::new(forest, 1, 2),
        ];
        let trees: usize = sleds
            .iter_mut()
            .map(|toboggan| toboggan.filter(|tile| *tile == Tile::Tree).count())
            .product();

        Ok(format!("Trees hit: {}", trees))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut lines = input.lines().filter_map(|r| r.ok());
    let first_line = lines.next().ok_or(anyhow!("Input is empty"))?;
    let width = first_line.len();
    let map = iter::once(first_line)
        .chain(lines)
        .flat_map(|line| line.chars().collect::<Vec<_>>().into_iter())
        .map(|c| if c == '#' { Tile::Tree } else { Tile::Empty })
        .collect();
    Ok(Box::new(Day3Solver(Forest { map, width })))
}
