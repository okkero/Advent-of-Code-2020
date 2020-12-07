use crate::day::{Day, DynSolver, Solver};

use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};
use itertools::iproduct;

pub const DAY1: Day = Day {
    title: "Report Repair",
    solver_from_input,
};

struct Day1Solver(Vec<u32>);
impl Solver for Day1Solver {
    fn part1(&self) -> Result<String> {
        let numbers = &self.0;
        let mut low = 0usize;
        let mut high = numbers.len() - 1;

        while low < high {
            let n1 = numbers[low];
            let n2 = numbers[high];
            let sum = n1 + n2;
            if sum < 2020 {
                low += 1;
            } else if sum > 2020 {
                high -= 1;
            } else {
                return Ok(format!("Solution: {}", n1 * n2));
            }
        }

        bail!("Could not find a solution")
    }

    fn part2(&self) -> Result<String> {
        let numbers = &self.0;
        let indices = 0..numbers.len();
        for (a, b, c) in iproduct!(indices.clone(), indices.clone(), indices) {
            if numbers[a] + numbers[b] + numbers[c] == 2020 {
                return Ok(format!(
                    "Solution: {}",
                    numbers[a] * numbers[b] * numbers[c]
                ));
            }
        }

        bail!("Could not find a solution")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut numbers = BufReader::new(input)
        .lines()
        .map(|line| -> Result<u32> { Ok(line?.parse()?) })
        .collect::<Result<Vec<_>, _>>()?;
    numbers.sort();
    Ok(Box::new(Day1Solver(numbers)))
}
