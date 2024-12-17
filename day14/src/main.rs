use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
use std::io;
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
    pub fn tick_single(&mut self) {
        self.pos.0 = (self.pos.0 + self.vel.0 + 101) % 101;
        self.pos.1 = (self.pos.1 + self.vel.1 + 103) % 103;
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
    part2(&data);
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

//interactive brute force
fn part2(data: &Vec<Robot>) {
    let mut local_data = data.clone().to_owned();
    let mut iter_count = 0;
    //Simulate robots
    loop {
        let mut stdinput = String::new();
        for robot in local_data.iter_mut() {
            robot.tick_single();
        }
        if check_for_robots_in_a_line(&local_data.as_slice()) {
            print_robots(local_data.as_slice());
            println!("Is it a tree yet? y/N");

            match io::stdin().read_line(&mut stdinput) {
                Ok(_) => match stdinput.to_lowercase().as_str().contains("y") {
                    true => {
                        println!("Day 14 Part 2 Result: {iter_count}");
                        break;
                    }
                    false => (),
                },
                Err(_) => panic!("No Stdin"),
            }
        }
            iter_count += 1;
    }
}

fn print_robots(robots: &[Robot]) {
    let mut grid = vec![vec![0; 101]; 103];
    for robot in robots {
        let (x, y) = robot.pos;
        grid[y as usize][x as usize] += 1;
    }
    for row in grid {
        let row_str = row.into_iter().fold(String::new(), |acc, num| match num {
            0 => acc + ".",
            _ => acc + num.to_string().as_str(),
        });
        println!("{row_str}");
    }
}

fn check_for_robots_in_a_line(robots: &[Robot]) -> bool {
    let mut partitions: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    for robot in robots {
        match partitions.get_mut(&robot.pos.0) {
            Some(vec) => vec.push(robot.pos),
            None => {
                let _ = partitions.insert(robot.pos.0, vec![robot.pos]);
            }
        }
    }

    partitions.iter_mut().fold(false, |acc, (_, vec)| {
        acc || if vec.len() < 10 {
            false
        } else {
            vec.sort_by(|a, b| a.1.cmp(&b.1));
            vec.windows(10).fold(false, |acc, window| {
                acc || window
                    .windows(2)
                    .fold(true, |acc, pair| acc && (pair[1].1 == pair[0].1 + 1))
            })
        }
    })
}
