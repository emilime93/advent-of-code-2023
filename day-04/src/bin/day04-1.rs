use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    let wins = calculate_scratch_cards_win(input);

    println!("{wins}");
}

fn calculate_scratch_cards_win(input: &str) -> u32 {
    input.lines()
        .map(parse_game)
        .map(calculate_game_win)
        .sum()
}

#[derive(Debug)]
struct Game {
    my_numbers: Vec<u32>,
    winning_numbers: Vec<u32>
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, card_numbers) = s.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = card_numbers.split_once('|').unwrap();
        let winning_numbers: Vec<u32> = winning_numbers.trim().split(' ')
            .filter(|i| !i.is_empty())
            .map(|i|i.trim().parse::<u32>().expect("winning number wasn't a number"))
            .collect();
        let my_numbers: Vec<u32> = my_numbers.trim().split(' ')
            .filter(|i| !i.is_empty())
            .map(|i|i.trim().parse::<u32>().expect("winning number wasn't a number"))
            .collect();
        Ok(Game {my_numbers, winning_numbers })
    }
}

fn parse_game(line: &str) -> Game {
    match line.parse::<Game>() {
        Ok(game) => return game,
        Err(_) => panic!("Game not parseable")
    }
}

fn calculate_game_win(game: Game) -> u32 {
    let mut num_wins = 0;
    for num in game.my_numbers {
        if game.winning_numbers.contains(&num) {
            num_wins += 1;
        }
    }
    return if num_wins == 0 {
        0
    } else {
        u32::pow(2, num_wins - 1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, calculate_scratch_cards_win(input));
    }
}
