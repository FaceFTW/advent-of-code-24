use std::{fs::File, io::Read};

fn main() {
    let mut string = String::new();
    match File::open("day4/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(data.as_slice());
    part2();
}

fn parse_data(string: &str) -> Vec<Vec<char>> {
    string
        .trim_end()
        .split("\n")
        .map(|raw| raw.chars().collect())
        .collect()
}

fn part1(data: &[Vec<char>]) {
    let mut final_result = 0;
    final_result = final_result + count_fwds_bkwds(data);
    final_result = final_result + count_up_down(data);
    final_result = final_result + count_diagonals(data);

    println!("Day 4 Part 1 result: {final_result}");
}

fn part2() {

    // println!("Day 4 Part 2 result: {final_result}");
}

fn count_fwds_bkwds(data: &[Vec<char>]) -> u64 {
    let mut count: u64 = 0;
    for row in data {
        for chunk in row.windows(4) {
            if (chunk[0] == 'X' && chunk[1] == 'M' && chunk[2] == 'A' && chunk[3] == 'S')
                || (chunk[0] == 'S' && chunk[1] == 'A' && chunk[2] == 'M' && chunk[3] == 'X')
            {
                count = count + 1;
            }
        }
    }
    count
}

fn count_up_down(data: &[Vec<char>]) -> u64 {
    let mut count: u64 = 0;
    for row_chunk in data.windows(4) {
        //Assertion, row len is same
        for idx in 0..row_chunk[0].len() {
            if (row_chunk[0][idx] == 'X'
                && row_chunk[1][idx] == 'M'
                && row_chunk[2][idx] == 'A'
                && row_chunk[3][idx] == 'S')
                || (row_chunk[0][idx] == 'S'
                    && row_chunk[1][idx] == 'A'
                    && row_chunk[2][idx] == 'M'
                    && row_chunk[3][idx] == 'X')
            {
                count = count + 1;
            }
        }
    }
    count
}

fn count_diagonals(data: &[Vec<char>]) -> u64 {
    let mut count: u64 = 0;
    for row_chunk in data.windows(4) {
        //Assertion, row len is the same
        let chunked_rows: Vec<Vec<&[char]>> = row_chunk
            .into_iter()
            .map(|row| row.windows(4).collect())
            .collect();
        //quite a bit of "indirection" here
        // first idx is the row
        // second idx is the "chunk" in the rows
        // third is the idx in the chunk
        // dbg!(&chunked_rows);
        for idx in 0..chunked_rows[0].len() {
            //Bugfix - multiple diagonals in one chunk so check one diagonal at a time
            //shoutout https://www.reddit.com/r/adventofcode/comments/1h6otn5/2024_day_4_part_1_rust_i_cant_figure_out_if_im/

            //Forwards Diagonal
            if chunked_rows[0][idx][0] == 'X'
                && chunked_rows[1][idx][1] == 'M'
                && chunked_rows[2][idx][2] == 'A'
                && chunked_rows[3][idx][3] == 'S'
            {
                count = count + 1;
            }
            if chunked_rows[0][idx][0] == 'S'
                && chunked_rows[1][idx][1] == 'A'
                && chunked_rows[2][idx][2] == 'M'
                && chunked_rows[3][idx][3] == 'X'
            {
                count = count + 1;
            }

            //backwards diagonal
            if chunked_rows[0][idx][3] == 'X'
                && chunked_rows[1][idx][2] == 'M'
                && chunked_rows[2][idx][1] == 'A'
                && chunked_rows[3][idx][0] == 'S'
            {
                count = count + 1
            }
            if chunked_rows[0][idx][3] == 'S'
                && chunked_rows[1][idx][2] == 'A'
                && chunked_rows[2][idx][1] == 'M'
                && chunked_rows[3][idx][0] == 'X'
            {
                count = count + 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{count_diagonals, parse_data};

    #[test]
    fn check_diag_counting() {
        let input = r"
S..S
.AA.
.MM.
X..X
        ";

        let data = parse_data(input);
        assert_eq!(count_diagonals(data.as_slice()), 2);
    }
}
