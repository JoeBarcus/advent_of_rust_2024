use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct ObstacleMap {
    ch: char,
    x: i32,
    y: i32,
    visited: bool,
    direction: Direction,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let data = read_input("src/input.txt");
    let data_converted = convert_to_obstacle_map(data);
    let total = game_loop(data_converted);
    println!("Part 1: {}", total);
}

fn game_loop(mut data: Vec<ObstacleMap>) -> usize {
    let mut position = find_starting_position(&mut data).clone();
    loop {
        match position.direction {
            Direction::Up => {
                if let Some(check_position) = data
                    .iter_mut()
                    .find(|loc| loc.x == position.x && loc.y == position.y - 1)
                {
                    if check_position.ch == '#' {
                        if let Some(next) = data
                                            .iter_mut()
                                            .find(|loc| loc.x == position.x + 1 && loc.y == position.y) {
                                                next.direction = Direction::Right;
                                                next.visited = true;
                                                position = next.clone();
                                            } else {
                                                break;
                                            }
                    } else {
                        check_position.direction = Direction::Up;
                        check_position.visited = true;
                        position = check_position.clone();
                    }
                } else {
                    break;
                }
            }
            Direction::Right => {
                if let Some(check_position) = data
                    .iter_mut()
                    .find(|loc| loc.x == position.x + 1 && loc.y == position.y)
                {
                    if check_position.ch == '#' {
                        if let Some(next) = data
                                            .iter_mut()
                                            .find(|loc| loc.x == position.x && loc.y == position.y + 1) {
                                                next.direction = Direction::Down;
                                                next.visited = true;
                                                position = next.clone();
                                            } else {
                                                break;
                                            }
                    } else {
                        check_position.direction = Direction::Right;
                        check_position.visited = true;
                        position = check_position.clone();
                    }
                } else {
                    break;
                }
            }
            Direction::Down => {
                if let Some(check_position) = data
                    .iter_mut()
                    .find(|loc| loc.x == position.x && loc.y == position.y + 1)
                {
                    if check_position.ch == '#' {
                        if let Some(next) = data
                                            .iter_mut()
                                            .find(|loc| loc.x == position.x - 1 && loc.y == position.y) {
                                                next.direction = Direction::Left;
                                                next.visited = true;
                                                position = next.clone();
                                            } else {
                                                break;
                                            }
                    } else {
                        check_position.direction = Direction::Down;
                        check_position.visited = true;
                        position = check_position.clone();
                    }
                } else {
                    break;
                }
            }
            Direction::Left => {
                if let Some(check_position) = data
                    .iter_mut()
                    .find(|loc| loc.x == position.x - 1 && loc.y == position.y)
                {
                    if check_position.ch == '#' {
                        if let Some(next) = data
                                            .iter_mut()
                                            .find(|loc| loc.x == position.x && loc.y == position.y - 1) {
                                                next.direction = Direction::Up;
                                                next.visited = true;
                                                position = next.clone();
                                            } else {
                                                break;
                                            }
                    } else {
                        check_position.direction = Direction::Left;
                        check_position.visited = true;
                        position = check_position.clone();
                    }
                } else {
                    break;
                }
            }
        }
    }

    data.iter().filter(|cell| cell.visited).count()
}

fn find_starting_position(data: &mut Vec<ObstacleMap>) -> ObstacleMap {
    let start = data.iter_mut().find(|loc| loc.ch == '^').unwrap();
    start.visited = true;
    start.clone()
}

fn convert_to_obstacle_map(data: Vec<String>) -> Vec<ObstacleMap> {
    let mut obstacle_vec = Vec::new();
    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            obstacle_vec.push(ObstacleMap {
                ch,
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
                visited: false,
                direction: Direction::Up,
            })
        }
    }

    obstacle_vec
}

fn read_input(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
