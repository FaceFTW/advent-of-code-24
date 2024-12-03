use std::{fs::File, io::Read};

fn main() {
    let mut string = String::new();
    match File::open("day.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1();
    part2();
}

fn parse_data(string: &str) -> Vec {
    string.trim_end().split("\n").map(|raw| {}).collect()
}

fn part1() {}

fn part2() {}
