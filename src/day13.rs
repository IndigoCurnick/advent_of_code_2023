use std::fmt::Display;

use crate::read_lines;

pub fn day13() {
    let path = "data/day13.txt";
    let sum = part1(path);
    println!("Day 13 Part 1: {}", sum);
    let sum = part2(path);
    println!("Day 13 Part 2: {}", sum);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);

    let parsed_lines = parse_lines(&lines);

    let mut sum = 0;

    for block in parsed_lines {
        sum += handle_one_block(&block, None);
    }

    return sum;
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);

    let parsed_lines = parse_lines(&lines);

    let mut sum = 0;

    for block in parsed_lines {
        sum += part2_handle_one_block(&block);
    }

    return sum;
}

fn part2_handle_one_block(block: &Vec<Vec<Ground>>) -> usize {
    let old = handle_one_block(block, None);

    println!("The block about to be processed");
    display(block);

    for i in 0..block.len() {
        for j in 0..block[0].len() {
            let mut new = block.clone();
            new[i][j] = block[i][j].flip();

            println!("New Being Looked for ");
            display(&new);
            let proposed = handle_one_block(&new, Some(old));

            if proposed > 0 && proposed != old {
                println!("Smudge detected at {} {}", i, j);
                println!("Here is the symmetric smudge fixed");
                display(&new);
                return proposed;
            }
        }
    }

    panic!("BAd");
}

fn handle_one_block(block: &Vec<Vec<Ground>>, previous: Option<usize>) -> usize {
    // Horizontal symmetry

    let len = block.len();

    let mut sum = 0;

    // println!("About to process this block");
    // display(block);

    for i in 0..len - 1 {
        let lower_reflecting_line = i;
        let upper_reflecting_line = i + 1;

        let num_on_left = lower_reflecting_line;
        let num_on_right = len - upper_reflecting_line - 1;

        let padding = find_padding(num_on_right, num_on_left);

        let is_mirror = is_horizontally_symmetric(
            &block[lower_reflecting_line - padding..=upper_reflecting_line + padding],
        );

        if is_mirror {
            if previous.is_none() {
                println!("It's a mirror");
                println!("Lower reflecting line is {}", lower_reflecting_line);
                sum += (lower_reflecting_line + 1) * 100;
                break;
            }

            if previous.unwrap() != (lower_reflecting_line + 1) * 100 {
                println!("It's a mirror2");
                println!("Lower reflecting line is {}", lower_reflecting_line);
                sum += (lower_reflecting_line + 1) * 100;
                break;
            }
        }
    }

    if sum > 0 {
        println!("Sum is {}", sum);
        if previous.is_none() {
            println!("Returning sum of {}", sum);
            return sum;
        }

        println!("previous {}", previous.unwrap());
        if previous.unwrap() != sum {
            println!("Just unwrapped previous and it was different to sum");
            return sum;
        }
    }

    sum = 0;
    let grid = transpose(&block);

    let len = grid.len();

    for i in 0..len - 1 {
        let lower_reflecting_line = i;
        let upper_reflecting_line = i + 1;

        let num_on_left = lower_reflecting_line;
        let num_on_right = len - upper_reflecting_line - 1;

        let padding = find_padding(num_on_right, num_on_left);

        let is_mirror = is_vertically_symmetric(
            &grid[lower_reflecting_line - padding..=upper_reflecting_line + padding],
        );

        if is_mirror {
            if previous.is_none() {
                println!(
                    "Found vertical symmetry between cols {} and {}",
                    lower_reflecting_line, upper_reflecting_line
                );
                sum += lower_reflecting_line + 1;
                break;
            }

            if previous.unwrap() != lower_reflecting_line + 1 {
                println!(
                    "2 Found vertical symmetry between cols {} and {}",
                    lower_reflecting_line, upper_reflecting_line
                );
                sum += lower_reflecting_line + 1;
                break;
            }
        }
    }

    return sum;
}

fn transpose(grid: &[Vec<Ground>]) -> Vec<Vec<Ground>> {
    if grid.is_empty() || grid[0].is_empty() {
        // Handle empty grid case
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Create a new transposed grid with swapped dimensions
    let mut transposed_grid = vec![vec![Ground::Ash; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            // Swap rows and columns during transposition
            transposed_grid[j][i] = grid[i][j];
        }
    }

    return transposed_grid;
}

fn find_padding(a: usize, b: usize) -> usize {
    let min = a.min(b);

    // if min % 2 != 0 {
    //     return min - 1;
    // }

    return min;
}

fn is_horizontally_symmetric(block: &[Vec<Ground>]) -> bool {
    let rows = block.len();
    let cols = block[0].len();

    println!("Looking for horizontal symmetry in");
    display(block);

    for row in 0..rows / 2 {
        for col in 0..cols {
            if block[row][col] != block[rows - 1 - row][col] {
                return false;
            }
        }
    }
    println!("Found horizontal symmetry");
    return true;
}

fn is_vertically_symmetric(block: &[Vec<Ground>]) -> bool {
    let t = transpose(block);
    println!("Looking for vertical symmetry in");
    display(&t);

    let cols = block.len();
    let rows = block[0].len();

    for col in 0..cols / 2 {
        for row in 0..rows {
            if block[col][row] != block[cols - 1 - col][row] {
                return false;
            }
        }
    }

    return true;
}

fn display(block: &[Vec<Ground>]) {
    for row in block {
        for g in row {
            print!("{}", g);
        }

        print!("\n");
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<Vec<Ground>>> {
    let mut out = vec![];

    let mut tmp_block = vec![];

    for line in lines {
        if line == "" {
            out.push(tmp_block);
            tmp_block = vec![];
        } else {
            tmp_block.push(line.chars().map(|x| Ground::from(x)).collect())
        }
    }

    out.push(tmp_block);

    return out;
}

#[derive(PartialEq, Clone, Copy)]
enum Ground {
    Ash,
    Rocks,
}

impl Ground {
    pub fn flip(&self) -> Self {
        match self {
            Self::Ash => Self::Rocks,
            Self::Rocks => Self::Ash,
        }
    }
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        if value == '.' {
            return Self::Rocks;
        } else if value == '#' {
            return Self::Ash;
        } else {
            panic!("Unknown char {}", value);
        }
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => write!(f, "#"),
            Self::Rocks => write!(f, "."),
        }
    }
}

#[test]
fn test_part1() {
    let path = "data_demo/day13_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 405);
}

#[test]
fn test_part2() {
    let path = "data_demo/day13_demo.txt";
    let sum = part2(path);
    assert_eq!(sum, 400);
}
