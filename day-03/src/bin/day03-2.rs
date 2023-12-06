fn main() {
    let input = include_str!("./input.txt");

    let sum = gear_ratios(input);

    println!("{sum}")
}

fn gear_ratios(input: &str) -> u32 {
    let schema: Vec<Vec<char>> = input.lines().map(to_char_vec).collect();
    let schema = Schema {
        width: schema[0].len() as u32,
        height: schema.len() as u32,
        matrix: schema,
    };
    process(&schema)
}

fn to_char_vec(text: &str) -> Vec<char> {
    text.trim().chars().collect()
}

struct Schema {
    width: u32,
    height: u32,
    matrix: Vec<Vec<char>>,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    row: u32,
    col: u32
}

#[derive(Debug, Clone, Copy)]
struct Numer {
    start: Point,
    end: Point,
    value: u32
}

fn process(schema: &Schema) -> u32 {
    let mut multiplied_gear_ratios: Vec<u32> = Vec::new();
    let matrix = &schema.matrix;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == '*' {
                match find_adjecent_digits_if_two(&matrix, row as u32, col as u32) {
                    Some((number1, number2)) => multiplied_gear_ratios.push(number1 * number2),
                    None => continue,
                }   
                
            }
        }
    }
    multiplied_gear_ratios.iter().sum()
}

fn find_adjecent_digits_if_two(matrix: &Vec<Vec<char>>, row: u32, col: u32) -> Option<(u32, u32)> {
    todo!()
}

fn has_adjecent_digit(point: &Point, schema: &Schema) -> Option<Point> {
    let row = point.row;
    let col = point.col;
    let max_row = (schema.matrix.len() - 1) as isize;
    let max_col = (schema.matrix[0].len() - 1) as isize;
    let matrix = &schema.matrix;
    if is_digit(check_if_in_bounds(row-1, col-1, max_row, max_col, &matrix)) {
        return Point {  }
    }
    is_digit(check_if_in_bounds(row-1, col, max_row, max_col, &matrix)) ||
    is_digit(check_if_in_bounds(row-1, col+1, max_row, max_col, &matrix)) ||
    is_digit(check_if_in_bounds(row, col-1, max_row, max_col, &matrix)) ||
    is_digit(check_if_in_bounds(row, col+1, max_row, max_col, &matrix)) ||
    is_digit(check_if_in_bounds(row+1, col-1, max_row, max_col, &matrix)) ||
    is_digit(check_if_in_bounds(row+1, col, max_row, max_col, &matrix)) ||
    is_digit(check_if_in_bounds(row+1, col+1, max_row, max_col, &matrix))
}

fn find_number(digit: Point) -> (u32, Point, Point) {}

fn check_if_in_bounds(
    row: isize,
    col: isize,
    max_row: isize,
    max_col: isize,
    matrix: &Vec<Vec<char>>,
) -> Result<char, ()> {
    if row < 0 || row > max_row {
        return Err(());
    } else if col < 0 || col > max_col {
        return Err(());
    }
    Ok(matrix[row as usize][col as usize])
}
fn is_digit(c: Result<char, ()>) -> bool {
    let Ok(c) = c else {
        return false;
    };
    c.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_example() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!(467835, gear_ratios(input));
    }
}
