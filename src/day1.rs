use std::io::{BufRead, BufReader};

use crate::day::{Day, Solution};
use anyhow::{bail, Result};
use itertools::iproduct;

pub const DAY1: Day = Day {
    title: "Report Repair",
    solution: Solution { part1, part2 },
};

fn part1(input: &mut dyn BufRead) -> Result<String> {
    let mut numbers = parse_input(input)?;
    numbers.sort();
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

fn part2(input: &mut dyn BufRead) -> Result<String> {
    let mut numbers = parse_input(input)?;
    numbers.sort();

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

fn parse_input(input: &mut dyn BufRead) -> Result<Vec<u32>> {
    Ok(BufReader::new(input)
        .lines()
        .map(|line| -> Result<u32> { Ok(line?.parse()?) })
        .collect::<Result<_, _>>()?)
}
