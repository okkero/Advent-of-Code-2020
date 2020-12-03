use anyhow::{anyhow, Result};
use std::io::BufRead;

use crate::day::{Day, Solution};

pub const DAY2: Day = Day {
    title: "Password Philosophy",
    solution: Solution { part1, part2 },
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

fn part1(input: &mut dyn BufRead) -> Result<String> {
    let passwords = parse_input(input)?;
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

fn part2(input: &mut dyn BufRead) -> Result<String> {
    let passwords = parse_input(input)?;
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

fn parse_input(input: &mut dyn BufRead) -> Result<Vec<Password>> {
    Ok(input
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
        .collect::<Result<_, _>>()?)
}
