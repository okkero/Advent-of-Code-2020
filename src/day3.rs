use anyhow::{anyhow, Result};
use std::io::BufRead;

use crate::day::{Day, Solution};
use std::iter;

pub const DAY3: Day = Day {
    title: "Toboggan Trajectory",
    solution: Solution { part1, part2 },
};

struct Forest {
    map: Vec<bool>,
    width: usize,
}

impl Forest {
    fn tree_at(&self, x: usize, y: usize) -> Option<bool> {
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
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let tree = self.forest.tree_at(self.x % self.forest.width, self.y)?;
        self.x += self.right;
        self.y += self.down;
        Some(tree)
    }
}

fn part1(input: &mut dyn BufRead) -> Result<String> {
    let forest = parse_input(input)?;
    let toboggan = Toboggan::new(&forest, 3, 1);
    let trees = toboggan.filter(|tree| *tree).count();

    Ok(format!("Trees hit: {}", trees))
}

fn part2(input: &mut dyn BufRead) -> Result<String> {
    let forest = parse_input(input)?;
    let mut sleds = [
        Toboggan::new(&forest, 1, 1),
        Toboggan::new(&forest, 3, 1),
        Toboggan::new(&forest, 5, 1),
        Toboggan::new(&forest, 7, 1),
        Toboggan::new(&forest, 1, 2),
    ];
    let trees: usize = sleds
        .iter_mut()
        .map(|toboggan| toboggan.filter(|tree| *tree).count())
        .product();

    Ok(format!("Trees hit: {}", trees))
}

fn parse_input(input: &mut dyn BufRead) -> Result<Forest> {
    let mut lines = input.lines().filter_map(|r| r.ok());
    let first_line = lines.next().ok_or(anyhow!("Input is empty"))?;
    let width = first_line.len();
    let map = iter::once(first_line)
        .chain(lines)
        .flat_map(|line| line.chars().collect::<Vec<_>>().into_iter())
        .map(|c| c == '#')
        .collect();
    Ok(Forest { map, width })
}
