//Not my finest work with all of the duplicated code
//At least its fairly verbose

use std::{fs::File, io::Read};
#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Data {
    start_idx: (usize, usize),
    obstructions: Vec<Vec<bool>>,
}

fn main() {
    let mut string = String::new();
    match File::open("day6/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(&data);
    part2(&data);
}

fn parse_data(string: &str) -> Data {
    let raw_lines: Vec<&str> = string.trim_end().split("\n").collect();
    let mut obstructions: Vec<Vec<bool>> = vec![vec![false; raw_lines[0].len()]; raw_lines.len()];
    let mut start: Option<(usize, usize)> = None;
    for row_idx in 0..raw_lines.len() {
        let col_iter: Vec<char> = raw_lines[row_idx].chars().collect();
        for col_idx in 0..col_iter.len() {
            match col_iter[col_idx] {
                '#' => obstructions[row_idx][col_idx] = true,
                '^' => start = Some((row_idx, col_idx)),
                _ => (),
            }
        }
    }

    Data {
        start_idx: start.unwrap(),
        obstructions,
    }
}

fn part1(data: &Data) {
    let mut curr_direction = Direction::Up;
    let mut curr_pos: (usize, usize) = data.start_idx;
    let max_h = data.obstructions.len();
    let max_w = data.obstructions[0].len();

    let mut travelled_map = vec![vec![false; max_w as usize]; max_h as usize];
    travelled_map[data.start_idx.0][data.start_idx.1] = true;

    loop {
        travelled_map[curr_pos.0][curr_pos.1] = true;
        match curr_direction {
            Direction::Up => match curr_pos.0.checked_sub(1) {
                Some(new_row) => {
                    if data.obstructions[new_row][curr_pos.1] {
                        curr_direction = Direction::Right;
                    } else {
                        curr_pos.0 = new_row;
                    }
                }
                None => break,
            },
            Direction::Down => match curr_pos.0 + 1 < max_h {
                true => {
                    if data.obstructions[curr_pos.0 + 1][curr_pos.1] {
                        curr_direction = Direction::Left;
                    } else {
                        curr_pos.0 = curr_pos.0 + 1;
                    }
                }
                false => break,
            },
            Direction::Left => match curr_pos.1.checked_sub(1) {
                Some(new_col) => {
                    if data.obstructions[curr_pos.0][new_col] {
                        curr_direction = Direction::Up;
                    } else {
                        curr_pos.1 = new_col;
                    }
                }
                None => break,
            },
            Direction::Right => match curr_pos.1 + 1 < max_w {
                true => {
                    if data.obstructions[curr_pos.0][curr_pos.1 + 1] {
                        curr_direction = Direction::Down;
                    } else {
                        curr_pos.1 = curr_pos.1 + 1;
                    }
                }
                false => break,
            },
        }
    }

    let final_result = travelled_map.into_iter().fold(0, |acc, row| {
        acc + row.into_iter().fold(0, |acc, pos| match pos {
            true => acc + 1,
            false => acc,
        })
    });

    println!("Day 6 Part 1 result: {final_result}");
}

fn part2(data: &Data) {
    let travelled_map = get_visited_map(data);
    let mut coords_list: Vec<(usize, usize)> = vec![];
    for row in 0..travelled_map.len() {
        for col in 0..travelled_map[0].len() {
            if travelled_map[row][col] {
                coords_list.push((row, col));
            }
        }
    }

    let mut final_result = 0;

    //I know there are better ways but I want to get this done and call it a night
    for obstruction_pos in coords_list {
        if obstruction_pos == data.start_idx {
            continue;
        }
        let mut temp_map = vec![vec![false; travelled_map[0].len()]; travelled_map.len()];
        for row in 0..travelled_map.len() {
            for col in 0..travelled_map[0].len() {
                temp_map[row][col] = data.obstructions[row][col];
            }
        }
        temp_map[obstruction_pos.0][obstruction_pos.1] = true;

        if check_does_loop(data.start_idx, &temp_map) {
            println!("pos causes loop: {obstruction_pos:#?}");
            final_result = final_result + 1;
        } else {
            println!("no loop: {obstruction_pos:#?}");
        }
    }

    println!("Day 6 Part 2 result: {final_result}");
}

fn get_visited_map(data: &Data) -> Vec<Vec<bool>> {
    let mut curr_direction = Direction::Up;
    let mut curr_pos: (usize, usize) = data.start_idx;
    let max_h = data.obstructions.len();
    let max_w = data.obstructions[0].len();

    let mut travelled_map = vec![vec![false; max_w as usize]; max_h as usize];
    travelled_map[data.start_idx.0][data.start_idx.1] = true;

    loop {
        travelled_map[curr_pos.0][curr_pos.1] = true;
        match curr_direction {
            Direction::Up => match curr_pos.0.checked_sub(1) {
                Some(new_row) => {
                    if data.obstructions[new_row][curr_pos.1] {
                        curr_direction = Direction::Right;
                    } else {
                        curr_pos.0 = new_row;
                    }
                }
                None => break,
            },
            Direction::Down => match curr_pos.0 + 1 < max_h {
                true => {
                    if data.obstructions[curr_pos.0 + 1][curr_pos.1] {
                        curr_direction = Direction::Left;
                    } else {
                        curr_pos.0 = curr_pos.0 + 1;
                    }
                }
                false => break,
            },
            Direction::Left => match curr_pos.1.checked_sub(1) {
                Some(new_col) => {
                    if data.obstructions[curr_pos.0][new_col] {
                        curr_direction = Direction::Up;
                    } else {
                        curr_pos.1 = new_col;
                    }
                }
                None => break,
            },
            Direction::Right => match curr_pos.1 + 1 < max_w {
                true => {
                    if data.obstructions[curr_pos.0][curr_pos.1 + 1] {
                        curr_direction = Direction::Down;
                    } else {
                        curr_pos.1 = curr_pos.1 + 1;
                    }
                }
                false => break,
            },
        }
    }
    travelled_map
}

fn check_does_loop(start: (usize, usize), map: &Vec<Vec<bool>>) -> bool {
    let mut curr_direction = Direction::Up;
    let mut curr_pos: (usize, usize) = start;
    let max_h = map.len();
    let max_w = map[0].len();

    let mut travelled_map = vec![vec![vec![]; max_w as usize]; max_h as usize];
    travelled_map[start.0][start.1] = vec![Direction::Up];

    loop {
        travelled_map[curr_pos.0][curr_pos.1].push(curr_direction.clone());
        match curr_direction {
            Direction::Up => match curr_pos.0.checked_sub(1) {
                Some(new_row) => {
                    if travelled_map[new_row][curr_pos.1].contains(&curr_direction) {
                        return true;
                    } else if map[new_row][curr_pos.1] {
                        curr_direction = Direction::Right;
                    } else {
                        curr_pos.0 = new_row;
                    }
                }
                None => return false,
            },
            Direction::Down => match curr_pos.0 + 1 < max_h {
                true => {
                    if travelled_map[curr_pos.0 + 1][curr_pos.1].contains(&curr_direction) {
                        return true;
                    } else if map[curr_pos.0 + 1][curr_pos.1] {
                        curr_direction = Direction::Left;
                    } else {
                        curr_pos.0 = curr_pos.0 + 1;
                    }
                }
                false => return false,
            },
            Direction::Left => match curr_pos.1.checked_sub(1) {
                Some(new_col) => {
                    if travelled_map[curr_pos.0][new_col].contains(&curr_direction) {
                        return true;
                    } else if map[curr_pos.0][new_col] {
                        curr_direction = Direction::Up;
                    } else {
                        curr_pos.1 = new_col;
                    }
                }
                None => return false,
            },
            Direction::Right => match curr_pos.1 + 1 < max_w {
                true => {
                    if travelled_map[curr_pos.0][curr_pos.1 + 1].contains(&curr_direction) {
                        return true;
                    } else if map[curr_pos.0][curr_pos.1 + 1] {
                        curr_direction = Direction::Down;
                    } else {
                        curr_pos.1 = curr_pos.1 + 1;
                    }
                }
                false => return false,
            },
        }
    }
}
