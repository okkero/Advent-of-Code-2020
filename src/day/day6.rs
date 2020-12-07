use crate::day::{Day, DynSolver, Solver};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;
use std::iter;

pub const DAY6: Day = Day {
    title: "Custom Customs",
    solver_from_input,
};

struct Form {
    answers: Vec<char>,
}

struct Group {
    forms: Vec<Form>,
}

impl Group {
    fn form_count(&self) -> usize {
        self.forms.len()
    }
}

struct Day6Solver(Vec<Group>);
impl Solver for Day6Solver {
    fn part1(&self) -> Result<String> {
        let count: usize = self
            .0
            .iter()
            .map(|group| {
                group
                    .forms
                    .iter()
                    .flat_map(|form| &form.answers)
                    .unique()
                    .count()
            })
            .sum();

        Ok(format!("At least one yes: {}", count))
    }

    fn part2(&self) -> Result<String> {
        let count: usize = self
            .0
            .iter()
            .map(|group| {
                let form_count = group.form_count();
                let mut counts = HashMap::new();
                for c in group.forms.iter().flat_map(|form| &form.answers) {
                    counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
                }

                counts.into_iter().filter(|(_, v)| *v == form_count).count()
            })
            .sum();

        Ok(format!("All yeses: {}", count))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for line in input.lines().chain(iter::once(Ok("".to_string()))) {
        let line = line?;
        if line.is_empty() {
            groups.push(Group {
                forms: current_group,
            });
            current_group = Vec::new();
            continue;
        }

        current_group.push(Form {
            answers: line.chars().collect::<Vec<char>>(),
        });
    }

    Ok(Box::new(Day6Solver(groups)))
}
