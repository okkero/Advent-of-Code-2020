use crate::day::{Day, DynSolver, Solver};

use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

pub const DAY8: Day = Day {
    title: "Handheld Halting",
    solver_from_input,
};

#[derive(Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    fn flip(&mut self) {
        *self = match self {
            Self::Nop(arg) => Self::Jmp(*arg),
            Self::Acc(arg) => Self::Acc(*arg),
            Self::Jmp(arg) => Self::Nop(*arg),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut words = s.split(' ');
        let op = words.next().ok_or(anyhow!("No operation"))?;
        let arg = words.next().ok_or(anyhow!("No argument"))?;

        let arg = arg.parse()?;
        match op {
            "nop" => Ok(Self::Nop(arg)),
            "acc" => Ok(Self::Acc(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            _ => bail!("Illegal operation"),
        }
    }
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>,
}

struct Process<'a> {
    program: &'a Program,
    accumulator: i32,
    instruction_pointer: usize,
    processed_instructions: HashSet<usize>,
}

enum ProgramResult {
    InfiniteLoop,
    Terminated,
}

impl<'a> Process<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            program,
            accumulator: 0,
            instruction_pointer: 0,
            processed_instructions: HashSet::new(),
        }
    }

    /// Run until the program enters an infinite loop, or terminates.
    fn run(&mut self) -> ProgramResult {
        loop {
            self.step();

            if self
                .processed_instructions
                .contains(&self.instruction_pointer)
            {
                break ProgramResult::InfiniteLoop;
            }

            if self.instruction_pointer == self.program.instructions.len() {
                break ProgramResult::Terminated;
            }
        }
    }

    fn step(&mut self) {
        let instruction = self.program.instructions[self.instruction_pointer];
        self.processed_instructions.insert(self.instruction_pointer);
        match instruction {
            Instruction::Nop(_) => self.instruction_pointer += 1,
            Instruction::Acc(amount) => {
                self.accumulator += amount;
                self.instruction_pointer += 1;
            }
            Instruction::Jmp(offset) => self.offset_instruction_pointer(offset),
        }
    }

    fn offset_instruction_pointer(&mut self, offset: i32) {
        self.instruction_pointer = self.instruction_pointer.wrapping_add(offset as usize);
    }
}

struct Day8Solver(Program);
impl Solver for Day8Solver {
    fn part1(&self) -> Result<String> {
        let mut process = Process::new(&self.0);
        let program_result = process.run();

        if let ProgramResult::InfiniteLoop = program_result {
            Ok(format!(
                "Infinite loop detected. Accumulator: {}",
                process.accumulator
            ))
        } else {
            bail!("Program did not result in infinite loop")
        }
    }

    fn part2(&self) -> Result<String> {
        let mut program = self.0.clone();

        for i in 0..program.instructions.len() {
            program.instructions[i].flip();

            let mut process = Process::new(&program);
            if let ProgramResult::Terminated = process.run() {
                return Ok(format!(
                    "Program terminated! Accumulator: {}",
                    process.accumulator
                ));
            }

            program.instructions[i].flip();
        }

        bail!("Program did not terminate")
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let instructions = input
        .lines()
        .map(|line| -> Result<Instruction> { line?.parse() })
        .collect::<Result<_>>()?;
    Ok(Box::new(Day8Solver(Program { instructions })))
}
