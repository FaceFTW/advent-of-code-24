//Ah, the "regex" problem
//Thank god I'm not terrible with it

use regex::Regex;
use std::{fs::File, io::Read, u64};

enum Instr {
    MultInstr(u64, u64),
    DoInstr,
    DontInstr,
}

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
    part2(&string.trim_end());
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
    let nums: Vec<(u64, u64)> = re
        .captures_iter(memory)
        .map(|data| {
            (
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

fn part2(memory: &str) {
    let re =
        Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don\'t\(\)").expect("regex 🅱️roke, sorry :(");
    let nums: Vec<Instr> = re
        .captures_iter(memory)
        .map(|data| {
            //check the first chars before the (
            let instr_prefix = data.get(0).unwrap().as_str().split_once("(").unwrap().0;
            match instr_prefix {
                "do" => Instr::DoInstr,
                "don't" => Instr::DontInstr,
                "mul" => Instr::MultInstr(
                    data.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    data.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                ),
                _ => panic!("regex is 🅱️ad :("),
            }
        })
        .collect();

    let mut enable_mul = true;
    let final_result = nums.into_iter().fold(0u64, |acc, instr| match instr {
        Instr::MultInstr(a, b) => match enable_mul {
            true => acc + (a * b),
            false => acc,
        },
        Instr::DoInstr => {
            enable_mul = true;
            acc
        }
        Instr::DontInstr => {
            enable_mul = false;
            acc
        }
    });
    println!("Day 3 Part 2 result: {final_result}");
}
