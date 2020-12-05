use crate::day::{Day, DynSolver, Solver};
use anyhow::{anyhow, bail, Result};
use std::io::BufRead;

pub const DAY5: Day = Day {
    title: "Binary Boarding",
    solver_from_input,
};

struct Seat {
    row: u32,
    column: u32,
}

struct Day5Solver(Vec<Seat>);
impl Solver for Day5Solver {
    fn part1(&self) -> Result<String> {
        let highest_id = self
            .0
            .iter()
            .map(|seat| seat.row * 8 + seat.column)
            .max()
            .ok_or(anyhow!("No passports"))?;
        Ok(format!("Highest seat ID: {}", highest_id))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let seats = input
        .lines()
        .map(|line| -> Result<Seat> {
            let line = line?;
            let mut row = 0;
            for (i, c) in line[..7].chars().enumerate() {
                if c == 'B' {
                    row |= 1 << (6 - i);
                }
            }
            let mut column = 0;
            for (i, c) in line[7..].chars().enumerate() {
                if c == 'R' {
                    column |= 1 << (2 - i);
                }
            }

            Ok(Seat { row, column })
        })
        .collect::<Result<_>>()?;

    Ok(Box::new(Day5Solver(seats)))
}
