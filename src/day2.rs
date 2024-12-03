use std::{fs::File, io::Read};

//Useful Alias
type Report = Vec<u64>;

fn main() {
    let mut string = String::new();
    match File::open("day2.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let reports = parse_reports(string.as_str());

    day1(reports.as_slice());
    day2(reports.as_slice());
}

fn parse_reports(string: &str) -> Vec<Report> {
    string
        .trim_end()
        .split("\n")
        .map(|raw| {
            raw.split(" ")
                .map(|e| e.parse::<u64>().expect("Report had some bad data!"))
                .collect()
        })
        .collect()
}

#[derive(PartialEq)]
enum Trend {
    Increasing = 0,
    Decreasing = 1,
}

const UNSAFE_MAX_THRESH: u64 = 3;
const UNSAFE_MIN_THRESH: u64 = 1; //More flexible than eq check

fn day1(reports: &[Report]) {
    let mut safe_count = 0;
    for report in reports {
        //Assertions
        //Reports have at least two levels

        //Take the first two levels
        let (first, second) = (report[0], report[1]);
        //establish trend
        let trend = match first > second {
            true => Trend::Decreasing,
            false => Trend::Increasing,
        };

        //
        match report.windows(2).into_iter().fold(true, |acc, pair| {
            let (first, second) = (pair[0], pair[1]);
            let pair_trend = match first > second {
                true => Trend::Decreasing,
                false => Trend::Increasing,
            };

            let diff = u64::abs_diff(first, second);

            acc && (pair_trend == trend)
                && (diff >= UNSAFE_MIN_THRESH)
                && (diff <= UNSAFE_MAX_THRESH)
        }) {
            true => safe_count = safe_count + 1,
            false => (),
        };
    }

    println!("Results for Day 2 Part 1 using day2.txt: {safe_count}");
}

//Most of the Same, except the last fold operation is _slightly_ different
fn day2(reports: &[Report]) {
    let mut safe_count = 0;
    for report in reports {
        //Assertions
        //Reports have at least two levels

        //Take the first two levels
        let (first, second) = (report[0], report[1]);
        //establish trend
        let trend = match first > second {
            true => Trend::Decreasing,
            false => Trend::Increasing,
        };

        //
        match report.windows(2).into_iter().fold(true, |acc, pair| {
            let (first, second) = (pair[0], pair[1]);
            let pair_trend = match first > second {
                true => Trend::Decreasing,
                false => Trend::Increasing,
            };

            let diff = u64::abs_diff(first, second);

            acc && (pair_trend == trend)
                && (diff >= UNSAFE_MIN_THRESH)
                && (diff <= UNSAFE_MAX_THRESH)
        }) {
            true => safe_count = safe_count + 1,
            false => {
                //Brute force but remove levels one at a time
                match (0..report.len()).fold(false, |acc, test| {
                    let mut temp_report = report.clone();
                    temp_report.remove(test);

                    //Take the first two levels again
                    let (first, second) = (temp_report[0], temp_report[1]);
                    //establish trend (shadowed)
                    let trend = match first > second {
                        true => Trend::Decreasing,
                        false => Trend::Increasing,
                    };
                    acc || temp_report.windows(2).into_iter().fold(true, |acc, pair| {
                        let (first, second) = (pair[0], pair[1]);
                        let pair_trend = match first > second {
                            true => Trend::Decreasing,
                            false => Trend::Increasing,
                        };

                        let diff = u64::abs_diff(first, second);

                        acc && (pair_trend == trend)
                            && (diff >= UNSAFE_MIN_THRESH)
                            && (diff <= UNSAFE_MAX_THRESH)
                    })
                }) {
                    true => safe_count = safe_count + 1,
                    false => (),
                }
            }
        };
    }

    println!("Results for Day 2 Part 2 using day2.txt: {safe_count}");
}
