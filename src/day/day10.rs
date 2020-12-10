use crate::day::{Day, DynSolver, Solver};

use std::io::BufRead;

use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub const DAY10: Day = Day {
    title: "Adapter Array",
    solver_from_input,
};

struct Day10Solver(Vec<u32>);
impl Solver for Day10Solver {
    fn part1(&self) -> Result<String> {
        let (ones, threes) = self.0.iter().tuple_windows().map(|(a, b)| b - a).fold(
            (0, 0),
            |(ones, threes), diff| match diff {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            },
        );

        Ok(format!("Result: {}", ones * threes))
    }

    fn part2(&self) -> Result<String> {
        fn count_arrangements(ratings: &[u32], counted: &mut HashMap<u32, u64>) -> u64 {
            if ratings.len() == 1 {
                return 1;
            }

            let current_rating = ratings[0];
            if let Some(count) = counted.get(&current_rating) {
                return *count;
            }

            let count = (1..ratings.len())
                .take_while(|index| {
                    let rating = ratings[*index];
                    let diff = rating - current_rating;
                    diff >= 1 && diff <= 3
                })
                .map(|index| count_arrangements(&ratings[index..], counted))
                .sum();
            counted.insert(current_rating, count);

            count
        }

        let ratings = self.0.iter().map(|n| *n).collect::<Vec<_>>();
        let arrangements = count_arrangements(&ratings, &mut HashMap::new());

        Ok(format!("Possible adapter arrangements: {}", arrangements))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut largest_rating = 0;
    let mut adapters = input
        .lines()
        .map(|line| -> Result<u32> {
            let rating = line?.parse()?;
            if rating > largest_rating {
                largest_rating = rating;
            }

            Ok(rating)
        })
        .collect::<Result<Vec<_>>>()?;

    adapters.push(0);
    adapters.sort();
    adapters.push(largest_rating + 3);

    Ok(Box::new(Day10Solver(adapters)))
}
