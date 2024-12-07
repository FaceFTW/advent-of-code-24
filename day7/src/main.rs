use std::{fs::File, io::Read, iter::zip};

struct Equation {
    expected: u64,
    operands: Vec<u64>,
}

fn main() {
    let mut string = String::new();
    match File::open("day7/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(data.as_slice());
    part2();
}

fn parse_data(string: &str) -> Vec<Equation> {
    string
        .trim_end()
        .split("\n")
        .map(|raw| {
            let (expected_str, operands_str) = raw.split_once(":").unwrap();
            let expected = expected_str
                .parse()
                .expect("Didn't find a valid number for the result");

            let operands = operands_str
                .trim()
                .split(" ")
                .map(|num_str| {
                    num_str
                        .parse()
                        .expect("Didn't find a valid number for the operand")
                })
                .collect();
            Equation { expected, operands }
        })
        .collect()
}

fn part1(equations: &[Equation]) {
    let mut final_result = 0;

    for equation in equations {
        let op_vecs = create_operation_lists(equation.operands.len());
        match op_vecs.into_iter().fold(false, |acc, op_vec| {
            acc || zip(
                equation.operands.as_slice().into_iter(),
                op_vec.as_slice().into_iter(),
            )
            .fold(0, |acc, (operand, operation)| match operation {
                Operation::Add => acc + operand,
                Operation::Mult => acc * operand,
            }) == equation.expected
        }) {
            true => final_result = final_result + equation.expected,
            false => (),
        }
    }

    println!("Day 7 Part 1 result: {final_result}");
}

fn part2() {
    // println!("Day  Part 2 result: {final_result}");
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mult,
}

fn create_operation_lists(operand_count: usize) -> Vec<Vec<Operation>> {
    //Exclude first op which is always an add to allow for ez folding
    expand_op_vec(
        vec![vec![Operation::Add], vec![Operation::Mult]],
        operand_count - 1,
    )
    .into_iter()
    //NOTE This is not efficient obviously?
    .map(|op_vec| {
        let mut init = vec![Operation::Add];
        for op in op_vec {
            init.push(op);
        }
        init
    })
    .collect()
}

fn expand_op_vec(curr_vecs: Vec<Vec<Operation>>, iters_left: usize) -> Vec<Vec<Operation>> {
    if iters_left == 0 {
        curr_vecs
    } else {
        expand_op_vec(
            curr_vecs
                .into_iter()
                .flat_map(|op_vec| {
                    let mut add_vec = op_vec.clone();
                    let mut mult_vec = op_vec.clone();
                    add_vec.push(Operation::Add);
                    mult_vec.push(Operation::Mult);
                    vec![add_vec, mult_vec]
                })
                .collect(),
            iters_left - 1,
        )
    }
}
