use crate::day::{Day, DynSolver, Solver};

use std::collections::HashMap;
use std::convert::TryInto;
use std::io::BufRead;
use std::{iter, mem};

use anyhow::{anyhow, Result};

pub const DAY11: Day = Day {
    title: "Seating System",
    solver_from_input,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Free,
    Occupied,
}

impl Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Floor),
            'L' => Some(Self::Free),
            '#' => Some(Self::Occupied),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Pos,
    Neg,
    None,
}

impl Direction {
    const ALL: [(Self, Self); 8] = [
        (Self::None, Self::Neg),
        (Self::Pos, Self::Neg),
        (Self::Pos, Self::None),
        (Self::Pos, Self::Pos),
        (Self::None, Self::Pos),
        (Self::Neg, Self::Pos),
        (Self::Neg, Self::None),
        (Self::Neg, Self::Neg),
    ];

    fn sign(&self) -> isize {
        match self {
            Self::Pos => 1,
            Self::Neg => -1,
            Self::None => 0,
        }
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    tiles: Vec<Tile>,
    previous_tiles: Vec<Tile>,
}

impl Map {
    fn new(width: usize, tiles: Vec<Tile>) -> Self {
        let next_tiles = tiles.clone();
        Self {
            width,
            tiles,
            previous_tiles: next_tiles,
        }
    }

    fn step(&mut self, adjacents: &HashMap<usize, Vec<usize>>, occupied_tolerance: usize) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let adjacent_occupied = |i| {
                let adjacents = match adjacents.get(&i) {
                    Some(adjacents) => adjacents.iter(),
                    None => return 0,
                };

                adjacents
                    .filter(|adjacent_index| {
                        if let Some(Tile::Occupied) = self.tiles.get(**adjacent_index) {
                            true
                        } else {
                            false
                        }
                    })
                    .count()
            };

            let new_tile = match tile {
                Tile::Floor => continue,
                Tile::Free => {
                    if adjacent_occupied(i) == 0 {
                        Tile::Occupied
                    } else {
                        Tile::Free
                    }
                }
                Tile::Occupied => {
                    if adjacent_occupied(i) >= occupied_tolerance {
                        Tile::Free
                    } else {
                        Tile::Occupied
                    }
                }
            };

            self.previous_tiles[i] = new_tile;
        }

        mem::swap(&mut self.tiles, &mut self.previous_tiles);
    }

    fn scan(&self, index: usize, x: Direction, y: Direction) -> impl Iterator<Item = usize> {
        let width = self.width as isize;
        iter::successors(Some(index), move |prev| {
            let prev = *prev as isize;
            if x == Direction::Pos && prev % width == width - 1 {
                return None;
            }
            if x == Direction::Neg && prev % width == 0 {
                return None;
            }

            (prev + x.sign() + y.sign() * width).try_into().ok()
        })
        .skip(1)
    }
}

struct Day11Solver(Map);
impl Solver for Day11Solver {
    fn part1(&self) -> Result<String> {
        let mut map = self.0.clone();
        let adjacent_tiles = (0..map.tiles.len())
            .map(|i| {
                (
                    i,
                    Direction::ALL
                        .iter()
                        .filter_map(|(x, y)| map.scan(i, *x, *y).next())
                        .collect(),
                )
            })
            .collect();
        loop {
            map.step(&adjacent_tiles, 4);
            if map.tiles == map.previous_tiles {
                break;
            }
        }
        let occupied_seats = map
            .tiles
            .into_iter()
            .filter(|tile| *tile == Tile::Occupied)
            .count();

        Ok(format!("Occupied seats: {}", occupied_seats))
    }

    fn part2(&self) -> Result<String> {
        let mut map = self.0.clone();

        let adjacent_seats = (0..map.tiles.len())
            .map(|i| {
                let map = &map;
                (
                    i,
                    Direction::ALL
                        .iter()
                        .filter_map(|(x, y)| {
                            map.scan(i, *x, *y)
                                .skip_while(|i| map.tiles.get(*i) == Some(&Tile::Floor))
                                .next()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        loop {
            map.step(&adjacent_seats, 5);
            if map.tiles == map.previous_tiles {
                break;
            }
        }
        let occupied_seats = map
            .tiles
            .into_iter()
            .filter(|tile| *tile == Tile::Occupied)
            .count();

        Ok(format!("Occupied seats: {}", occupied_seats))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let mut lines = input.lines().filter_map(|r| r.ok());
    let first_line = lines.next().ok_or(anyhow!("No lines"))?;
    let width = first_line.len();
    let tiles = iter::once(first_line)
        .chain(lines)
        .flat_map(|line| line.chars().collect::<Vec<_>>().into_iter())
        .filter_map(|c| Tile::from_char(c))
        .collect();

    Ok(Box::new(Day11Solver(Map::new(width, tiles))))
}
