use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use std::iter;

pub const DAY19: Day = Day {
    title: "Monster Messages",
    solver_from_input,
};

#[derive(Clone, Copy, PartialEq)]
enum Character {
    A,
    B,
}

#[derive(Clone)]
enum Rule {
    Literal(Character),
    // Match one list out of the supplied lists of rules
    OneOf(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "\"a\"" => Ok(Rule::Literal(Character::A)),
            "\"b\"" => Ok(Rule::Literal(Character::B)),
            _ => {
                let lists = s
                    .split(" | ")
                    .map(|list_part| list_part.split(' ').map(|n| Ok(n.parse()?)).collect())
                    .collect::<Result<_>>()?;

                Ok(Rule::OneOf(lists))
            }
        }
    }
}

struct Message(Vec<Character>);

impl Message {
    fn matches(&self, rule: &Rule, rules: &[Rule]) -> bool {
        Self::matches_impl(&self.0, rule, rules)
            .filter(|m| m.is_empty())
            .next()
            .is_some()
    }

    // TODO reduce allocations
    fn matches_impl<'a>(
        message: &'a [Character],
        rule: &'a Rule,
        rules: &'a [Rule],
    ) -> Box<dyn Iterator<Item = &'a [Character]> + 'a> {
        match rule {
            Rule::OneOf(lists) => Box::new(lists.iter().flat_map(move |list| {
                let mut messages: Box<dyn Iterator<Item = &'a [Character]> + 'a> =
                    Box::new(iter::once(message));
                for index in list {
                    let rule = &rules[*index];
                    messages = Box::new(
                        messages.flat_map(move |message| Self::matches_impl(message, rule, rules)),
                    );
                }
                messages
            })),
            Rule::Literal(c) => {
                if let Some(first) = message.first() {
                    if first == c {
                        Box::new(iter::once(&message[1..]))
                    } else {
                        Box::new(iter::empty())
                    }
                } else {
                    Box::new(iter::empty())
                }
            }
        }
    }
}

struct Day19Solver {
    rules: Vec<Rule>,
    messages: Vec<Message>,
}
impl Solver for Day19Solver {
    fn part1(&self) -> Result<String> {
        let rule0 = &self.rules[0];
        let match_count = self
            .messages
            .iter()
            .filter(|message| message.matches(rule0, &self.rules))
            .count();

        Ok(format!(
            "Amount of messages matching rule 0: {}",
            match_count
        ))
    }

    fn part2(&self) -> Result<String> {
        let mut rules = self.rules.clone();
        rules[8] = Rule::OneOf(vec![vec![42, 8], vec![42]]);
        rules[11] = Rule::OneOf(vec![vec![42, 31], vec![42, 11, 31]]);

        let rule0 = &rules[0];
        let match_count = self
            .messages
            .iter()
            .filter(|message| message.matches(rule0, &rules))
            .count();

        Ok(format!(
            "Amount of messages matching rule 0: {}",
            match_count
        ))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let rules = {
        let mut rules = Vec::new();
        for line in input.lines() {
            let line = line?;
            if line == "" {
                break;
            }

            let index_string = line.chars().take_while(|c| *c != ':').collect::<String>();
            let index = index_string.parse::<usize>()?;
            let rule = line[(index_string.len() + 2)..].parse::<Rule>()?;

            rules.resize_with(rules.len().max(index + 1), || Rule::OneOf(vec![]));
            rules[index] = rule;
        }

        rules
    };

    let messages = input
        .lines()
        .map(|line| {
            let line = line?;
            let m = line
                .chars()
                .map(|c| match c {
                    'a' => Ok(Character::A),
                    'b' => Ok(Character::B),
                    _ => Err(anyhow!("Invalid character '{}'", c)),
                })
                .collect::<Result<_>>()?;

            Ok(Message(m))
        })
        .collect::<Result<_>>()?;
    Ok(Box::new(Day19Solver { rules, messages }))
}
