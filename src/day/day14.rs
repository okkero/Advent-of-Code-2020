use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

pub const DAY14: Day = Day {
    title: "Docking Data",
    solver_from_input,
};

#[derive(Clone, Copy)]
struct Mask {
    and: u64,
    or: u64,
}

impl Mask {
    fn id() -> Self {
        Self {
            and: u64::max_value(),
            or: u64::min_value(),
        }
    }

    fn apply(&self, value: u64) -> u64 {
        value & self.and | self.or
    }
}

impl FromStr for Mask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        let (and, or) = s
            .chars()
            .enumerate()
            .try_fold((0, 0), |(and, or), (i, c)| {
                let mask = 1 << len - i - 1;
                match c {
                    'X' => Ok((and | mask, or)),
                    '1' => Ok((and | mask, or | mask)),
                    '0' => Ok((and, or)),
                    _ => Err(anyhow!("Invalid mask character '{}'", c)),
                }
            })?;

        Ok(Mask { and, or })
    }
}

#[derive(Clone, Copy)]
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
    memory: Vec<u64>,
    largest_touched_address: u64,
}

impl Program {
    fn new() -> Self {
        Self {
            current_mask: Mask::id(),
            memory: vec![0; 68719476736], // 36 bit memory space
            largest_touched_address: 0,
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(mask) => self.current_mask = mask,
            Instruction::Write { address, value } => {
                self.memory[address as usize] = self.current_mask.apply(value);
                if address > self.largest_touched_address {
                    self.largest_touched_address = address;
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
            program.run_instruction(*instruction);
        }

        let sum: u64 = program.memory[..=program.largest_touched_address as usize]
            .iter()
            .sum();

        Ok(format!("Sum of all values in memory: {}", sum))
    }

    fn part2(&self) -> Result<String> {
        bail!("Unimplemented")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let instructions = input
        .lines()
        .map(|line| -> Result<Instruction> { Ok(line?.parse()?) })
        .collect::<Result<_>>()?;
    Ok(Box::new(Day14Solver(instructions)))
}
