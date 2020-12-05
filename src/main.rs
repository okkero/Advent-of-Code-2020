use anyhow::Result;
use std::env;
use std::io::{BufReader, Read};

use day::Day;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::fs::File;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;

const DAYS: [Day; 4] = [day1::DAY1, day2::DAY2, day3::DAY3, day4::DAY4];

fn main() {
    let args = env::args();
    let day_string = args.skip(1).next().expect("Missing day argument");
    let day_num: usize = day_string.parse().expect("Unable to parse day");
    let mut cookie = String::new();
    File::open("cookie")
        .expect("Unable to open cookie file")
        .read_to_string(&mut cookie)
        .expect("Unable to read cookie");

    run_day(&cookie, day_num);
}

fn run_day(cookie: &str, day_num: usize) {
    let day = DAYS.get(day_num - 1).expect("Invalid day");
    let input_result = Client::builder()
        .build()
        .unwrap()
        .get(&format!(
            "https://adventofcode.com/2020/day/{}/input",
            day_num
        ))
        .header(COOKIE, format!("session={}", cookie))
        .send();
    let input = input_result.expect("Unable to fetch input");

    let solver =
        (day.solver_from_input)(&mut BufReader::new(input)).expect("Unable to parse input");
    println!("--- Day {}: {} ---", day_num, day.title);
    println!();
    println!("Part 1:");
    let solution = solver.part1();
    print_solution(solution);
    println!();
    println!("Part 2:");
    let solution = solver.part2();
    print_solution(solution);
}

fn print_solution(solution: Result<String>) {
    match solution {
        Ok(solution) => println!("{}", solution),
        Err(error) => println!("ERROR: {}", error),
    }
}
