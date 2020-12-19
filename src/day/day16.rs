use crate::day::{Day, DynSolver, Solver};

use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub const DAY16: Day = Day {
    title: "Ticket Translation",
    solver_from_input,
};

struct Ticket(Vec<u32>);

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Ticket(
            s.split(',')
                .map(|s| Ok(s.parse()?))
                .collect::<Result<_>>()?,
        ))
    }
}

struct TicketRules {
    rules: HashMap<String, (RangeInclusive<u32>, RangeInclusive<u32>)>,
}

impl TicketRules {
    fn validate_ticket<'a>(&'a self, ticket: &'a Ticket) -> impl Iterator<Item = u32> + 'a {
        ticket.0.iter().copied().filter(move |value| {
            self.rules
                .values()
                .all(|(range1, range2)| !range1.contains(value) && !range2.contains(value))
        })
    }
}

struct Day16Solver {
    rules: TicketRules,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}
impl Solver for Day16Solver {
    fn part1(&self) -> Result<String> {
        let error_rate: u32 = self
            .nearby_tickets
            .iter()
            .flat_map(|ticket| self.rules.validate_ticket(ticket))
            .sum();

        Ok(format!("Nearby ticket error rate: {}", error_rate))
    }

    fn part2(&self) -> Result<String> {
        let mut field_mapping = vec![self.rules.rules.keys().collect::<HashSet<_>>(); 20];
        let valid_tickets = self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.rules.validate_ticket(ticket).next().is_none());

        for ticket in valid_tickets {
            for (i, value) in ticket.0.iter().enumerate() {
                let invalid_field_names = self
                    .rules
                    .rules
                    .iter()
                    .filter(|(_, (range1, range2))| {
                        !range1.contains(value) && !range2.contains(value)
                    })
                    .map(|(key, _)| key);

                for field_name in invalid_field_names {
                    field_mapping[i].remove(&field_name);
                }
            }
        }

        let mut taken_field_names = HashSet::new();
        let mut field_order = field_mapping
            .iter()
            .enumerate()
            .sorted_by_key(|(_, s)| s.len())
            .map(|(i, possible_field_names)| {
                let field_name = possible_field_names
                    .iter()
                    .filter(|field_name| !taken_field_names.contains(field_name))
                    .exactly_one()
                    .map_err(|_| anyhow!("No single solution"))?;
                taken_field_names.insert(field_name);

                Ok((i, field_name))
            })
            .collect::<Result<Vec<_>>>()?;
        field_order.sort_by_key(|(i, _)| *i);

        let sum: u64 = self
            .my_ticket
            .0
            .iter()
            .enumerate()
            .filter(|(i, _)| field_order[*i].1.starts_with("departure"))
            .map(|(_, value)| *value as u64)
            .product();

        Ok(format!("Product of departure fields: {}", sum))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let rules = input
        .lines()
        .take(20)
        .map(|line| {
            let line = line?;
            let key = line.chars().take_while(|c| *c != ':').collect::<String>();
            let range1_string = line[(key.len() + 2)..]
                .chars()
                .take_while(|c| *c != ' ')
                .collect::<String>();
            let range1 = parse_range(&range1_string)?;
            let range2 = parse_range(&line[(key.len() + range1_string.len() + 6)..])?;

            Ok((key, (range1, range2)))
        })
        .collect::<Result<_>>()?;
    let my_ticket = input
        .lines()
        .nth(2)
        .ok_or(anyhow!("My ticket not found"))??
        .parse()?;
    let nearby_tickets = input
        .lines()
        .skip(2)
        .map(|line| line?.parse())
        .collect::<Result<_>>()?;
    Ok(Box::new(Day16Solver {
        rules: TicketRules { rules },
        my_ticket,
        nearby_tickets,
    }))
}

fn parse_range(s: &str) -> Result<RangeInclusive<u32>> {
    let low_string = s.chars().take_while(|c| *c != '-').collect::<String>();
    let low = low_string.parse()?;
    let high = s[(low_string.len() + 1)..].parse()?;

    Ok(low..=high)
}
