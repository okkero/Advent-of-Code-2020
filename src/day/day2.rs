use anyhow::{anyhow, Result};
use std::io::BufRead;

use crate::day::{Day, DynSolver, Solver};

pub const DAY2: Day = Day {
    title: "Password Philosophy",
    solver_from_input,
};

struct Policy {
    character: char,
    least: usize,
    most: usize,
}

struct Password {
    policy: Policy,
    password: String,
}

struct Day2Solver(Vec<Password>);
impl Solver for Day2Solver {
    fn part1(&self) -> Result<String> {
        let passwords = &self.0;
        let matches = passwords
            .into_iter()
            .filter(
                |Password {
                     policy,
                     password: pw,
                 }| {
                    let count = pw.chars().filter(|c| *c == policy.character).count();
                    count >= policy.least && count <= policy.most
                },
            )
            .count();

        Ok(format!("Matches: {}", matches))
    }

    fn part2(&self) -> Result<String> {
        let passwords = &self.0;
        let matches = passwords
            .into_iter()
            .filter(
                |Password {
                     policy,
                     password: pw,
                 }| {
                    let mut chars = pw.chars();
                    let c1 = chars.nth(policy.least - 1);
                    let c2 = chars.nth(policy.most - policy.least - 1);
                    (c1 == Some(policy.character)) ^ (c2 == Some(policy.character))
                },
            )
            .count();

        Ok(format!("Matches: {}", matches))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    Ok(Box::new(Day2Solver(
        input
            .lines()
            .map(|line| -> Result<Password> {
                let line = line?;
                let mut parts = line.split(": ");

                let policy_string = parts.next().ok_or(anyhow!("No policy"))?;
                let mut policy_parts = policy_string.split(" ");
                let range_string = policy_parts.next().ok_or(anyhow!("No policy range"))?;
                let mut range_parts = range_string.split("-");
                let least = range_parts
                    .next()
                    .ok_or(anyhow!("Invalid range"))?
                    .parse()?;
                let most = range_parts
                    .next()
                    .ok_or(anyhow!("Invalid range"))?
                    .parse()?;
                let character = policy_parts
                    .next()
                    .ok_or(anyhow!("No policy char"))?
                    .chars()
                    .next()
                    .ok_or(anyhow!("Invalid policy char"))?;

                let password = parts.next().ok_or(anyhow!("No password"))?.to_string();

                Ok(Password {
                    policy: Policy {
                        character,
                        least,
                        most,
                    },
                    password,
                })
            })
            .collect::<Result<_, _>>()?,
    )))
}
