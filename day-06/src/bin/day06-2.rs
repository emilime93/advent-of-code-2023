
fn main() {
    let input = include_str!("./input.txt");

    let error_margin = determine_error_margin_score(input);

    println!("{error_margin}");
}

fn determine_error_margin_score(input: &str) -> u64 {
    let lines = input.lines().map(|l| l.to_string()).collect::<Vec<String>>();

    let time = lines[0].replace("Time:", "").replace(" ", "").parse::<u64>().unwrap();
    dbg!(lines[1].replace("Distance:", "").replace(" ", ""));
    let distance = lines[1].replace("Distance:", "").replace(" ", "").parse::<u64>().unwrap();

    let race = Race { time, distance };

    race.number_of_ways_to_win()
}

fn parse_numbers(nubers: String) -> Vec<u64> {
    nubers.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn number_of_ways_to_win(&self) -> u64 {
        let mut counter: u64 = 0;
        for charge_time in 0..(self.time - 1) {
            if self.wins(charge_time) {
                counter += 1;
            }
        }
        counter
    }

    fn wins(&self, charge_millis: u64) -> bool {
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

        assert_eq!(71503, determine_error_margin_score(input));
    }

}