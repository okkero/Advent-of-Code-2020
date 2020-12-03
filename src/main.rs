use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};

use day::Day;

mod day;
mod day1;
mod day2;

const DAYS: [Day; 2] = [day1::DAY1, day2::DAY2];

fn main() {
    let args = env::args();
    let day_string = args.skip(1).next().expect("Missing day argument");
    let day_num: usize = day_string.parse().expect("Unable to parse day");

    run_day(day_num);
}

fn run_day(day_num: usize) {
    let day = DAYS.get(day_num - 1).expect("Invalid day");
    let mut input_file =
        File::open(format!("input/day{}.txt", day_num)).expect("Input file not found");

    println!("--- Day {}: {} ---", day_num, day.title);
    println!();
    println!("Part 1:");
    let solution = (day.solution.part1)(&mut BufReader::new(&mut input_file));
    print_solution(solution);
    println!();
    input_file.seek(SeekFrom::Start(0)).unwrap();
    println!("Part 2:");
    let solution = (day.solution.part2)(&mut BufReader::new(&mut input_file));
    print_solution(solution);
}

fn print_solution(solution: Result<String>) {
    match solution {
        Ok(solution) => println!("{}", solution),
        Err(error) => println!("ERROR: {}", error),
    }
}
