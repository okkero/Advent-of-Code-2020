use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;
use std::iter;

use anyhow::{anyhow, bail, Result};

pub const DAY25: Day = Day {
    title: "Combo Breaker",
    solver_from_input,
};

fn transform_subject_number(subject_number: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(1), move |transformed| {
        Some((*transformed * subject_number) % 20201227)
    })
}

fn encryption_key(public_key: u64, loop_size: usize) -> u64 {
    transform_subject_number(public_key).nth(loop_size).unwrap()
}

struct Day25Solver {
    door_key: u64,
    card_key: u64,
}
impl Solver for Day25Solver {
    fn part1(&self) -> Result<String> {
        let card_loop_size = transform_subject_number(7)
            .enumerate()
            .skip_while(|(_, transformed)| *transformed != self.card_key)
            .next()
            .unwrap()
            .0;

        let encryption_key = encryption_key(self.door_key, card_loop_size);

        Ok(format!("The encryption key is {}", encryption_key))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut lines = input.lines();
    let door_key = lines.next().ok_or(anyhow!("No door key"))??.parse()?;
    let card_key = lines.next().ok_or(anyhow!("No card key"))??.parse()?;

    Ok(Box::new(Day25Solver { door_key, card_key }))
}
