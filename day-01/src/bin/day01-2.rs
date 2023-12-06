use std::time::Instant;

fn main() {
    let time = Instant::now();
    let input = include_str!("./input.txt");
    println!("File read took: {:?}", time.elapsed());
    let calibration_value = find_calibration_value(input);
    println!("Work: {:?}", time.elapsed());
    println!("{}", calibration_value);
}

fn find_calibration_value(text: &str) -> u32 {
    text
        .split('\n')
        .map(replace_digits)
        .map(find_first_and_last)
        .map(combine)
        .sum()
}

fn print(number: u32) -> u32 {
    println!("{}", number.to_string());
    number
}

fn find_first_and_last(line: String) -> (u32, u32) {
    let mut digits: Vec<u32> = Vec::new();
    for character in line.chars() {
        if character.is_digit(10) {
            digits.push(character.to_string().parse::<u32>().unwrap())
        }
    }
    let first = digits.get(0).unwrap().to_owned();
    let last = digits.get(digits.len() - 1).unwrap().to_owned();
    
    (first, last)
}

fn replace_digits(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn combine(first_and_last: (u32, u32)) -> u32 {
    format!("{}{}", first_and_last.0.to_string(), first_and_last.1.to_string()).parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_first_and_last() {
        let (first, last) = find_first_and_last(String::from("1abc2"));
        assert_eq!(first, 1);
        assert_eq!(last, 2);
    }

    #[test]
    fn get_first_and_last_complex() {
        let (first, last) = find_first_and_last(String::from("21ab3c5f"));
        assert_eq!(first, 2);
        assert_eq!(last, 5);
    }

    #[test]
    fn get_first_and_last_spelled_out_1() {
        let (first, last) = find_first_and_last(String::from("twoeight67three5"));
        assert_eq!(first, 6);
        assert_eq!(last, 5);
    }

    #[test]
    fn part_two_test_input() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(281, find_calibration_value(input));
    }
}