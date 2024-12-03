use std::{fs::File, io::Read, iter::zip};

fn main() {
    let mut string = String::new();
    match File::open("day1.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    part1(string.as_str());
}

fn part1(string: &str) {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = string
        .trim_end() //Prevent iterator from splitting at the end
        .split("\n")
        .map(|line| {
            dbg!(&line);
            //Assertions
            //1. Numbers are always divided by 3 spaces
            //2. Only two numbers per line
            let nums: Vec<&str> = line.split("   ").collect();
            (
                nums[0]
                    .parse::<u64>()
                    .expect("First item was not a number!"),
                nums[1]
                    .parse::<u64>()
                    .expect("Second item was not a number!"),
            )
        })
        .unzip();

    //Sure, Add O(*) sorting overhead but not performance optimized just want something that works
    left.sort();
    right.sort();

    //Rezip the sorted iterators, then reduce
    let final_res =
        zip(left.as_slice().into_iter(), right.as_slice().into_iter()).fold(0u64, |acc, pair| {
            let diff = u64::abs_diff(pair.0.clone(), pair.1.clone());
            acc + diff
        });

    println!("Day 1 Part 1 Result w/ day1.txt: {final_res}");
}
