use std::{
    cmp::{max, min},
    fmt::Display,
};

use crate::read_lines;

pub fn day11() {
    let path = "data/day11.txt";
    let sum = part1(path);
    println!("Day 11 Part 1 {}", sum);
    let sum = part2(path, 1_000_000);
    println!("Day 11 Part 2 {}", sum);
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);

    let universe = convert_to_galaxy(&lines);

    let universe = expand_universe(universe);

    let galaxy_indecies = find_indecies(&universe);

    let distances = find_shortest_distances(&galaxy_indecies);

    let sum = distances.iter().sum();

    return sum;
}

fn part2(path: &str, scale_factor: i64) -> i64 {
    let lines = read_lines(path);

    let universe = convert_to_galaxy(&lines);

    let galaxy_indecies = find_indecies(&universe);

    let distances = find_shortest_distances_alt(&universe, &galaxy_indecies, scale_factor);

    let sum = distances.iter().sum();

    return sum;
}

fn find_shortest_distances_alt(
    universe: &Vec<Vec<Universe>>,
    index: &Vec<(i64, i64)>,
    scale_factor: i64,
) -> Vec<i64> {
    let mut out = vec![];
    let len = index.len();

    for (j, galaxy) in index.iter().enumerate() {
        for i in j..len {
            let comp_galaxy = &index[i];

            if galaxy == comp_galaxy {
                continue;
            }

            let row_min = min(galaxy.0, comp_galaxy.0);
            let row_max = max(galaxy.0, comp_galaxy.0);

            let col_min = min(galaxy.1, comp_galaxy.1);
            let col_max = max(galaxy.1, comp_galaxy.1);

            let empty_rows = count_blank_rows(universe, row_min as usize, row_max as usize);
            let empty_columns = count_blank_columns(universe, col_min as usize, col_max as usize);

            let new_distance = (galaxy.0 - comp_galaxy.0).abs()
                + (galaxy.1 - comp_galaxy.1).abs()
                + (scale_factor - 1) * empty_rows
                + (scale_factor - 1) * empty_columns;

            out.push(new_distance);
        }
    }

    return out;
}

fn find_shortest_distances(index: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut out = vec![];
    let len = index.len();

    for (j, galaxy) in index.iter().enumerate() {
        for i in j..len {
            let comp_galaxy = &index[i];

            if galaxy == comp_galaxy {
                continue;
            }

            let new_distance = (galaxy.0 - comp_galaxy.0).abs() + (galaxy.1 - comp_galaxy.1).abs();

            out.push(new_distance);
        }
    }

    return out;
}

#[derive(PartialEq, Clone, Copy)]
enum Universe {
    Void,
    Galaxy,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Galaxy => write!(f, "#"),
            Self::Void => write!(f, "."),
        }
    }
}

fn find_indecies(universe: &Vec<Vec<Universe>>) -> Vec<(i64, i64)> {
    let mut output = vec![];

    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] == Universe::Galaxy {
                output.push((i as i64, j as i64));
            }
        }
    }

    return output;
}

fn convert_to_galaxy(lines: &Vec<String>) -> Vec<Vec<Universe>> {
    let mut out = vec![];

    for line in lines {
        let mut tmp = vec![];

        for c in line.chars() {
            if c == '.' {
                tmp.push(Universe::Void);
            } else if c == '#' {
                tmp.push(Universe::Galaxy);
            } else {
                panic!("Unknown Input {}", c);
            }
        }

        out.push(tmp);
    }

    return out;
}

fn expand_universe(universe: Vec<Vec<Universe>>) -> Vec<Vec<Universe>> {
    let height = universe.len();
    let width = universe[0].len();

    let mut rows_added = 0;
    let mut columns_added = 0;

    let rows_to_add = find_empty_rows(&universe);
    let columns_to_add = find_empty_columns(&universe);

    let mut new = universe.clone();

    for additional_row in rows_to_add {
        new.insert(additional_row + rows_added, vec![Universe::Void; width]);
        rows_added += 1;
    }

    for additional_column in columns_to_add {
        let column_index = additional_column + columns_added;

        for i in 0..new.len() {
            new[i].insert(column_index, Universe::Void);
        }

        columns_added += 1;
    }

    return new;
}

fn count_blank_rows(universe: &Vec<Vec<Universe>>, start: usize, end: usize) -> i64 {
    let mut sum = 0;
    for i in start..=end {
        if is_every_element_void(&universe[i]) {
            sum += 1;
        }
    }

    return sum;
}

fn count_blank_columns(universe: &Vec<Vec<Universe>>, start: usize, end: usize) -> i64 {
    let mut sum = 0;

    let len = universe.len();

    for j in start..=end {
        let mut tmp = true;
        for i in 0..len {
            if universe[i][j] == Universe::Galaxy {
                tmp = false;
                break;
            }
        }

        if tmp {
            sum += 1;
        }
    }

    return sum;
}

fn display_universe(universe: &Vec<Vec<Universe>>) {
    for row in universe {
        for el in row {
            print!("{}", el);
        }
        println!("\n");
    }
}

fn find_empty_rows(universe: &Vec<Vec<Universe>>) -> Vec<usize> {
    let mut out = vec![];
    for (i, row) in universe.iter().enumerate() {
        if is_every_element_void(row) {
            out.push(i);
        }
    }

    return out;
}

fn find_empty_columns(universe: &Vec<Vec<Universe>>) -> Vec<usize> {
    let mut out = vec![];

    let height = universe.len();
    let width = universe[0].len();

    for j in 0..width {
        let mut tmp = true;
        for i in 0..height {
            if universe[i][j] == Universe::Galaxy {
                tmp = false;
                break;
            }
        }

        if tmp {
            out.push(j);
        }
    }

    return out;
}

fn is_every_element_void(line: &Vec<Universe>) -> bool {
    for c in line {
        if *c == Universe::Galaxy {
            return false;
        }
    }

    return true;
}

#[test]
fn test_part1() {
    let path = "data_demo/day11_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 374);
}

#[test]
fn test_part2() {
    let path = "data_demo/day11_demo.txt";
    let sum = part2(path, 10);
    assert_eq!(sum, 1030);
    let sum = part2(path, 100);
    assert_eq!(sum, 8410);
}
