use std::io::BufRead;

use anyhow::Result;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub type DynSolver = Box<dyn Solver>;
pub type SolverFromInput = fn(input: &mut dyn BufRead) -> Result<DynSolver>;

pub struct Day {
    pub title: &'static str,
    pub solver_from_input: SolverFromInput,
}

pub trait Solver {
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}
