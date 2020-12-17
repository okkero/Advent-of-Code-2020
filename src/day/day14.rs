use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub const DAY14: Day = Day {
    title: "Docking Data",
    solver_from_input,
};

#[derive(Clone)]
struct Mask {
    and: u64,
    or: u64,
    floating_bits: Vec<u8>,
}

impl Mask {
    fn id() -> Self {
        Self {
            and: u64::max_value(),
            or: u64::min_value(),
            floating_bits: Vec::new(),
        }
    }

    fn apply(&self, value: u64) -> u64 {
        value & self.and | self.or
    }

    fn decode_address(&self, address: u64) -> impl Iterator<Item = u64> + '_ {
        let address = address | self.or;
        self.floating_bits
            .iter()
            .map(|bit| vec![(bit, true), (bit, false)])
            .multi_cartesian_product()
            .map(move |floating_bits| {
                floating_bits.iter().fold(address, |acc, (bit, high)| {
                    if *high {
                        acc | 1 << *bit
                    } else {
                        acc & !(1 << *bit)
                    }
                })
            })
    }
}

impl FromStr for Mask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut floating_bits = Vec::new();
        let len = s.len();
        let (and, or) = s
            .chars()
            .enumerate()
            .try_fold((0, 0), |(and, or), (i, c)| {
                let mask = 1 << len - i - 1;
                match c {
                    'X' => {
                        floating_bits.push((len - i - 1) as u8);
                        Ok((and | mask, or))
                    }
                    '1' => Ok((and | mask, or | mask)),
                    '0' => Ok((and, or)),
                    _ => Err(anyhow!("Invalid mask character '{}'", c)),
                }
            })?;

        Ok(Mask {
            and,
            or,
            floating_bits,
        })
    }
}

enum Instruction {
    SetMask(Mask),
    Write { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if &s[..7] == "mask = " {
            Ok(Self::SetMask(s[7..].parse()?))
        } else if &s[..4] == "mem[" {
            let address_string = s[4..].chars().take_while(|c| *c != ']').collect::<String>();
            let address = address_string.parse()?;
            let value = s[(8 + address_string.len())..].parse()?;
            Ok(Self::Write { address, value })
        } else {
            Err(anyhow!("Invalid instruction"))
        }
    }
}

struct Program {
    current_mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Program {
    fn new() -> Self {
        Self {
            current_mask: Mask::id(),
            memory: HashMap::new(),
        }
    }

    fn run_instruction_v1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::SetMask(mask) => self.current_mask = mask.clone(),
            Instruction::Write { address, value } => {
                self.memory
                    .insert(*address, self.current_mask.apply(*value));
            }
        }
    }

    fn run_instruction_v2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::SetMask(mask) => self.current_mask = mask.clone(),
            Instruction::Write { address, value } => {
                for address in self.current_mask.decode_address(*address) {
                    self.memory.insert(address, *value);
                }
            }
        }
    }
}

struct Day14Solver(Vec<Instruction>);
impl Solver for Day14Solver {
    fn part1(&self) -> Result<String> {
        let mut program = Program::new();
        for instruction in &self.0 {
            program.run_instruction_v1(instruction);
        }

        let sum: u64 = program.memory.values().sum();

        Ok(format!("Sum of all values in memory: {}", sum))
    }

    fn part2(&self) -> Result<String> {
        let mut program = Program::new();
        for instruction in &self.0 {
            program.run_instruction_v2(instruction);
        }

        let sum: u64 = program.memory.values().sum();

        Ok(format!("Sum of all values in memory: {}", sum))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let instructions = input
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<_>>()?;
    Ok(Box::new(Day14Solver(instructions)))
}
