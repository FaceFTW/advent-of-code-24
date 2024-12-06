use std::{fs::File, io::Read};

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
    part2();
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

fn part2() {
    // println!("Day 6 Part 2 result: {final_result}");
}
