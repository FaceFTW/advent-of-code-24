use std::{fs::File, io::Read};

fn main() {
    let mut string = String::new();
    match File::open("day10/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(&data);
    part2();
}

fn parse_data(string: &str) -> Vec<Vec<u32>> {
    string
        .trim_end()
        .split("\n")
        .map(|raw| raw.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1(data: &Vec<Vec<u32>>) {
    let mut final_result = 0;
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] == 9 {
                final_result = final_result + calc_trailheads(data, (row, col), &mut Vec::new());
            }
        }
    }

    println!("Day 10 Part 1 result: {final_result}");
}

fn calc_trailheads(
    data: &Vec<Vec<u32>>,
    pos: (usize, usize),
    visited: &mut Vec<(usize, usize)>,
) -> u64 {
    let (row, col) = pos;
    match data[row][col] {
        0 if visited.contains(&pos) => 0,
        0 => {
            //Prevents double counting
            visited.push(pos);
            1
        }
        _ => {
            let mut count = 0;
            //Up
            if let Some(new_row) = row.checked_sub(1) {
                if data[new_row][col] == data[row][col] - 1 {
                    count = count + calc_trailheads(data, (new_row, col), visited);
                }
            }
            //Down
            if row + 1 < data.len() {
                if data[row + 1][col] == data[row][col] - 1 {
                    count = count + calc_trailheads(data, (row + 1, col), visited);
                }
            }
            //Left
            if let Some(new_col) = col.checked_sub(1) {
                if data[row][new_col] == data[row][col] - 1 {
                    count = count + calc_trailheads(data, (row, new_col), visited);
                }
            }
            //Right
            if col + 1 < data[0].len() {
                if data[row][col + 1] == data[row][col] - 1 {
                    count = count + calc_trailheads(data, (row, col + 1), visited);
                }
            }
            dbg!(count);
            count
        }
    }
}

fn part2() {
    // println!("Day 10 Part 2 result: {final_result}");
}
