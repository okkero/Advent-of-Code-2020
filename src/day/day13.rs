use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;

use anyhow::{anyhow, bail, Result};

pub const DAY13: Day = Day {
    title: "Shuttle Search",
    solver_from_input,
};

struct Day13Solver {
    estimated_arrival: u32,
    bus_ids: Vec<u32>,
}
impl Solver for Day13Solver {
    fn part1(&self) -> Result<String> {
        let (earliest_id, time_to_departure) = self
            .bus_ids
            .iter()
            .map(|id| (id, *id - (self.estimated_arrival % *id)))
            .min_by_key(|(_, time_to_departure)| *time_to_departure)
            .ok_or(anyhow!("No earliest bus"))?;

        Ok(format!("Result {}", earliest_id * time_to_departure))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut lines = input.lines();
    let estimated_arrival = lines.next().ok_or(anyhow!("No first line"))??.parse()?;
    let bus_ids_line = lines.next().ok_or(anyhow!("No second line"))??;
    let bus_ids = bus_ids_line
        .split(',')
        .filter_map(|s| if s == "x" { None } else { s.parse().ok() })
        .collect();
    Ok(Box::new(Day13Solver {
        estimated_arrival,
        bus_ids,
    }))
}
