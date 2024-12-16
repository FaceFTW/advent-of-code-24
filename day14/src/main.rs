use std::cmp::Ordering::{Greater, Less};
use std::{fs::File, io::Read};

use regex::Regex;

//NOTE code makes assumptions that w=101 and h=103, that won't change significantly

#[derive(Clone, Debug)]
struct Robot {
    pos: (i64, i64), //could be u64 but rather not worry about typecasting
    vel: (i64, i64),
}

impl Robot {
    pub fn tick(&mut self, count: i64) {
        self.pos.0 = (self.pos.0 + (self.vel.0 * count) + (101 * count)) % 101;
        self.pos.1 = (self.pos.1 + (self.vel.1 * count) + (103 * count)) % 103;
    }
}

fn main() {
    let mut string = String::new();
    match File::open("day14/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(&data);
    part2();
}

fn parse_data(string: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    string
        .trim_end()
        .split("\n")
        .map(|raw| {
            let cap = re.captures(raw).unwrap();
            let pos = (
                cap.get(1).unwrap().as_str().parse().unwrap(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            );
            let vel = (
                cap.get(3).unwrap().as_str().parse().unwrap(),
                cap.get(4).unwrap().as_str().parse().unwrap(),
            );
            Robot { pos, vel }
        })
        .collect()
}

fn part1(data: &Vec<Robot>) {
    let mut local_data = data.clone().to_owned();

    //Simulate robots
    // for _ in 0..100 {
    for robot in local_data.iter_mut() {
        robot.tick(100);
    }
    // }

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in local_data {
        let (x, y) = robot.pos;

        match (x.cmp(&(101 / 2)), y.cmp(&(103 / 2))) {
            (Less, Less) => q1 += 1,
            (Less, Greater) => q2 += 1,
            (Greater, Less) => q3 += 1,
            (Greater, Greater) => q4 += 1,
            _ => (),
        }
    }

    let final_result = q1 * q2 * q3 * q4;
    println!("Day 14 Part 1 result: {final_result}");
}

fn part2() {

    // println!("Day 14 Part 2 result: {final_result}");
}
