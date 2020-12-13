use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

use anyhow::{anyhow, Result};

pub const DAY12: Day = Day {
    title: "Rain Risk",
    solver_from_input,
};

enum Operation {
    Direction(Direction),
    Left,
    Right,
    Forward,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "N" => Ok(Self::Direction(Direction::North)),
            "S" => Ok(Self::Direction(Direction::South)),
            "E" => Ok(Self::Direction(Direction::East)),
            "W" => Ok(Self::Direction(Direction::West)),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "F" => Ok(Self::Forward),
            _ => Err(anyhow!("Invalid operation")),
        }
    }
}

struct Instruction {
    operation: Operation,
    argument: u32,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn coord_offsets(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

impl AddAssign<u32> for Direction {
    fn add_assign(&mut self, rhs: u32) {
        let rhs = rhs % 4;
        for _ in 0..rhs {
            *self = self.next();
        }
    }
}

impl SubAssign<u32> for Direction {
    fn sub_assign(&mut self, rhs: u32) {
        let rhs = rhs % 4;
        for _ in 0..rhs {
            *self = self.prev();
        }
    }
}

struct Ferry {
    position: (i32, i32),
    facing: Direction,
}

impl Ferry {
    fn new() -> Self {
        Self {
            position: (0, 0),
            facing: Direction::East,
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let argument = instruction.argument as i32;
        match instruction.operation {
            Operation::Direction(dir) => {
                let (offset_x, offset_y) = dir.coord_offsets();
                self.position.0 += offset_x * argument;
                self.position.1 += offset_y * argument;
            }
            Operation::Left => {
                self.facing -= instruction.argument;
            }
            Operation::Right => {
                self.facing += instruction.argument;
            }
            Operation::Forward => {
                let (offset_x, offset_y) = self.facing.coord_offsets();
                self.position.0 += offset_x * argument;
                self.position.1 += offset_y * argument;
            }
        }
    }
}

struct WaypointFerry {
    position: (i32, i32),
    waypoint_offset: (i32, i32),
}

impl WaypointFerry {
    fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint_offset: (10, 1),
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let argument = instruction.argument as i32;
        match instruction.operation {
            Operation::Direction(dir) => {
                let (offset_x, offset_y) = dir.coord_offsets();
                self.waypoint_offset.0 += offset_x * argument;
                self.waypoint_offset.1 += offset_y * argument;
            }
            Operation::Left => {
                for _ in 0..(instruction.argument % 4) {
                    self.waypoint_offset = (-self.waypoint_offset.1, self.waypoint_offset.0);
                }
            }
            Operation::Right => {
                for _ in 0..(instruction.argument % 4) {
                    self.waypoint_offset = (self.waypoint_offset.1, -self.waypoint_offset.0);
                }
            }
            Operation::Forward => {
                let (offset_x, offset_y) = self.waypoint_offset;
                self.position.0 += offset_x * argument;
                self.position.1 += offset_y * argument;
            }
        }
    }
}

struct Day12Solver(Vec<Instruction>);
impl Solver for Day12Solver {
    fn part1(&self) -> Result<String> {
        let mut ferry = Ferry::new();
        for instruction in &self.0 {
            ferry.run_instruction(instruction);
        }

        let manhattan_distance = ferry.position.0.abs() + ferry.position.1.abs();

        Ok(format!(
            "Manhattan distance from origin: {}",
            manhattan_distance
        ))
    }

    fn part2(&self) -> Result<String> {
        let mut ferry = WaypointFerry::new();
        for instruction in &self.0 {
            ferry.run_instruction(instruction);
        }

        let manhattan_distance = ferry.position.0.abs() + ferry.position.1.abs();

        Ok(format!(
            "Manhattan distance from origin: {}",
            manhattan_distance
        ))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let instructions = input
        .lines()
        .map(|line| -> Result<Instruction> {
            let line = line?;
            let operation = line[..1].parse()?;
            let argument = line[1..].parse()?;
            let argument = match operation {
                Operation::Left | Operation::Right => argument / 90,
                _ => argument,
            };

            Ok(Instruction {
                operation,
                argument,
            })
        })
        .collect::<Result<_>>()?;
    Ok(Box::new(Day12Solver(instructions)))
}
