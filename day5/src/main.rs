use std::{fs::File, io::Read, iter::once};

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
    part2(&data);
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

fn part2(data: &Data) {
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

        if !valid {
            // println!("Found invalid");
            // //Brute force permutations until we find something valid
            // for perm in update.iter().permutations(update.len()).unique() {
            //     // println!("trying permutaiton");
            //     let mut inner_valid = true;
            //     for check_idx in 0..update.len() {
            //         inner_valid = inner_valid
            //             && data
            //                 .line_rules
            //                 .as_slice()
            //                 .into_iter()
            //                 .filter(|rule| rule.0 == *perm[check_idx])
            //                 .fold(true, |acc, rule| {
            //                     acc && match perm.iter().position(|item| **item == rule.1) {
            //                         Some(pos) => check_idx < pos,
            //                         None => true,
            //                     }
            //                 })
            //     }
            //     if inner_valid {
            //         let midpoint = update.len() / 2;
            //         final_result = final_result + update[midpoint];

            //         break;
            //     }
            // }
            // println!("Fixed invalid");
            // let corrected = correct_update_list(
            //     update.as_slice(),
            //     &create_rules_map(data.line_rules.as_slice()),
            //     data.line_rules.as_slice(),
            // );

            let mut corrected = Vec::from(update.as_slice());
            corrected.sort_by(|&a, &b| {
                custom_compare(a, b, &make_rules_map(data.line_rules.as_slice()))
            });

            let midpoint = corrected.len() / 2;
            final_result = final_result + corrected[midpoint];
        }
    }

    println!("Day 5 Part 2 result: {final_result}");
}
//After fooling around with algorithms and getting frustrated, I decided to port over this solution from Timmoth but using
//my data structures. I just want to get back to Bioshock man :(
//Source: https://github.com/Timmoth/AdventofCode/blob/main/2024/src/solution_05_part2.rs

// fn is_in_order(values: &Vec<u64>, rules: &Vec<Vec<bool>>) -> bool {
//     for pair in values.windows(2) {
//         if !rules[pair[0] as usize][pair[1] as usize] {
//             return false;
//         }
//     }
//     true
// }

fn custom_compare(a: u64, b: u64, rules: &Vec<Vec<bool>>) -> std::cmp::Ordering {
    if rules[a as usize][b as usize] {
        std::cmp::Ordering::Less // a should come before b
    } else if rules[b as usize][a as usize] {
        std::cmp::Ordering::Greater // b should come before a
    } else {
        std::cmp::Ordering::Equal // a and b are equal according to the rules
    }
}

//This part I did write
fn make_rules_map(rules: &[(u64, u64)]) -> Vec<Vec<bool>> {
    //Find the highest possible number in both tuples
    let max = (rules
        .into_iter()
        .flat_map(|pair| once(pair.0).chain(once(pair.1)))
        .max()
        .unwrap()
        + 1) as usize;

    let mut map = vec![vec![false; max]; max];
    for rule in rules {
        map[rule.0 as usize][rule.1 as usize] = true;
    }
    map
}

// fn correct_update_list(
//     update: &[u64],
//     rules: &HashMap<u64, Vec<u64>>,
//     orig_rules: &[(u64, u64)],
// ) -> Vec<u64> {
//     let mut processed = vec![];
//     let mut bad_idx = None;
//     for idx in 0..update.len() {
//         let u = update[idx];
//         if rules.contains_key(&u) && !rules.get(&u).unwrap().is_empty() {
//             for rule in rules.get(&u).unwrap() {
//                 if update.contains(&u) && !processed.as_slice().contains(&u) {
//                     bad_idx = Some(idx);
//                     break;
//                 }
//             }
//             if bad_idx != None {
//                 break;
//             }
//         }
//         processed.push(u);
//     }
//     //Assert bad_idx is some
//     let e = update[bad_idx.unwrap()];
//     let rule = rules.get(&e).unwrap();
//     let mut relevant: Vec<_> = rule
//         .as_slice()
//         .into_iter()
//         .filter(|r| update.contains(r) && !processed.as_slice().contains(r))
//         .collect();
//     for i in bad_idx.unwrap() + 1..update.len() {
//         let u = update[i];
//         if let Some(pos) = relevant.as_slice().into_iter().position(|item| **item == u) {
//             relevant.remove(pos);
//             processed.push(u);
//             if relevant.is_empty() {
//                 processed.push(e);
//             }
//         } else {
//             processed.push(u);
//         }
//     }

//     //valid
//     if valid_update(processed.as_slice(), orig_rules) {
//         processed
//     } else {
//         correct_update_list(processed.as_slice(), &rules, &orig_rules)
//     }
// }

// fn valid_update(processed: &[u64], rules: &[(u64, u64)]) -> bool {
//     let mut valid = true;
//     for check_idx in 0..processed.len() {
//         valid = valid
//             && rules
//                 .into_iter()
//                 .filter(|rule| rule.0 == processed[check_idx])
//                 .fold(true, |acc, rule| {
//                     acc && match processed.iter().position(|item| *item == rule.1) {
//                         Some(pos) => check_idx < pos,
//                         None => true,
//                     }
//                 })
//     }
//     valid
// }

// fn create_rules_map(rules: &[(u64, u64)]) -> HashMap<u64, Vec<u64>> {
//     let mut map: HashMap<u64, Vec<u64>> = HashMap::new();
//     for pair in rules.as_slice() {
//         match map.contains_key(&pair.0) {
//             true => map.get_mut(&pair.0).unwrap().push(pair.1),
//             false => {
//                 let _ = map.insert(pair.0, vec![pair.1]);
//             }
//         }
//     }
//     map
// }
