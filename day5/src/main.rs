use std::{fs::File, io::Read};

#[derive(Debug)]
struct Data {
    line_rules: Vec<(u64, u64)>,
    line_updates: Vec<Vec<u64>>,
}

fn main() {
    let mut string = String::new();
    match File::open("day5/input.txt") {
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
    let split: Vec<&str> = string.trim_end().split("\n\n").collect();
    let line_rules = split[0]
        .split("\n")
        .map(|line| line.split_once("|").unwrap())
        .map(|pair| {
            (
                pair.0.parse().expect("Was not getting a number"),
                pair.1.parse().expect("Was not getting a number"),
            )
        })
        .collect();

    let line_updates = split[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|number| {
                    number
                        .parse::<u64>()
                        .expect("Was not finding an update number")
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Data {
        line_rules,
        line_updates,
    }
}

fn part1(data: &Data) {
    let mut final_result = 0;
    for update in data.line_updates.as_slice().into_iter() {
        let mut valid = true;
        for check_idx in 0..update.len() {
            valid = valid
                && data
                    .line_rules
                    .as_slice()
                    .into_iter()
                    .filter(|rule| rule.0 == update[check_idx])
                    .fold(true, |acc, rule| {
                        acc && match update.iter().position(|item| *item == rule.1) {
                            Some(pos) => check_idx < pos,
                            None => true,
                        }
                    })
        }

        if valid {
            let midpoint = update.len() / 2;
            final_result = final_result + update[midpoint];
        }
    }

    println!("Day 5 Part 1 result: {final_result}");
}

fn part2() {
    // println!("Day 5 Part 2 result: {final_result}");
}
