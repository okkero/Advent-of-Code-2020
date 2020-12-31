use day::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21,
    day22, day23, day24, day25, day3, day4, day5, day6, day7, day8, day9, Day,
};

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;

mod day;

const DAYS: [Day; 25] = [
    day1::DAY1,
    day2::DAY2,
    day3::DAY3,
    day4::DAY4,
    day5::DAY5,
    day6::DAY6,
    day7::DAY7,
    day8::DAY8,
    day9::DAY9,
    day10::DAY10,
    day11::DAY11,
    day12::DAY12,
    day13::DAY13,
    day14::DAY14,
    day15::DAY15,
    day16::DAY16,
    day17::DAY17,
    day18::DAY18,
    day19::DAY19,
    day20::DAY20,
    day21::DAY21,
    day22::DAY22,
    day23::DAY23,
    day24::DAY24,
    day25::DAY25,
];

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
