use std::fs::read_to_string;

#[derive(Debug)]
struct Xmas {
    value: char,
    x_coord: i32,
    y_coord: i32,
}

fn main() {
    let path = "src/input.txt";
    let data = read_data(path);
    let xmas_struct: Vec<Xmas> = convert_to_xmas_struct(data);
    let part_one = search_for_xmas(xmas_struct);
    println!("Part 1: {}", part_one.0);
    println!("Part 2: {}", part_one.1);
}

fn convert_to_xmas_struct(data: Vec<String>) -> Vec<Xmas> {
    let mut xmas_vec: Vec<Xmas> = Vec::new();
    for (x_index, row_value) in data.iter().enumerate() {
        for (y_index, xmas_value) in row_value.chars().into_iter().enumerate() {
            let xmas_struct = Xmas {
                value: xmas_value,
                x_coord: x_index as i32,
                y_coord: y_index as i32,
            };
            xmas_vec.push(xmas_struct);
        }
    }
    xmas_vec
}

fn search_for_xmas(data: Vec<Xmas>) -> (i32, i32) {
    let mut totals = 0;
    let mut totals_two = 0;
    for i in &data {
        if i.value == 'X' {
            totals += check_perimiter(&data, &i)
        }
    }
    for j in &data {
        if j.value == 'A' {
            totals_two += check_x_perimiter(&data, &j)
        }
    }
    (totals, totals_two)
}

fn check_x_perimiter(data: &Vec<Xmas>, current_xmas: &Xmas) -> i32 {
    let mut cell_totals = 0;
    let diagonal_vec = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let top_left_char = match find_next_val(data, current_xmas, diagonal_vec[0]) {
        Some(found) => found.value,
        None => 'Z',
    };
    let top_right_char = match find_next_val(data, current_xmas, diagonal_vec[1]) {
        Some(found) => found.value,
        None => 'Z',
    };
    let bottom_left_char = match find_next_val(data, current_xmas, diagonal_vec[2]) {
        Some(found) => found.value,
        None => 'Z',
    };
    let bottom_right_char = match find_next_val(data, current_xmas, diagonal_vec[3]) {
        Some(found) => found.value,
        None => 'Z',
    };

    let pattern = (top_left_char, top_right_char, bottom_left_char, bottom_right_char);

    let diagonal_pattern = vec![
        ('M','M','S','S'),
        ('M','S','M','S'),
        ('S','M','S','M'),
        ('S','S','M','M'),
    ];

    if diagonal_pattern.contains(&pattern) {
        cell_totals += 1;
    }
    
    cell_totals
}

fn check_perimiter(data: &Vec<Xmas>, current_xmas: &Xmas) -> i32 {
    let mut cell_totals = 0;
    let direction_vec = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for dir in direction_vec {
        let searching = find_next_val(data, current_xmas, dir);
        let searching = match searching {
            Some(searching) => searching,
            None => continue,
        };
        if searching.value == 'M' {
            let searching_again = find_next_val(data, searching, dir);
            let searching_again = match searching_again {
                Some(searching_again) => searching_again,
                None => continue,
            };

            if searching_again.value == 'A' {
                let searching_final = find_next_val(data, searching_again, dir);
                let searching_final = match searching_final {
                    Some(searching_final) => searching_final,
                    None => continue,
                };

                if searching_final.value == 'S' {
                    cell_totals += 1;
                }
            }
        }
    }

    cell_totals
}

fn find_next_val<'a>(
    data: &'a Vec<Xmas>,
    current_xmas: &'a Xmas,
    directions: (i32, i32),
) -> Option<&'a Xmas> {
    let searching = data.iter().find(|&searching_check| {
        searching_check.x_coord == current_xmas.x_coord + directions.0
            && searching_check.y_coord == current_xmas.y_coord + directions.1
    });
    searching
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
        let data = vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ];
        let xmas_struct: Vec<Xmas> = convert_to_xmas_struct(data);
        let part_one = search_for_xmas(xmas_struct);
        assert_eq!(part_one.0, 18);
    }

    #[test]
    fn test_part_two() {
        let data = vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ];
        let xmas_struct: Vec<Xmas> = convert_to_xmas_struct(data);
        let part_one = search_for_xmas(xmas_struct);
        assert_eq!(part_one.1, 9);
    }
}
