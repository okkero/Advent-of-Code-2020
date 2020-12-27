use crate::day::{Day, DynSolver, Solver};

use std::collections::HashMap;
use std::io::BufRead;
use std::iter;

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub const DAY23: Day = Day {
    title: "Crab Cups",
    solver_from_input,
};

struct Day23Solver(Vec<u32>);
impl Solver for Day23Solver {
    fn part1(&self) -> Result<String> {
        let mut cups = self.0.clone();
        for _ in 0..100 {
            let mut yanked = [0; 3];
            yanked.copy_from_slice(&cups[1..4]);
            cups.drain(1..4);

            let mut destination_label = cups[0] - 1;
            while !cups.contains(&destination_label) {
                destination_label = if destination_label == 0 {
                    9
                } else {
                    destination_label - 1
                }
            }
            let destination_index =
                1 + cups
                    .iter()
                    .position(|n| *n == destination_label)
                    .ok_or(anyhow!(
                        "Unable to find cup with label {}",
                        destination_label,
                    ))?;
            cups.splice(destination_index..destination_index, yanked.iter().copied());
            cups.rotate_left(1);
        }

        let result = cups
            .iter()
            .cycle()
            .skip_while(|n| **n != 1)
            .skip(1)
            .take(cups.len() - 1)
            .join("");

        Ok(format!("Labels after cup 1: {}", result))
    }

    fn part2(&self) -> Result<String> {
        let mut cups = self
            .0
            .iter()
            .copied()
            .chain(10..=1_000_000)
            .chain(iter::once(self.0[0]))
            .tuple_windows()
            .collect::<HashMap<_, _>>();
        let mut current = self.0[0];
        for _ in 0..10_000_000 {
            let a = cups[&current];
            let b = cups[&a];
            let c = cups[&b];
            let yanked = [a, b, c];
            cups.insert(current, cups[&c]);

            let mut destination = current - 1;
            while destination == 0 || yanked.contains(&destination) {
                destination = if destination == 0 {
                    1_000_000
                } else {
                    destination - 1
                }
            }

            let after = cups[&destination];
            cups.insert(destination, a);
            cups.insert(c, after);
            current = cups[&current];
        }

        let star1 = cups[&1];
        let star2 = cups[&star1];
        let result = star1 as u64 * star2 as u64;

        Ok(format!("Product of cups hiding stars: {}", result))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut s = String::new();
    input.read_to_string(&mut s)?;
    s.pop();
    let cups = s
        .chars()
        .map(|c| c.to_digit(10).ok_or(anyhow!("Invalid digit: '{}'", c)))
        .collect::<Result<_>>()?;
    Ok(Box::new(Day23Solver(cups)))
}
