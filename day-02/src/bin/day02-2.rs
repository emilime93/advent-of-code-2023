use std::{str::FromStr, time::Instant};
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    let id_sum = combined_set_powers(input);

    println!("{}", id_sum.to_string());
}

fn combined_set_powers(input: &str) -> u32 {
    input
        .split('\n')
        .map(|s| s.parse::<Game>().unwrap())
        .map(|g| g.fewest_possible())
        .map(|g| set_power(g))
        .sum()
}

fn set_power(set: Set) -> u32 {
    set.red * set.green * set.blue
}

struct Game {
    id: u32,
    rounds: Vec<Round>
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, rounds) = s.split_once(":")
            .expect("Game not on correct format");

        let game_id_re = Regex::new(r"Game (?<digits>\d+)").unwrap();
        let Some(game_id) = game_id_re.captures(game_id) else {
            return Err(String::from(format!("Could not create game from: '{}'", game_id)));
        };
        let id = game_id["digits"].parse::<u32>().unwrap();
        let rounds: Vec<Round> = rounds
            .split(';')
            .map(|s| s.trim())
            .map(|e| e.parse::<Round>().unwrap())
            .collect();
        
        Ok(Game { id, rounds })
    }

}

impl Game {
    fn fewest_possible(&self) -> Set {
        let rounds_sets: Vec<Set> = self.rounds.iter()
            .map(|r| Set { red: r.red, green: r.green, blue: r.blue })
            .collect();

        Game::smallest_over_rounds(rounds_sets)
    }

    fn smallest_over_rounds(smallest_sets_per_round: Vec<Set>) -> Set {
        let smallest_red = smallest_sets_per_round.iter()
            .map(|s| s.red)
            .max().unwrap();
        let smallest_green = smallest_sets_per_round.iter()
            .map(|s| s.green)
            .max().unwrap();
        let smallest_blue = smallest_sets_per_round.iter()
            .map(|s| s.blue)
            .max().unwrap();
        Set { red: smallest_red, green: smallest_green, blue: smallest_blue }
    }
}

struct Round {
    red: u32,
    green: u32,
    blue: u32
}

struct Set {
    red: u32,
    green: u32,
    blue: u32
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?<digits>\d+) (?<color>red|blue|green)").unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for reveal in s.split(',').map(|s| s.trim()).collect::<Vec<&str>>() {
            let cap = re.captures(reveal)
                .expect("No match in round");
            let digit = cap["digits"].parse::<u32>().unwrap();
            match &cap["color"] {
                "red" => { red = digit; },
                "green" => { green = digit },
                "blue" => { blue = digit },
                _ => return Err(String::from("unexpected color"))
            }
        }

        Ok(Round { red, green, blue })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let set_powers = combined_set_powers(input);

        assert_eq!(2286, set_powers);
    }

    #[test]
    fn test_regex() {
        let re = Regex::new(r"Game: (?<digits>\d+)").unwrap();
        let hay = "Game: 5";
        let Some(caps) = re.captures(hay) else { panic!("No regex match") };

        assert_eq!(&caps["digits"], "5");
    }
}