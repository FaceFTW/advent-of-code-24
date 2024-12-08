use std::{collections::HashMap, fs::File, io::Read};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

enum MapState {
    Open,
    Antinode,
    Antenna,
}

struct Data {
    antenna_map: HashMap<Position, MapState>,
    freq_map: HashMap<char, Vec<Position>>,
    max_row: usize,
    max_col: usize,
}

fn main() {
    let mut string = String::new();
    match File::open("day8/input.txt") {
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
    let mut antenna_map = HashMap::new();
    let mut freq_map: HashMap<char, Vec<Position>> = HashMap::new();
    let max_row = string.lines().count();
    let max_col = string.lines().next().unwrap().chars().count();

    string
        .trim_end()
        .split("\n")
        .enumerate()
        .for_each(|(row, raw_line)| {
            raw_line.chars().enumerate().for_each(|(col, raw_char)| {
                let _ = antenna_map.insert(
                    Position(row as isize, col as isize),
                    match raw_char {
                        '.' => MapState::Open,
                        '#' => MapState::Antinode,
                        _ => {
                            match freq_map.get_mut(&raw_char) {
                                Some(vec) => {
                                    vec.push(Position(row as isize, col as isize));
                                }
                                None => {
                                    let _ = freq_map.insert(
                                        raw_char,
                                        vec![Position(row as isize, col as isize)],
                                    );
                                }
                            };
                            MapState::Antenna
                        }
                    },
                );
            })
        });

    Data {
        antenna_map,
        freq_map,
        max_row,
        max_col,
    }
}

fn part1(data: &Data) {
    //For doing unique impl
    let mut used: HashMap<Position, ()> = HashMap::new();

    let final_result = data
        .freq_map
        .clone() //To lazy to figure out the ownership issues
        .into_iter()
        .flat_map(|(_, positions)| calc_antinodes(positions.as_slice()))
        .filter(|antinode_pos| data.antenna_map.contains_key(antinode_pos))
        .filter(|pos| used.insert(*pos, ()).is_none())
        .count();

    println!("Day  Part 1 result: {final_result}");
}

fn part2(data: &Data) {
    //For doing unique impl
    let mut used: HashMap<Position, ()> = HashMap::new();
    // let max_row = data.antenna_map.len();
    // let max_col = data.antenna_map[0].len();

    let final_result = data
        .freq_map
        .clone() //To lazy to figure out the ownership issues
        .into_iter()
        .flat_map(|(_, positions)| {
            calc_antinodes_v2(
                positions.as_slice(),
                data.max_row as isize,
                data.max_col as isize,
            )
        })
        .filter(|pos| used.insert(*pos, ()).is_none())
        .count();

    println!("Day  Part 1 result: {final_result}");
}

fn calc_antinodes(positions: &[Position]) -> Vec<Position> {
    let mut antinodes = vec![];
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (pos1, pos2) = (positions[i], positions[j]);
            let (dx, dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            antinodes.push(Position(pos1.0 - dx, pos1.1 - dy));
            antinodes.push(Position(pos2.0 + dx, pos2.1 + dy));
        }
    }
    antinodes
}

fn calc_antinodes_v2(positions: &[Position], max_row: isize, max_col: isize) -> Vec<Position> {
    let mut antinodes = vec![];
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (pos1, pos2) = (positions[i], positions[j]);
            let (dx, dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            antinodes.extend(get_antinodes_in_range(&pos1, -dx, -dy, max_row, max_col));
            antinodes.extend(get_antinodes_in_range(&pos2, dx, dy, max_row, max_col));
        }
    }
    antinodes
}

fn get_antinodes_in_range(
    pos: &Position,
    delta_row: isize,
    delta_col: isize,
    max_row: isize,
    max_col: isize,
) -> Vec<Position> {
    let mut antinodes = vec![];
    let (mut x, mut y) = (pos.0, pos.1);
    while x >= 0 && y >= 0 && x < max_row && y < max_col {
        antinodes.push(Position(x, y));
        x += delta_row;
        y += delta_col;
    }
    antinodes
}
