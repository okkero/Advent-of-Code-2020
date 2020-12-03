use anyhow::Result;
use std::io::BufRead;

pub type Part = fn(input: &mut dyn BufRead) -> Result<String>;

pub struct Day {
    pub title: &'static str,
    pub solution: Solution,
}

pub struct Solution {
    pub part1: Part,
    pub part2: Part,
}
