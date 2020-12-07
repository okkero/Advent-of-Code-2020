use crate::day::{Day, DynSolver, Solver};

use std::collections::HashMap;
use std::io::BufRead;
use std::iter;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

pub const DAY4: Day = Day {
    title: "Passport Processing",
    solver_from_input,
};

enum HeightUnit {
    In,
    Cm,
}

struct Height {
    value: u32,
    unit: HeightUnit,
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_digit = |c: &char| *c >= '0' && *c <= '9';
        let value = s.chars().take_while(is_digit).collect::<String>().parse()?;
        let unit = match s.chars().skip_while(is_digit).collect::<String>().as_str() {
            "in" => HeightUnit::In,
            "cm" => HeightUnit::Cm,
            _ => bail!("Invalid unit"),
        };

        Ok(Height { value, unit })
    }
}

struct Day4Solver(Vec<HashMap<String, String>>);
impl Solver for Day4Solver {
    fn part1(&self) -> Result<String> {
        const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let valid_count = self
            .0
            .iter()
            .filter(|passport| {
                REQUIRED_FIELDS
                    .iter()
                    .all(|key| passport.contains_key(*key))
            })
            .count();
        Ok(format!("Valid passports: {}", valid_count))
    }

    fn part2(&self) -> Result<String> {
        fn validate_passport(passport: &HashMap<String, String>) -> Option<()> {
            let birth_year = passport.get("byr")?.parse::<u32>().ok()?;
            let issue_year = passport.get("iyr")?.parse::<u32>().ok()?;
            let expires_year = passport.get("eyr")?.parse::<u32>().ok()?;
            let height = passport.get("hgt")?.parse::<Height>().ok()?;
            let hair_color = passport.get("hcl")?;
            let eye_color = passport.get("ecl")?;
            let id = passport.get("pid")?;

            if birth_year < 1920 || birth_year > 2002 {
                return None;
            }
            if issue_year < 2010 || issue_year > 2020 {
                return None;
            }
            if expires_year < 2020 || expires_year > 2030 {
                return None;
            }
            match height.unit {
                HeightUnit::In => {
                    if height.value < 59 || height.value > 76 {
                        return None;
                    }
                }
                HeightUnit::Cm => {
                    if height.value < 150 || height.value > 193 {
                        return None;
                    }
                }
            }
            if hair_color.chars().next()? != '#' {
                return None;
            }
            if hair_color
                .chars()
                .skip(1)
                .any(|c| (c < '0' || c > '9') && (c < 'a' || c > 'f'))
            {
                return None;
            }
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color.as_str()) {
                return None;
            }
            if id.len() != 9 || id.chars().any(|c| c < '0' || c > '9') {
                return None;
            }

            Some(())
        }

        let valid_count = self
            .0
            .iter()
            .filter(|passport| validate_passport(&passport).is_some())
            .count();

        Ok(format!("Valid passports: {}", valid_count))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut passports = Vec::new();
    let mut current_passport = HashMap::new();
    for line in input.lines().chain(iter::once(Ok("".to_string()))) {
        let line = line?;
        if line.is_empty() {
            passports.push(current_passport);
            current_passport = HashMap::new();
            continue;
        }

        let entries = line
            .split(' ')
            .map(|entry| entry.split(':').collect_tuple());
        for entry in entries {
            let (key, value) = entry.ok_or(anyhow!("Invalid entry format"))?;
            current_passport.insert(key.to_string(), value.to_string());
        }
    }

    Ok(Box::new(Day4Solver(passports)))
}
