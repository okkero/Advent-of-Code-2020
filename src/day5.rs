use crate::day::{Day, DynSolver, Solver};
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::io::BufRead;

pub const DAY5: Day = Day {
    title: "Binary Boarding",
    solver_from_input,
};

struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

struct Day5Solver(Vec<Seat>);
impl Solver for Day5Solver {
    fn part1(&self) -> Result<String> {
        let highest_id = self
            .0
            .iter()
            .map(Seat::id)
            .max()
            .ok_or(anyhow!("No passports"))?;
        Ok(format!("Highest seat ID: {}", highest_id))
    }

    fn part2(&self) -> Result<String> {
        let sorted_seats = self.0.iter().map(Seat::id).sorted();
        for (a, b) in sorted_seats.tuple_windows() {
            if a != b - 1 {
                return Ok(format!("Your seat ID: {}", a + 1));
            }
        }

        bail!("No seat ID found")
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
