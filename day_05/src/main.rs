use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let data = read_data("src/input.txt");
    let (rules, update) = split_vec(data);
    let (part_one, part_two) = find_valid_updates(rules, update);
    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}

fn find_valid_updates(rules: Vec<String>, update: Vec<String>) -> (i32, i32) {
    let mut middle_value_vec = Vec::new();
    let mut fixed_middle_value_vec = Vec::new();
    let get_rules = get_rules(rules);
    for i in update {
        let mut parts: Vec<i32> = i
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        parts.reverse();

        let bad_row = row_check(&parts, &get_rules);

        let parts_unreversed: Vec<i32> = parts.iter().rev().cloned().collect();
        
        let mut reordered: Vec<i32> = Vec::new();
        if bad_row == true {
            for i in parts_unreversed.iter() {
                let orders: Vec<i32> = get_rules
                    .iter()
                    .filter(|(first, _)| first == i)
                    .map(|(_, second)| *second)
                    .collect();
                let targets: HashSet<i32> = orders.clone().into_iter().collect();
                if orders.iter().any(|t| reordered.contains(t)) {
                    let indices: Vec<usize> = reordered.iter()
                                                            .enumerate()
                                                            .filter(|(_, x)| targets.contains(&x))
                                                            .map(|(i, _)| i)
                                                            .collect();
                    if let Some(min_index) = indices.iter().min() {
                        reordered.insert(*min_index, *i);
                    }
                } else {
                    reordered.push(*i);
                }
            }
            let fixed_middle_index = reordered.len() / 2;
            let fixed_middle_value = reordered[fixed_middle_index];
            fixed_middle_value_vec.push(fixed_middle_value); 
            continue;
            }
        
        let middle_index = parts.len() / 2;
        let middle_value = parts[middle_index];
        middle_value_vec.push(middle_value);

    }

    (middle_value_vec.iter().sum(), fixed_middle_value_vec.iter().sum())
}

fn row_check(parts: &Vec<i32>, get_rules: &Vec<(i32, i32)>) -> bool {
    for (idx, j) in parts.clone().iter().enumerate() {
        let matches: Vec<i32> = get_rules
            .iter()
            .filter(|(first, _)| first == j)
            .map(|(_, second)| *second)
            .collect();
        let bad_order_exists = matches.iter().any(|t| parts[idx..].contains(t));
        if bad_order_exists {
            return true;
        }
    }
    false
}

fn get_rules(rules: Vec<String>) -> Vec<(i32, i32)> {
    let rules_nums: Vec<(i32, i32)> = rules
        .iter()
        .filter_map(|s| {
            let mut parts = s.splitn(2, '|');
            let key = parts.next()?;
            let value = parts.next()?;
            let k: i32 = key.trim().parse().ok()?;
            let v: i32 = value.trim().parse().ok()?;
            Some((k, v))
        })
        .collect();

    rules_nums
}

fn split_vec(data: Vec<String>) -> (Vec<String>, Vec<String>) {
    if let Some(index) = data.iter().position(|s| s.is_empty()) {
        let (first, rest) = data.split_at(index);
        let second = rest[1..].to_vec();
        (first.to_vec(), second)
    } else {
        (data, Vec::new())
    }
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
        let data = read_data("src/test.txt");
        let (rules, update) = split_vec(data);
        let part_one = find_valid_updates(rules, update);
        assert_eq!(part_one.0, 143);
    }

    #[test]
    fn test_part_two() {
        let data = read_data("src/test.txt");
        let (rules, update) = split_vec(data);
        let part_one = find_valid_updates(rules, update);
        assert_eq!(part_one.1, 123);
    }
}
