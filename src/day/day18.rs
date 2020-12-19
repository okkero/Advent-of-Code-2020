use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;

use anyhow::{anyhow, bail, Result};
use std::str::FromStr;

pub const DAY18: Day = Day {
    title: "Operation Order",
    solver_from_input,
};

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Val(u32),
}

impl Expr {
    fn evaluate(&self) -> u32 {
        match self {
            Self::Add(a, b) => a.evaluate() + b.evaluate(),
            Self::Mul(a, b) => a.evaluate() * b.evaluate(),
            Self::Val(value) => *value,
        }
    }
}

impl Expr {
    fn parse(s: &str) -> Result<Self> {
        let (rest, expr) = Self::parse_term(&s)?;

        if rest.len() != 0 {
            bail!("Unexpected trailing input");
        }

        Ok(expr)
    }

    fn parse_term(s: &str) -> Result<(&str, Self)> {
        let i = Self::next_token_index(s).ok_or(anyhow!("No more tokens"))?;
        let (s, lhs) = match s.chars().nth(i).unwrap() {
            c @ '0'..='9' => (&s[(i + 1)..], Self::Val(c.to_digit(10).unwrap())),
            '(' => Self::parse_sub(&s[(i + 1)..])?,
            c => bail!("Unexpected '{}'", c),
        };

        if let Some(i) = Self::next_token_index(s) {
            match s.chars().nth(i).unwrap() {
                '+' => {
                    let (rest, rhs) = Self::parse_term(&s[(i + 1)..])?;
                    Ok((rest, Self::Add(Box::new(lhs), Box::new(rhs))))
                }
                '*' => {
                    let (rest, rhs) = Self::parse_term(&s[(i + 1)..])?;
                    Ok((rest, Self::Mul(Box::new(lhs), Box::new(rhs))))
                }
                ')' => Ok((&s[i..], lhs)),
                c => bail!("Unexpected '{}'", c),
            }
        } else {
            Ok((&s[s.len()..], lhs))
        }
    }

    fn parse_sub(s: &str) -> Result<(&str, Self)> {
        let (s, term) = Self::parse_term(&s)?;

        if let Some(')') = s.chars().next() {
            Ok((&s[1..], term))
        } else {
            bail!("Expected end of sub-expression ')'")
        }
    }

    fn next_token_index(s: &str) -> Option<usize> {
        s.find(|c| c != ' ')
    }
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

struct Day18Solver;
impl Solver for Day18Solver {
    fn part1(&self) -> Result<String> {
        bail!("Unimplemented")
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    Ok(Box::new(Day18Solver))
}
