use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;

use anyhow::{anyhow, Result};
use itertools::Itertools;

const PREAMBLE_LENGTH: usize = 25;

pub const DAY9: Day = Day {
    title: "Encoding Error",
    solver_from_input,
};

struct Day9Solver(Vec<u64>);
impl Solver for Day9Solver {
    fn part1(&self) -> Result<String> {
        let invalid_number = first_invalid_number(&self.0);
        Ok(format!(
            "First invalid number: {}",
            invalid_number.ok_or(anyhow!("No solution"))?
        ))
    }

    fn part2(&self) -> Result<String> {
        let numbers = &self.0;
        let invalid_number = first_invalid_number(numbers).ok_or(anyhow!("No invalid number"))?;

        let weakness = (0..numbers.len())
            .flat_map(|start| ((start + 2)..numbers.len()).map(move |end| &numbers[start..end]))
            .filter(|range| range.iter().sum::<u64>() == invalid_number)
            .flat_map(|range| range.iter().minmax().into_option().into_iter())
            .map(|(min, max)| min + max)
            .next()
            .ok_or(anyhow!("No weakness"))?;

        Ok(format!("Weakness: {}", weakness))
    }
}

fn first_invalid_number(numbers: &[u64]) -> Option<u64> {
    Some(
        (0..)
            .map(|start| {
                let end = start + PREAMBLE_LENGTH;
                (
                    numbers[end],
                    (start..end)
                        .cartesian_product(start..end)
                        .filter(|(a, b)| a != b)
                        .any(|(a, b)| numbers[a] + numbers[b] == numbers[end]),
                )
            })
            .find(|(_, valid)| !*valid)?
            .0,
    )
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let numbers = input
        .lines()
        .map(|line| -> Result<u64> { Ok(line?.parse()?) })
        .collect::<Result<_>>()?;
    Ok(Box::new(Day9Solver(numbers)))
}
