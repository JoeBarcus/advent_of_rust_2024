use std::fs::read_to_string;

fn main() {
    let data = read_data("src/input.txt");
    let part1 = check_totals(data);
    println!("{:?}", part1.0);
    println!("{:?}", part1.1);
}

fn check_totals(data: Vec<String>) -> (i32, i32) {
    let mut check_count = 0;
    let mut check_count_two = 0;
    for i in data {
        let row_ints: Vec<i32> = i 
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let normal_check = check_row(row_ints.clone());
        let row_ints_reversed = row_ints.iter().rev().cloned().collect();
        let reversed_check = check_row(row_ints_reversed);
        if normal_check || reversed_check {
            check_count += 1;
            check_count_two += 1;
        } else if remove_a_value(row_ints) {
            check_count_two += 1;
        } 
        
    }
    (check_count, check_count_two)
}

fn remove_a_value(row: Vec<i32>) -> bool {
    for i in 0..row.len() {
        let mut temp = row.clone();
        temp.remove(i);
        let temp_reversed = temp.iter().rev().cloned().collect();
        if check_row(temp) {
            return true;
        } else if check_row(temp_reversed) { 
            return true;
        }
    }
    false
}

fn check_row(row: Vec<i32>) -> bool {
    row.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        diff >= 1 && diff <= 3
    })
}

fn read_data(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_data = vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ];

        assert_eq!(check_totals(test_data).0, 2);
    }
    
    #[test]
    fn test_part_two() {
        let test_data = vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ];

        assert_eq!(check_totals(test_data).1, 4);
    }
}
