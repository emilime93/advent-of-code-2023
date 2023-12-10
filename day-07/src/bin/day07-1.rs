use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    let winnings = calculate_winnings(input);

    println!("{winnings}");
}

fn calculate_winnings(input: &str) -> u32 {
    let mut players: Vec<Player> = input.lines()
        .map(|l| l.parse::<Player>().unwrap())
        .collect();
    players.sort_by(hand_bestness);

    players.into_iter().enumerate()
        .map(|(hand_rank, player)| player.calc_win((hand_rank + 1) as u32))
        .sum()
}

fn hand_bestness(player_1: &Player, player_2: &Player) -> Ordering {
    player_1.partial_cmp(&player_2).unwrap()
}

#[derive(Eq, PartialEq, Debug)]
struct Player {
    hand: Hand,
    bid: u32,
}

impl Player {
    fn compare_hands(&self, other: &Hand) -> Option<Ordering> {
        for (index, card) in self.hand.cards.iter().enumerate() {
            match card.cmp(&other.cards[index]) {
                Ordering::Equal => continue,
                Ordering::Greater => {
                    return Some(Ordering::Greater)
                },
                Ordering::Less => {
                    return Some(Ordering::Less)
                }
            }
        }
        None
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                self.compare_hands(&other.hand)
            },
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater)
        }

    }
}

impl Player {
    fn calc_win(&self, hand_rank: u32) -> u32 {
        hand_rank * self.bid
    }

    fn hand_type(&self) -> HandType {
        let counted_types = self.count_types();
        return if counted_types.len() == 1 {
            HandType::FiveOfAKind
        } else if counted_types.len() == 2 && counted_types[0].count == 4 {
            HandType::FourOfAKind
        } else if counted_types.len() == 2 &&
            counted_types[0].count == 3 &&
            counted_types[1].count == 2 {
            HandType::FullHouse
        } else if counted_types[0].count == 3 {
            HandType::ThreeOfAKind
        } else if counted_types.len() == 3 &&
            counted_types[0].count == 2 &&
            counted_types[1].count == 2 {
            HandType::TwoPair
        } else if counted_types[0].count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn count_types(&self) -> Vec<CardCount> {
        let mut map = HashMap::new();
        for card in self.hand.cards {
            map.entry(card).and_modify(|item| {
                *item += 1
            }).or_insert(1);
        }
        let mut counted_types = map.into_iter()
            .map(|(card, count)| {
                CardCount { card, count }
            })
            .collect::<Vec<_>>();
        counted_types.sort_by(| cc1, cc2| {
            cc1.partial_cmp(cc2).unwrap().reverse()
        });
        counted_types
    }
}

#[derive(Debug, PartialEq)]
struct CardCount {
    card: Card,
    count: u32,
}

impl PartialOrd for CardCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl FromStr for Player {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = text.split_once(' ').unwrap();
        let hand: Hand = hand.parse::<Hand>().unwrap();
        let bid: u32 = bid.parse::<u32>().unwrap();
        Ok(Player { hand, bid })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5]
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut cards: [Card; 5] = [Card::Ace; 5];
        for i in 0..cards.len() {
            cards[i] = text[i..(i+1)].parse::<Card>().unwrap();
        }
        Ok( Hand{ cards } )
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind= 7, FourOfAKind = 6, FullHouse = 5, ThreeOfAKind = 4, TwoPair = 3, OnePair = 2, HighCard = 1
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord)]
enum Card {
    Ace = 14, King = 13, Queen = 12, Jack = 11,
    Ten = 10, Nine = 9, Eight = 8, Seven = 7, Six = 6, Five = 5, Four = 4, Three = 3, Two = 2
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*self as u32).cmp(&(*other as u32)))
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_uppercase().as_str() {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(format!("No enum value for: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(6440, calculate_winnings(input));
    }

    #[test]
    fn hand_compare_four_of_a_kind() {
        let p1 = Player {
            hand: Hand {
                cards: [Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace,]
            },
            bid: 123
        };
        let p2 = Player {
            hand: Hand {
                cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Two,]
            },
            bid: 123
        };

        assert_eq!(Ordering::Less, hand_bestness(&p1, &p2))
    }

    #[test]
    fn hand_compare_identical() {
        let p1 = Player {
            hand: Hand {
                cards: [Card::King, Card::Ace, Card::Ace, Card::King, Card::King,]
            },
            bid: 123
        };
        let p2 = Player {
            hand: Hand {
                cards: [Card::Ace, Card::Ace, Card::Two, Card::Two, Card::Two,]
            },
            bid: 123
        };

        assert_eq!(Ordering::Less, hand_bestness(&p1, &p2))
    }

    #[test]
    fn hand_compare_different() {
        let p1 = Player {
            hand: Hand {
                cards: [Card::Ten, Card::Ace, Card::Three, Card::Ten, Card::Two,]
            },
            bid: 123
        };
        let p2 = Player {
            hand: Hand {
                cards: [Card::Three, Card::King, Card::Ten, Card::Seven, Card::Two,]
            },
            bid: 123
        };

        assert_eq!(Ordering::Greater, hand_bestness(&p1, &p2))
    }
}