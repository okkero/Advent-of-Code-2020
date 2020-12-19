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
    Add(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Val(u64),
}

impl Expr {
    fn evaluate(&self) -> u64 {
        match self {
            Self::Add(a, b) => a.evaluate() + b.evaluate(),
            Self::Mul(a, b) => a.evaluate() * b.evaluate(),
            Self::Val(value) => *value,
        }
    }
}

impl Expr {
    fn parse(s: &str) -> Result<Self> {
        let (rest, expr) = Self::parse_term(&s, None)?;

        if rest.len() != 0 {
            bail!("Unexpected trailing input");
        }

        Ok(expr)
    }

    fn parse_term(s: &str, lhs: Option<Self>) -> Result<(&str, Self)> {
        let (s, lhs) = if let Some(lhs) = lhs {
            (s, lhs)
        } else {
            Self::parse_next_single(s)?
        };

        if let Some(i) = Self::next_token_index(s) {
            match s.chars().nth(i).unwrap() {
                '+' => {
                    let (rest, rhs) = Self::parse_next_single(&s[(i + 1)..])?;
                    let expr = Self::Add(Box::new(lhs), Box::new(rhs));
                    Self::parse_term(rest, Some(expr))
                }
                '*' => {
                    let (rest, rhs) = Self::parse_next_single(&s[(i + 1)..])?;
                    let expr = Self::Mul(Box::new(lhs), Box::new(rhs));
                    Self::parse_term(rest, Some(expr))
                }
                ')' => Ok((&s[i..], lhs)),
                c => bail!("Unexpected '{}'", c),
            }
        } else {
            Ok((&s[s.len()..], lhs))
        }
    }

    fn parse_sub(s: &str) -> Result<(&str, Self)> {
        let (s, term) = Self::parse_term(&s, None)?;

        if let Some(')') = s.chars().next() {
            Ok((&s[1..], term))
        } else {
            bail!("Expected end of sub-expression ')'")
        }
    }

    fn parse_next_single(s: &str) -> Result<(&str, Expr)> {
        let i = Self::next_token_index(s).ok_or(anyhow!("No more tokens"))?;
        match s.chars().nth(i).unwrap() {
            '(' => Self::parse_sub(&s[(i + 1)..]),
            c @ '0'..='9' => Ok((&s[(i + 1)..], Self::Val(c.to_digit(10).unwrap() as u64))),
            c => bail!("Unexpected '{}'", c),
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

struct Day18Solver(Vec<Expr>);
impl Solver for Day18Solver {
    fn part1(&self) -> Result<String> {
        let sum: u64 = self.0.iter().map(|expr| expr.evaluate()).sum();

        Ok(format!("Sum of all expressions: {}", sum))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let expressions = input
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<_>>()?;
    Ok(Box::new(Day18Solver(expressions)))
}
