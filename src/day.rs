use anyhow::Result;
use std::io::BufRead;

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
