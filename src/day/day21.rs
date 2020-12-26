use crate::day::{Day, DynSolver, Solver};

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub const DAY21: Day = Day {
    title: "Allergen Assessment",
    solver_from_input,
};

#[derive(Debug)]
struct FoodItem {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

struct AllergenTable<'a> {
    allergens: HashMap<&'a str, &'a str>,
    ingredients_with_allergen: HashSet<&'a str>,
}

impl<'a> AllergenTable<'a> {
    fn compile(food_items: &'a [FoodItem]) -> Self {
        let mut possible_allergens = HashMap::<&str, HashSet<&str>>::new();
        for food_item in food_items {
            for allergen in &food_item.allergens {
                possible_allergens
                    .entry(allergen)
                    .and_modify(|ingredients| {
                        ingredients.retain(|ingredient| {
                            food_item.ingredients.iter().any(|x| x == ingredient)
                        })
                    })
                    .or_insert_with(|| food_item.ingredients.iter().map(|s| s.as_str()).collect());
            }
        }

        let mut allergens = HashMap::new();
        let mut ingredients_with_allergen = HashSet::new();
        while allergens.len() != possible_allergens.len() {
            for (allergen, ingredients) in &possible_allergens {
                if allergens.contains_key(allergen) {
                    continue;
                }

                let the_one_ingredient = ingredients
                    .iter()
                    .filter(|ingredient| !ingredients_with_allergen.contains(*ingredient))
                    .exactly_one();

                if let Ok(matched_ingredient) = the_one_ingredient {
                    allergens.insert(*allergen, *matched_ingredient);
                    ingredients_with_allergen.insert(*matched_ingredient);
                }
            }
        }

        Self {
            allergens,
            ingredients_with_allergen,
        }
    }
}

struct Day21Solver(Vec<FoodItem>);
impl Solver for Day21Solver {
    fn part1(&self) -> Result<String> {
        let allergen_table = AllergenTable::compile(&self.0);

        let result = self
            .0
            .iter()
            .flat_map(|food_item| &food_item.ingredients)
            .filter(|ingredient| {
                !allergen_table
                    .ingredients_with_allergen
                    .contains(ingredient.as_str())
            })
            .count();

        Ok(format!("Allergen free ingredients appear {} times", result))
    }

    fn part2(&self) -> Result<String> {
        let allergen_table = AllergenTable::compile(&self.0);

        let result = allergen_table
            .allergens
            .iter()
            .sorted_by_key(|(allergen, _)| *allergen)
            .map(|(_, ingredient)| ingredient)
            .join(",");

        Ok(format!("Canonical dangerous ingredient list: {}", result))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let food_items = input
        .lines()
        .map(|line| {
            let line = line?;
            let allergens_start = line.find(" (").ok_or(anyhow!("No start of allergens"))?;
            let allergens_end = line.find(')').ok_or(anyhow!("No end of allergens"))?;
            let ingredients_string = &line[..allergens_start];
            let allergens_string = &line[(allergens_start + 11)..allergens_end];

            let ingredients = ingredients_string
                .split(' ')
                .map(|s| s.to_string())
                .collect();
            let allergens = allergens_string
                .split(", ")
                .map(|s| s.to_string())
                .collect();

            Ok(FoodItem {
                ingredients,
                allergens,
            })
        })
        .collect::<Result<_>>()?;
    Ok(Box::new(Day21Solver(food_items)))
}
