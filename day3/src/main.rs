//Ah, the "regex" problem
//Thank god I'm not terrible with it

use regex::Regex;
use std::{fs::File, io::Read, u64};

struct MultInstr(u64, u64);

fn main() {
    let mut string = String::new();
    match File::open("day3/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    // let data = parse_data(string.as_str());

    part1(string.trim_end());
    part2();
}

// fn parse_data(string: &str) -> Vec<> {
//     string
//         .trim_end()
//         .split("\n")
//         .map(|raw| {

//         })
//         .collect()
// }

fn part1(memory: &str) {
    let re = Regex::new(r"mul\(([0-9]+)\,([0-9]+)\)").expect("regex 🅱️roke, sorry :(");
    let nums: Vec<MultInstr> = re
        .captures_iter(memory)
        .map(|data| {
            MultInstr(
                data.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                data.get(2).unwrap().as_str().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let final_result = nums
        .into_iter()
        .fold(0u64, |acc, instr| acc + (instr.0 * instr.1));

    println!("Day 3 Part 1 result: {final_result}");
}

fn part2() {
    // println!("Day 3 Part 2 result: {final_result}");
}
