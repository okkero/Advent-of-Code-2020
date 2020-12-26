use crate::day::{Day, DynSolver, Solver};

use std::collections::VecDeque;
use std::io::BufRead;
use std::marker::PhantomData;

use anyhow::Result;

pub const DAY22: Day = Day {
    title: "Crab Combat",
    solver_from_input,
};

trait GameMode {}

#[derive(Clone, PartialEq)]
struct Combat;
impl GameMode for Combat {}

#[derive(Clone, PartialEq)]
struct RecursiveCombat;
impl GameMode for RecursiveCombat {}

#[derive(Clone, Copy)]
enum Player {
    Player1,
    Player2,
}

struct Winner {
    player: Player,
    score: u32,
}

enum GameState {
    Playing,
    Finished(Winner),
}

#[derive(Clone, PartialEq)]
struct Game<Mode: GameMode> {
    player1: VecDeque<u32>,
    player2: VecDeque<u32>,
    __phantom_mode: PhantomData<Mode>,
}

impl<Mode: GameMode> Game<Mode> {
    fn new(player1: VecDeque<u32>, player2: VecDeque<u32>) -> Self {
        Self {
            player1,
            player2,
            __phantom_mode: PhantomData,
        }
    }

    fn calculate_score(&self, player: Player) -> u32 {
        let deck = match player {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        };

        let deck_length = deck.len();
        deck.iter()
            .enumerate()
            .map(|(i, card)| card * (deck_length - i) as u32)
            .sum()
    }

    fn win(&self, winner: Player) -> Winner {
        Winner {
            player: winner,
            score: self.calculate_score(winner),
        }
    }
}

impl Game<Combat> {
    fn do_round(&mut self) -> GameState {
        let player1_card = if let Some(card) = self.player1.pop_front() {
            card
        } else {
            return GameState::Finished(self.win(Player::Player2));
        };
        let player2_card = if let Some(card) = self.player2.pop_front() {
            card
        } else {
            self.player1.push_front(player1_card);
            return GameState::Finished(self.win(Player::Player1));
        };

        if player1_card > player2_card {
            self.player1.push_back(player1_card);
            self.player1.push_back(player2_card);
        } else {
            self.player2.push_back(player2_card);
            self.player2.push_back(player1_card);
        }

        if self.player1.is_empty() {
            GameState::Finished(self.win(Player::Player2))
        } else if self.player2.is_empty() {
            GameState::Finished(self.win(Player::Player1))
        } else {
            GameState::Playing
        }
    }

    fn play_to_end(mut self) -> Winner {
        loop {
            let game_state = self.do_round();
            if let GameState::Finished(winner) = game_state {
                break winner;
            }
        }
    }
}

impl Game<RecursiveCombat> {
    fn do_round(&mut self) -> GameState {
        let player1_card = if let Some(card) = self.player1.pop_front() {
            card
        } else {
            return GameState::Finished(self.win(Player::Player2));
        };
        let player2_card = if let Some(card) = self.player2.pop_front() {
            card
        } else {
            self.player1.push_front(player1_card);
            return GameState::Finished(self.win(Player::Player1));
        };

        let winner = if self.player1.len() >= player1_card as usize
            && self.player2.len() >= player2_card as usize
        {
            let subgame = Self::new(
                self.player1
                    .iter()
                    .take(player1_card as usize)
                    .copied()
                    .collect(),
                self.player2
                    .iter()
                    .take(player2_card as usize)
                    .copied()
                    .collect(),
            );
            subgame.play_to_end().player
        } else if player1_card > player2_card {
            Player::Player1
        } else {
            Player::Player2
        };

        match winner {
            Player::Player1 => {
                self.player1.push_back(player1_card);
                self.player1.push_back(player2_card);
            }
            Player::Player2 => {
                self.player2.push_back(player2_card);
                self.player2.push_back(player1_card);
            }
        };

        if self.player1.is_empty() {
            GameState::Finished(self.win(Player::Player2))
        } else if self.player2.is_empty() {
            GameState::Finished(self.win(Player::Player1))
        } else {
            GameState::Playing
        }
    }

    fn play_to_end(mut self) -> Winner {
        let mut history = Vec::new();
        loop {
            if history.contains(&self) {
                break self.win(Player::Player1);
            }

            history.push(self.clone());
            let game_state = self.do_round();
            if let GameState::Finished(winner) = game_state {
                break winner;
            }
        }
    }
}

struct Day22Solver {
    player1: Vec<u32>,
    player2: Vec<u32>,
}
impl Solver for Day22Solver {
    fn part1(&self) -> Result<String> {
        let game = Game::<Combat>::new(self.player1.clone().into(), self.player2.clone().into());

        let winner_score = game.play_to_end().score;

        Ok(format!("Winner score: {}", winner_score))
    }

    fn part2(&self) -> Result<String> {
        let game =
            Game::<RecursiveCombat>::new(self.player1.clone().into(), self.player2.clone().into());

        let winner_score = game.play_to_end().score;

        Ok(format!("Winner score: {}", winner_score))
    }
}

fn solver_from_input(input: &mut dyn BufRead) -> Result<DynSolver> {
    let player1 = parse_player_deck(input)?;
    let player2 = parse_player_deck(input)?;
    Ok(Box::new(Day22Solver { player1, player2 }))
}

fn parse_player_deck(input: &mut dyn BufRead) -> Result<Vec<u32>> {
    input
        .lines()
        .skip(1)
        .take_while(|line| {
            if let Ok("") = line.as_ref().map(|s| s.as_str()) {
                false
            } else {
                true
            }
        })
        .map(|line| Ok(line?.parse()?))
        .collect()
}
