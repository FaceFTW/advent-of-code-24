use std::{fs::File, io::Read};

use regex::Regex;

struct ClawMachine {
    a_offsets: (i64, i64),
    b_offsets: (i64, i64),
    prize_pos: (i64, i64),
}

impl ClawMachine {
    pub fn solve(&self) -> Option<i64> {
        //throwback to linear algebra - Cramer's Rule
        // System of 2 eqs w/ 2 unknowns
        // prize_x = (i * ax) + (j * bx)
        // prize_y = (i * ay) + (j * by)
        //
        // Apply Cramers Rule =>
        // i = |A_i|/|A| => ((prize_x * by) - (prize_y * bx)) / ((ax * by) - (ay * bx))
        // j = |A_j|/|A| => ((prize_y * ax) - (prize_x * ay)) / ((ax * by) - (ay * bx))
        //Will separate out determinant calcs to check if it's a whole number result
        let (ax, ay) = self.a_offsets;
        let (bx, by) = self.b_offsets;
        let (prize_x, prize_y) = self.prize_pos;

        let det = (ax * by) - (ay * bx);
        let det_i = (prize_x * by) - (prize_y * bx);
        let det_j = (prize_y * ax) - (prize_x * ay);

        match det_i % det == 0 && det_j % det == 0 {
            true => Some(3 * (det_i / det) + (det_j / det)),
            false => None, //Not a valid soln
        }
    }
    pub fn solve_v2(&self) -> Option<i64> {
        //same as normal but prize x,y + 10000000000000
        let (ax, ay) = self.a_offsets;
        let (bx, by) = self.b_offsets;
        let (prize_x, prize_y) = (
            self.prize_pos.0 + 10000000000000,
            self.prize_pos.1 + 10000000000000,
        );

        let det = (ax * by) - (ay * bx);
        let det_i = (prize_x * by) - (prize_y * bx);
        let det_j = (prize_y * ax) - (prize_x * ay);

        match det_i % det == 0 && det_j % det == 0 {
            true => Some(3 * (det_i / det) + (det_j / det)),
            false => None, //Not a valid soln
        }
    }
}

fn main() {
    let mut string = String::new();
    match File::open("day13/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(data.as_slice());
    part2(data.as_slice());
}

fn parse_data(string: &str) -> Vec<ClawMachine> {
    //Listen, I like regex so shut up
    let re_str = r"Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)";
    let re = Regex::new(&re_str).unwrap();

    string
        .trim_end()
        .split("\n\n")
        .map(|raw| {
            let matches = re.captures(raw).unwrap();

            let a_offsets = (
                matches
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad regex a0"),
                matches
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad regex a1"),
            );

            let b_offsets = (
                matches
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad regex a0"),
                matches
                    .get(4)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad regex a1"),
            );

            let prize_pos = (
                matches
                    .get(5)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad regex a0"),
                matches
                    .get(6)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad regex a1"),
            );

            ClawMachine {
                a_offsets,
                b_offsets,
                prize_pos,
            }
        })
        .collect()
}

fn part1(data: &[ClawMachine]) {
    let final_result = data
        .into_iter()
        .fold(0, |acc, machine| match machine.solve() {
            Some(tokens) => acc + tokens,
            None => acc,
        });

    println!("Day 13 Part 1 result: {final_result}");
}

fn part2(data: &[ClawMachine]) {
    let final_result = data
        .into_iter()
        .fold(0, |acc, machine| match machine.solve_v2() {
            Some(tokens) => acc + tokens,
            None => acc,
        });
    println!("Day 13 Part 2 result: {final_result}");
}
