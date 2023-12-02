fn main() {
    let input = include_str!("./input.txt");
    let calibration_value: u32 = input
        .split('\n')
        .map(find_first_and_last)
        .map(combine)
        .sum();
    println!("{}", calibration_value);
}

fn find_first_and_last(line: &str) -> (u32, u32) {
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

fn combine(first_and_last: (u32, u32)) -> u32 {
    format!("{}{}", first_and_last.0.to_string(), first_and_last.1.to_string()).parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_first_and_last() {
        let (first, last) = find_first_and_last("1abc2");
        assert_eq!(first, 1);
        assert_eq!(last, 2);
    }


    #[test]
    fn get_first_and_last_complex() {
        let (first, last) = find_first_and_last("21ab3c5f");
        assert_eq!(first, 2);
        assert_eq!(last, 5);
    }
}