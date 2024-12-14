use std::{fs::File, io::Read};

fn main() {
    let mut string = String::new();
    match File::open("day11/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(&data);
    part2();
}

fn parse_data(string: &str) -> Vec<u64> {
    string
        .trim_end()
        .split(" ")
        .map(|raw| raw.parse().expect("Could not identify nubmer"))
        .collect()
}

fn part1(data: &Vec<u64>) {
    //create local copy for ops
    let stones = check_stones_rules(&data, 25);

    let final_result = stones.len();
    println!("Day  Part 1 result: {final_result}");
}

fn check_stones_rules(stones: &Vec<u64>, iter_left: u64) -> Vec<u64> {
    if iter_left == 0 {
        return stones.to_owned();
    }

    let mut new_stones = Vec::new();
    for stone in stones {
        if *stone == 0 {
            //becomes 1
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            dbg!(stone);
            //split the stone
            let stone_str = stone.to_string();
            let (new1, new2) = stone_str.split_at(stone_str.len() / 2 );
            new_stones.push(new1.parse().expect("split 1 bad"));
            new_stones.push(new2.parse().expect("split 2 bad"));
        } else {
            //replace with stone * 2024
            new_stones.push(stone * 2024);
        }
    }
    check_stones_rules(&new_stones, iter_left - 1)
}

fn part2() {
    // println!("Day  Part 2 result: {final_result}");
}
