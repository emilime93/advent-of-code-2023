fn main() {
    let input = include_str!("./input.txt");

    let sum = sum_engine_schematic(input);

    println!("{sum}")
}

fn sum_engine_schematic(input: &str) -> u32 {
    let schema: Vec<Vec<char>> = input
        .lines()
        .map(to_char_vec)
        .collect();
    let schema = Schema {
        width: schema[0].len() as u32,
        height: schema.len() as u32,
        matrix: schema
    };
    process(&schema)
}

fn to_char_vec(text: &str) -> Vec<char> {
    text.trim().chars().collect()
}

struct Schema {
    width: u32,
    height: u32,
    matrix: Vec<Vec<char>>
}

fn process(schema: &Schema) -> u32 {
    let mut numbers_to_count: Vec<u32> = Vec::new();
    let matrix = &schema.matrix;
    for row in 0..matrix.len() {
        let mut col_skips = 0;
        for col in 0..matrix[row].len() {
            // We already discovered this
            if col_skips > 0 {
                col_skips -= 1;
                continue;
            }
            if matrix[row][col].is_digit(10) {
                let mut digit_index = col;
                let mut add = false;
                let number_start = col as u32;
                let mut number = String::new();
                while digit_index < matrix[row].len() && matrix[row][digit_index].is_digit(10) {
                    number.push(matrix[row][digit_index]);
                    digit_index += 1;
                    col_skips += 1; // we don't want to residcover this
                }
                
                for check_col in number_start..(number_start + number.len() as u32) {
                    if has_adjecent_symbol(row, check_col as usize, &schema) {
                        add = true;
                        break;
                    }
                }
                if add {
                    numbers_to_count.push(number.parse::<u32>().unwrap());
                }
            }
        }
    }
    numbers_to_count.iter().sum()
}

fn has_adjecent_symbol(row: usize, col: usize, schema: &Schema) -> bool {
    let row = row as isize;
    let col = col as isize;
    let max_row = (schema.matrix.len() - 1) as isize;
    let max_col = (schema.matrix[0].len() - 1) as isize;
    let matrix = &schema.matrix;
    is_symbol(check_if_in_bounds(row-1, col-1, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row-1, col, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row-1, col+1, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row, col-1, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row, col, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row, col+1, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row+1, col-1, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row+1, col, max_row, max_col, &matrix)) ||
    is_symbol(check_if_in_bounds(row+1, col+1, max_row, max_col, &matrix))
}
fn check_if_in_bounds(row: isize, col: isize, max_row: isize, max_col: isize, matrix: &Vec<Vec<char>>) -> Result<char, ()> {
    if row < 0 || row > max_row {
        return Err(())
    } else if col < 0 || col > max_col {
        return Err(())
    }
    Ok( matrix[row as usize][col as usize] )
}
fn is_symbol(c: Result<char, ()>) -> bool {
    let Ok(c) = c else { return false; };
    !c.is_digit(10) && c != '.'
}

#[cfg(test)]
mod tests {
    use crate::sum_engine_schematic;


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

        assert_eq!(4361, sum_engine_schematic(input));
    }
}