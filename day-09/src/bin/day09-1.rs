fn main() {
    let input = include_str!("input.txt");

    let total_sum = extrapolated_values(input);

    println!("{total_sum}");
}

fn extrapolated_values(input: &str) -> i32 {
    input.lines()
        .map(to_u32_vec)
        .map(extrapolate_next_value)
        .sum()
}

fn to_u32_vec(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|e| e.parse::<i32>().expect(&*format!("Item was not number: {e}")))
        .collect()
}

// vec[0] = zero,
// vec[1] = middle,
// vec[2] = original,
fn extrapolate_next_value(input: Vec<i32>) -> i32 {
    let mut diffs = find_all_diffs_rec(input, Vec::new());
    diffs.reverse();
    diffs[0].push(0);
    for i in 1..diffs.len() {
        let this_row_size = diffs[i].len();
        let below_row_size = this_row_size - 1;
        let left_item = diffs[i][this_row_size - 1];
        let below_item = diffs[i - 1][below_row_size];

        let number_to_add = diff_by_increasing_left_value_with_value_below(left_item, below_item);

        diffs.get_mut(i)
            .expect("Array out of bounds")
            .push(number_to_add);
    }
    let last_index = diffs.len() - 1;
    diffs[last_index][diffs[last_index].len() - 1]
}

fn diff_by_increasing_left_value_with_value_below(left: i32, below: i32) -> i32 {
    left + below
}

fn find_all_diffs_rec(arr :Vec<i32>, mut arrs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    arrs.push(arr.clone());
    if is_zero_arr(&arr) {
        return arrs
    }
    find_all_diffs_rec(find_diff(&arr), arrs)
}

fn find_diff(arr: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::new();
    for i in 0..(arr.len() - 1) {
        diffs.push(arr[i+1] - arr[i])
    }
    diffs
}

fn is_zero_arr(arr: &Vec<i32>) -> bool {
    *arr == vec![0_i32; arr.len()]
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let total_sum = extrapolated_values(input);

        assert_eq!(total_sum, 114)
    }

    #[test]
    fn find_diff_test() {
        let input = vec![0, 3, 6, 9, 12, 15];

        assert_eq!(find_diff(&input), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn find_all_diffs_rec_test() {
        let input = vec![0, 3, 6, 9, 12, 15];

        assert_eq!(find_all_diffs_rec(input, Vec::new()),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn extrapolate_test() {
        let input = vec![0, 3, 6, 9, 12, 15];

        assert_eq!(extrapolate_next_value(input), 18);
    }

    #[test]
    fn extrapolate_test_2() {
        let input = vec![1, 3, 6, 10, 15, 21];

        assert_eq!(extrapolate_next_value(input), 28);
    }
}