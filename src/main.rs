use std::fs::read_to_string;

fn main() {
    let data = read_data("src/input.txt");
    let (left, right) = create_vecs(data);
    let total = subtract_values(&left, &right);
    println!("Part 1: {:?}", total);
    let total2 = get_similarity_score(left, right);
    println!("Part 2: {:?}", total2);
}

fn get_similarity_score(data1: Vec<i32>, data2: Vec<i32>) -> i32 {
    data1
        .iter()
        .map(|i| data2.iter().filter(|x| *x == i).count() as i32 * i)
        .sum()
}

fn subtract_values(data1: &Vec<i32>, data2: &Vec<i32>) -> i32 {
    let mut totals: Vec<i32> = Vec::new();
    for (a, b) in data1.iter().zip(data2.iter()) {
        totals.push((a - b).abs());
    }

    totals.iter().sum()
}

fn create_vecs(data: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for (_, val) in data.iter().enumerate() {
        let parts: Vec<&str> = val.splitn(2, "   ").collect();
        if let [key, value] = parts[..] {
            left.push(key.to_string());
            right.push(value.to_string());
        }
    }
    let mut left_converted = convert_vec_to_numbers(left).unwrap();
    let mut right_converted = convert_vec_to_numbers(right).unwrap();
    left_converted.sort();
    right_converted.sort();
    (left_converted, right_converted)
}

fn convert_vec_to_numbers(data: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    data.iter().map(|s| s.parse::<i32>()).collect()
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
            String::from("3   4"),
            String::from("4   3"),
            String::from("2   5"),
            String::from("1   3"),
            String::from("3   9"),
            String::from("3   3"),
        ];
        let (left, right) = create_vecs(test_data);
        let total = subtract_values(&left, &right);
        assert_eq!(total, 11);
    }

    #[test]
    fn test_part_two() {
        let test_data = vec![
            String::from("3   4"),
            String::from("4   3"),
            String::from("2   5"),
            String::from("1   3"),
            String::from("3   9"),
            String::from("3   3"),
        ];
        let (left, right) = create_vecs(test_data);
        let total = get_similarity_score(left, right);
        assert_eq!(total, 31);
    }
}
