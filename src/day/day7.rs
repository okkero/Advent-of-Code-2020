use crate::day::{Day, DynSolver, Solver};

use std::collections::HashMap;
use std::io::BufRead;

use anyhow::{anyhow, Result};
use itertools::Itertools;

const SHINY_GOLD: &str = "shiny gold";

pub const DAY7: Day = Day {
    title: "Handy Haversacks",
    solver_from_input,
};

#[derive(Debug)]
struct BagSpec {
    contents: Vec<(u32, String)>,
}

struct Day7Solver(HashMap<String, BagSpec>);
impl Solver for Day7Solver {
    fn part1(&self) -> Result<String> {
        fn walk<'a>(
            root: &'a str,
            specs: &'a HashMap<String, BagSpec>,
            visited: &mut HashMap<&'a str, bool>,
        ) -> bool {
            if let Some(contains) = visited.get(root) {
                return *contains;
            }

            let spec = &specs[root];

            for (_, child) in &spec.contents {
                if child == SHINY_GOLD {
                    visited.insert(root, true);
                    return true;
                }

                if walk(child, specs, visited) {
                    visited.insert(root, true);
                    return true;
                }
            }

            visited.insert(root, false);
            false
        }

        let mut visited = HashMap::new();
        for k in self.0.keys() {
            walk(k, &self.0, &mut visited);
        }

        let usable_colors_count = visited
            .values()
            .filter(|contains_shiny| **contains_shiny)
            .count();

        Ok(format!(
            "Amount of usable bag colors: {}",
            usable_colors_count
        ))
    }

    fn part2(&self) -> Result<String> {
        fn walk<'a>(
            root: &'a str,
            specs: &'a HashMap<String, BagSpec>,
            counted: &mut HashMap<&'a str, u32>,
        ) -> u32 {
            if let Some(count) = counted.get(root) {
                return *count;
            }

            let spec = &specs[root];

            let bag_count = spec
                .contents
                .iter()
                .map(|(amount, child)| amount + amount * walk(child, specs, counted))
                .sum();
            counted.insert(root, bag_count);

            bag_count
        }

        let mut counted = HashMap::new();
        let shiny_gold_content_count = walk(SHINY_GOLD, &self.0, &mut counted);

        Ok(format!(
            "Amount of bags contained in my bag: {}",
            shiny_gold_content_count
        ))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let specs = input
        .lines()
        .map(|line| -> Result<(String, BagSpec)> {
            let line = line?;
            let mut words = line.split(' ');
            let color = parse_color(&mut words).ok_or(anyhow!("Invalid color format"))?;
            words.nth(1).ok_or(anyhow!("Invalid color format"))?;
            let contents = parse_contents(words).ok_or(anyhow!("Invalid contents format"))?;

            Ok((color, BagSpec { contents }))
        })
        .collect::<Result<_>>()?;

    Ok(Box::new(Day7Solver(specs)))
}

fn parse_contents<'a>(mut words: impl Iterator<Item = &'a str>) -> Option<Vec<(u32, String)>> {
    let mut colors = Vec::new();
    while let Some(amount) = words.next() {
        if amount == "no" {
            break;
        }

        colors.push((amount.parse().ok()?, parse_color(&mut words)?));
        words.next();
    }

    Some(colors)
}

fn parse_color<'a>(words: &mut impl Iterator<Item = &'a str>) -> Option<String> {
    let (color1, color2) = words.next_tuple()?;
    Some(format!("{} {}", color1, color2))
}
