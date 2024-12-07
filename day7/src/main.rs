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
    part2(&data.as_slice());
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
        for op_vec in op_vecs.into_iter() {
            match zip(
                equation.operands.as_slice().into_iter(),
                op_vec.as_slice().into_iter(),
            )
            .fold(0, |acc, (operand, operation)| {
                match operation {
                    Operation::Add => acc + operand,
                    Operation::Mult => acc * operand,
                    Operation::Concat => acc, //Do nothing
                }
            }) == equation.expected
            {
                true => {
                    final_result = final_result + equation.expected;
                    break;
                }
                false => (),
            }
        }
    }

    println!("Day 7 Part 1 result: {final_result}");
}

fn part2(equations: &[Equation]) {
    let mut final_result = 0;

    for equation in equations {
        let op_vecs = create_operation_lists(equation.operands.len());
        for op_vec in op_vecs.into_iter() {
            match zip(
                equation.operands.as_slice().into_iter(),
                op_vec.as_slice().into_iter(),
            )
            .fold(0, |acc, (operand, operation)| match operation {
                Operation::Add => acc + operand,
                Operation::Mult => acc * operand,
                Operation::Concat => acc * 10u64.pow(operand.ilog10() + 1) + operand,
            }) == equation.expected
            {
                true => {
                    final_result = final_result + equation.expected;
                    break;
                }
                false => (),
            }
        }
    }

    println!("Day 7 Part 2 result: {final_result}");
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mult,
    Concat,
}

// fn create_operation_lists_no_concat(operand_count: usize) -> Vec<Vec<Operation>> {
//     //Exclude first op which is always an add to allow for ez folding
//     expand_op_vec_no_concat(
//         vec![vec![Operation::Add], vec![Operation::Mult]],
//         operand_count - 1,
//     )
//     .into_iter()
//     //NOTE This is not efficient obviously?
//     .map(|op_vec| {
//         let mut init = vec![Operation::Add];
//         for op in op_vec {
//             init.push(op);
//         }
//         init
//     })
//     .collect()
// }

// fn expand_op_vec_no_concat(
//     curr_vecs: Vec<Vec<Operation>>,
//     iters_left: usize,
// ) -> Vec<Vec<Operation>> {
//     if iters_left == 0 {
//         curr_vecs
//     } else {
//         expand_op_vec_no_concat(
//             curr_vecs
//                 .into_iter()
//                 .flat_map(|op_vec| {
//                     let mut add_vec = op_vec.clone();
//                     let mut mult_vec = op_vec.clone();
//                     add_vec.push(Operation::Add);
//                     mult_vec.push(Operation::Mult);
//                     vec![add_vec, mult_vec]
//                 })
//                 .collect(),
//             iters_left - 1,
//         )
//     }
// }

fn create_operation_lists(operand_count: usize) -> Vec<Vec<Operation>> {
    //Exclude first op which is always an add to allow for ez folding
    expand_op_vec(
        vec![
            vec![Operation::Add],
            vec![Operation::Mult],
            vec![Operation::Concat],
        ],
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
                    let mut concat_vec = op_vec.clone();
                    add_vec.push(Operation::Add);
                    mult_vec.push(Operation::Mult);
                    concat_vec.push(Operation::Concat);
                    //Exclude concat vec if it is the last operation
                    // if iters_left == 1 {
                    //     vec![add_vec, mult_vec]
                    // } else {
                    vec![add_vec, mult_vec, concat_vec]
                    // }
                })
                .collect(),
            iters_left - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::{create_operation_lists, parse_data, Operation};

    #[test]
    pub fn test_example() {
        let example = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
        .trim();

        let equations = parse_data(&example);

        let mut final_result = 0;

        for equation in equations {
            let op_vecs = create_operation_lists(equation.operands.len());
            match op_vecs.into_iter().fold(false, |acc, op_vec| {
                let zipped_ops: Vec<(&u64, Operation)> =
                    zip(equation.operands.as_slice().into_iter(), op_vec.into_iter()).collect();
                acc || zipped_ops.into_iter().fold(0, |acc, (operand, operation)| {
                    match operation {
                        Operation::Add => acc + operand,
                        Operation::Mult => acc * operand,
                        Operation::Concat => {
                            acc * 10u64.pow((*operand as f64).log10().ceil() as u32) + operand
                        } //Do nothing
                    }
                }) == equation.expected
            }) {
                true => final_result = final_result + equation.expected,
                false => (),
            }
        }

        assert_eq!(11387, final_result);
    }
}
