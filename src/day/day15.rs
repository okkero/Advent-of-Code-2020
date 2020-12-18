use crate::day::{Day, DynSolver, Solver};

use std::collections::HashMap;
use std::io::BufRead;
use std::iter;

use anyhow::{anyhow, Result};

pub const DAY15: Day = Day {
    title: "Rambunctious Recitation",
    solver_from_input,
};

struct Game {
    starting_numbers: Vec<u32>,
    previously_spoken: HashMap<u32, usize>,
}

impl Game {
    fn new(starting_numbers: &[u32]) -> Self {
        let previously_spoken = starting_numbers
            .iter()
            .take(starting_numbers.len() - 1)
            .enumerate()
            .map(|(i, n)| (*n, i))
            .collect::<HashMap<_, _>>();
        Self {
            starting_numbers: starting_numbers.to_vec(),
            previously_spoken,
        }
    }

    fn previously_spoken(&self, previous: u32) -> Option<usize> {
        self.previously_spoken.get(&previous).copied()
    }

    fn simulate(mut self) -> impl Iterator<Item = u32> {
        let mut previous_turn = self.starting_numbers.len() - 1;
        iter::successors(self.starting_numbers.last().copied(), move |previous| {
            let next_number =
                previous_turn - self.previously_spoken(*previous).unwrap_or(previous_turn);
            self.previously_spoken.insert(*previous, previous_turn);
            previous_turn += 1;

            Some(next_number as u32)
        })
    }
}

struct Day15Solver(Vec<u32>);
impl Solver for Day15Solver {
    fn part1(&self) -> Result<String> {
        let game = Game::new(&self.0);
        let result = game
            .simulate()
            .nth(2020 - self.0.len())
            .ok_or(anyhow!("No 2020th number"))?;

        Ok(format!("The 2020th number spoken is {}", result))
    }

    fn part2(&self) -> Result<String> {
        let game = Game::new(&self.0);
        let result = game
            .simulate()
            .nth(30000000 - self.0.len())
            .ok_or(anyhow!("No 30000000th number"))?;

        Ok(format!("The 30000000th number spoken is {}", result))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut s = String::new();
    input.read_to_string(&mut s)?;
    s.pop();
    let starting_numbers = s
        .split(',')
        .map(|num| Ok(num.parse()?))
        .collect::<Result<_>>()?;
    Ok(Box::new(Day15Solver(starting_numbers)))
}
