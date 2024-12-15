use std::{fs::File, io::Read};

fn main() {
    let mut string = String::new();
    match File::open("day12/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(&data);
    part2(&data);
}

fn parse_data(string: &str) -> Vec<Vec<char>> {
    string
        .trim_end()
        .split("\n")
        .map(|raw| raw.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(data: &Vec<Vec<char>>) {
    let mut plots = Vec::new();
    let mut garden = data.clone();

    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            if garden[row][col].is_uppercase() {
                let target = garden[row][col].clone();
                plots.push(floodfill(&mut garden, target, (row, col)));
            }
        }
    }

    let final_result = plots.into_iter().fold(0, |acc, e| {
        acc + (e.spaces.len() as u64 * e.calc_perimeter())
    });

    println!("Day 12 Part 1 result: {final_result}");
}

struct Plot {
    spaces: Vec<(usize, usize)>,
}

impl Plot {
    pub fn add(&mut self, other: &mut Plot) {
        self.spaces.append(&mut other.spaces);
    }

    pub fn calc_perimeter(&self) -> u64 {
        //clone the vec but cast to isize for convenience
        let spaces_isize: Vec<(isize, isize)> = self
            .spaces
            .iter()
            .map(|e| (e.0 as isize, e.1 as isize))
            .collect();
        let mut perimeter = 0;
        for (row, col) in &spaces_isize {
            if !spaces_isize.contains(&(row - 1, *col)) {
                perimeter += 1;
            }
            if !spaces_isize.contains(&(row + 1, *col)) {
                perimeter += 1;
            }
            if !spaces_isize.contains(&(*row, col - 1)) {
                perimeter += 1;
            }
            if !spaces_isize.contains(&(*row, col + 1)) {
                perimeter += 1;
            }
        }
        perimeter
    }

    pub fn count_corners(&self) -> u64 {
        //clone the vec but cast to isize for convenience
        let spaces_isize: Vec<(isize, isize)> = self
            .spaces
            .iter()
            .map(|e| (e.0 as isize, e.1 as isize))
            .collect();
        let mut corners = 0;
        for (row, col) in &spaces_isize {
            //outer corner checks
            //ul
            if !spaces_isize.contains(&(row - 1, *col))
                && !spaces_isize.contains(&(row - 1, col - 1))
                && !spaces_isize.contains(&(*row, col - 1))
            {
                corners += 1;
            }
            //ur
            if !spaces_isize.contains(&(row - 1, *col))
                && !spaces_isize.contains(&(row - 1, col + 1))
                && !spaces_isize.contains(&(*row, col + 1))
            {
                corners += 1;
            }
            //dr
            if !spaces_isize.contains(&(*row, col + 1))
                && !spaces_isize.contains(&(row + 1, col + 1))
                && !spaces_isize.contains(&(row + 1, *col))
            {
                corners += 1;
            }
            //dl
            if !spaces_isize.contains(&(row + 1, *col))
                && !spaces_isize.contains(&(row + 1, col - 1))
                && !spaces_isize.contains(&(*row, col - 1))
            {
                corners += 1;
            }

            //inner corner checks
            //ul-inner
            if !spaces_isize.contains(&(*row, col - 1))
                && spaces_isize.contains(&(row - 1, col - 1))
            {
                corners += 1;
            }
            //ur-inner
            if !spaces_isize.contains(&(*row, col + 1))
                && spaces_isize.contains(&(row - 1, col + 1))
            {
                corners += 1;
            }
            //dr-inner
            if !spaces_isize.contains(&(*row, col + 1))
                && spaces_isize.contains(&(row + 1, col + 1))
            {
                corners += 1;
            }
            //dl-inner
            if !spaces_isize.contains(&(*row, col - 1))
                && spaces_isize.contains(&(row + 1, col - 1))
            {
                corners += 1;
            }
        }
        corners
    }

    pub fn count_edges(&self) -> u64 {
        //Eulers polyhedra formula V+F = E+2
        // F=1 (2D), so reduces to V - 1 = E, but one off error intests?
        self.count_corners()
    }
}

fn floodfill(
    grid: &mut Vec<Vec<char>>,
    // visited: &mut Vec<(usize, usize)>,
    target: char,
    coords: (usize, usize),
) -> Plot {
    let (row, col) = coords;
    let mut plot = Plot { spaces: Vec::new() };
    //Up
    if grid[row][col] == target {
        //Do I need this naymore?
        grid[row][col] = target.to_ascii_lowercase();
        plot.spaces.push((row, col));
        //Up
        if let Some(new_row) = row.checked_sub(1) {
            plot.add(&mut floodfill(grid, target, (new_row, col)));
        }
        //Down
        if row + 1 < grid.len() {
            plot.add(&mut floodfill(grid, target, (row + 1, col)));
        }
        //Up
        if let Some(new_col) = col.checked_sub(1) {
            plot.add(&mut floodfill(grid, target, (row, new_col)));
        }
        //Right
        if col + 1 < grid[0].len() {
            plot.add(&mut floodfill(grid, target, (row, col + 1)));
        }
    }

    plot
}

fn part2(data: &Vec<Vec<char>>) {
    let mut plots = Vec::new();
    let mut garden = data.clone();

    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            if garden[row][col].is_uppercase() {
                let target = garden[row][col].clone();
                plots.push(floodfill(&mut garden, target, (row, col)));
            }
        }
    }

    let final_result = plots
        .into_iter()
        .fold(0, |acc, e| acc + (e.spaces.len() as u64 * e.count_edges()));

    println!("Day 12 Part 2 result: {final_result}");
}

#[cfg(test)]
mod tests {
    use crate::{floodfill, parse_data};

    #[test]
    fn floodfill_basic() {
        let mut garden = parse_data("AAAA\nBBCD\nBBCC\nEEEC");
        let mut plots = Vec::new();

        for row in 0..garden.len() {
            for col in 0..garden[0].len() {
                if garden[row][col].is_uppercase() {
                    let target = garden[row][col].clone();
                    plots.push(floodfill(&mut garden, target, (row, col)));
                }
            }
        }

        //Check everything has been assigned a plot
        assert_eq!(5, plots.len());
        assert!(garden.into_iter().fold(true, |acc, row| {
            acc && row
                .into_iter()
                .fold(true, |acc, char| acc && char.is_lowercase())
        }));

        let plot1 = &plots[0];
        assert_eq!(4, plot1.spaces.len());
        assert_eq!(10, plot1.calc_perimeter());
        assert_eq!(vec![(0, 0), (0, 1), (0, 2), (0, 3)], plot1.spaces);
    }

    #[test]
    fn edge_count_e() {
        let mut garden = parse_data("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE");
        let mut plots = Vec::new();

        for row in 0..garden.len() {
            for col in 0..garden[0].len() {
                if garden[row][col].is_uppercase() {
                    let target = garden[row][col].clone();
                    plots.push(floodfill(&mut garden, target, (row, col)));
                }
            }
        }

        //Check everything has been assigned a plot
        assert_eq!(3, plots.len());

        let plot1 = &plots[0];
        assert_eq!(17, plot1.spaces.len());
        assert_eq!(12, plot1.count_corners());
        assert_eq!(12, plot1.count_edges());
    }
}
