use std::str::FromStr;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    let id_sum = sum_ids_of_compatible_games(input);

    println!("{}", id_sum.to_string());
}

fn sum_ids_of_compatible_games(input: &str) -> u32 {
    let game_rules = Rules { red: 12, green: 13, blue: 14 };
    input
        .split('\n')
        .map(|s| s.parse::<Game>().unwrap())
        .filter(|g| g.fits_rules(&game_rules))
        .map(|g| g.id)
        .sum()
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
    fn fits_rules(&self, rules: &Rules) -> bool {
        for round in self.rounds.iter() {
            if !round.fits_rules(rules) {
                return false
            }
        }
        true
    }
}

struct Round {
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
                _ => panic!("unexpected color")
            }
        }

        Ok(Round { red, green, blue })
    }
}

impl Round {
    fn fits_rules(&self, rules: &Rules) -> bool {
        self.red <= rules.red &&
        self.green <= rules.green &&
        self.blue <= rules.blue
    }
}

struct Rules {
    red: u32,
    green: u32,
    blue: u32
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

        let id_sum = sum_ids_of_compatible_games(input);

        assert_eq!(8, id_sum);
    }

    #[test]
    fn test_regex() {
        let re = Regex::new(r"Game: (?<digits>\d+)").unwrap();
        let hay = "Game: 5";
        let Some(caps) = re.captures(hay) else { panic!("No regex match") };

        assert_eq!(&caps["digits"], "5");
    }
}