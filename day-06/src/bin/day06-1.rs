
fn main() {
    let input = include_str!("./input.txt");

    let error_margin = determine_error_margin_score(input);

    println!("{error_margin}");
}

fn determine_error_margin_score(input: &str) -> u32 {
    let lines = input.lines().map(|l| l.to_string()).collect::<Vec<String>>();

    let times = lines[0].replace("Time:", "").trim().to_string();
    let times: Vec<u32> = parse_numbers(times);

    let distances = lines[1].replace("Distance:", "").trim().to_string();
    let distances: Vec<u32> = parse_numbers(distances);

    let mut races = Vec::new();
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        races.push(Race { time, distance });
    }

    let mut error_margin: u32 = 1;
    for race in races {
        dbg!(race.number_of_ways_to_win());
        error_margin *= race.number_of_ways_to_win();
    }

    error_margin
}

fn parse_numbers(nubers: String) -> Vec<u32> {
    nubers.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn number_of_ways_to_win(&self) -> u32 {
        let mut counter: u32 = 0;
        for charge_time in 0..(self.time - 1) {
            if self.wins(charge_time) {
                counter += 1;
            }
        }
        counter
    }

    fn wins(&self, charge_millis: u32) -> bool {
        let duration_left_after_charge = self.time - charge_millis;
        let distance = charge_millis * duration_left_after_charge;
        distance > self.distance
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, determine_error_margin_score(input));
    }
}