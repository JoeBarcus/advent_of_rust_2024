use std::fs::read_to_string;
use fancy_regex::Regex;

fn main() {
    let path = "src/input.txt";
    let data = read_data(path);
    let total = look_for_mul(data);
    println!("Part 1: {}", total.0);
    println!("Part 2: {}", total.1);
}

fn look_for_mul(data: Vec<String>) -> (i32, i32) {
    let mut total1 = 0;
    let mut total2 = 0;
    let mut do_it = true;
    let re = Regex::new(r"(don't\(\)|do\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for i in data {
        for cap_result in re.captures_iter(&i) {
            let cap = cap_result.unwrap();
            if let Some(action) = cap.get(1) {
                match action.as_str() {
                    "don't()" => {
                        do_it = false;
                    }
                    "do()" => {
                        do_it = true;
                    }
                    
                    _ => {},
                } 
            }

            if let (Some(x), Some(y)) = (cap.get(2), cap.get(3)) {
                let x_num: i32 = x.as_str().parse().unwrap();
                let y_num: i32 = y.as_str().parse().unwrap();
                total1 += x_num * y_num;
                if do_it {
                    total2 += x_num * y_num;
                }
            }
        }
    }
    (total1, total2)
}

fn read_data(data: &str) -> Vec<String> {
    read_to_string(data)
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
        let data = vec![
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
            ];
        let total = look_for_mul(data);
        assert_eq!(total.0, 161);
    }
    
    #[test]
    fn test_part_two() {
        let data = vec![
            String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
            ];
        let total = look_for_mul(data);
        assert_eq!(total.1, 48);
    }
}
