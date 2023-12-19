use std::{collections::HashMap, fmt::Display, time::Instant};

use crate::read_lines;

pub fn day14() {
    let path = "data/day14.txt";
    let sum = part1(path);
    println!("Day 14 Part 1: {}", sum);
    let now = Instant::now();
    let sum = part2(path);
    let time = now.elapsed().as_millis();
    println!("Day 14 Part 2: {} (took {}ms)", sum, time);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);

    let mut parsed = parse_input(&lines);

    // MOVE ROCKS
    for i in 0..parsed.len() {
        for j in 0..parsed[0].len() {
            let mut running = true;
            let mut k = i;
            while running {
                let tmp = move_rock_north(&mut parsed, k, j);

                if tmp == k {
                    running = false;
                } else {
                    k = tmp;
                }
            }
        }
    }

    display(&parsed);

    // load

    let mut sum = 0;

    for (i, row) in parsed.iter().rev().enumerate() {
        for g in row {
            if *g == Ground::Movable {
                sum += i + 1;
            }
        }
    }

    return sum;
}

const TOTAL_CYCLES: usize = 1_000_000_000;

fn perform_cycle(parsed: &mut Vec<Vec<Ground>>) {
    // North
    for i in 0..parsed.len() {
        for j in 0..parsed[0].len() {
            let mut running = true;
            let mut k = i;
            while running {
                let tmp = move_rock_north(parsed, k, j);

                if tmp == k {
                    running = false;
                } else {
                    k = tmp;
                }
            }
        }
    }

    // West
    for j in 0..parsed[0].len() {
        for i in 0..parsed.len() {
            let mut running = true;
            let mut k = j;
            while running {
                let tmp = move_rock_west(parsed, i, k);

                if tmp == k {
                    running = false;
                } else {
                    k = tmp;
                }
            }
        }
    }

    // South
    for i in (0..parsed.len()).rev() {
        for j in 0..parsed[0].len() {
            let mut running = true;
            let mut k = i;
            while running {
                let tmp = move_rock_south(parsed, k, j);

                if tmp == k {
                    running = false;
                } else {
                    k = tmp;
                }
            }
        }
    }

    // East
    for j in (0..parsed[0].len()).rev() {
        for i in 0..parsed.len() {
            let mut running = true;
            let mut k = j;
            while running {
                let tmp = move_rock_east(parsed, i, k);

                if tmp == k {
                    running = false;
                } else {
                    k = tmp;
                }
            }
        }
    }
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);

    let mut parsed = parse_input(&lines);
    let mut cache = HashMap::new();
    // MOVE ROCKS
    for cycle in 1..=TOTAL_CYCLES {
        perform_cycle(&mut parsed);

        let hash = create_vec_id(&parsed);

        if cache.contains_key(&hash) {
            // We are in a cycle!!!!!
            let current_cycle = cycle;
            let previous_cycle_this_config = *cache.get(&hash).unwrap();

            let period = current_cycle - previous_cycle_this_config;

            let length_through_period = (TOTAL_CYCLES - previous_cycle_this_config) % period;

            // Need to do length_through_period cycles then get north weight
            for _ in 0..length_through_period {
                perform_cycle(&mut parsed);
            }

            break;
        } else {
            cache.insert(hash, cycle);
        }
    }

    // display(&parsed);

    // load

    let mut sum = 0;

    for (i, row) in parsed.iter().rev().enumerate() {
        for g in row {
            if *g == Ground::Movable {
                sum += i + 1;
            }
        }
    }

    return sum;
}

fn create_vec_id(vec_chars: &[Vec<Ground>]) -> [u64; 157] {
    // Encode the locations of the rounded rocks, assuming grid is 100x100 max.
    //
    // NOTE: `64 * 157 > 100 * 100`
    let mut bits = [0; 157];
    for (idx, obj) in vec_chars.iter().flatten().enumerate() {
        if *obj == Ground::Movable {
            bits[idx / 64] |= 1 << (idx % 64);
        }
    }
    return bits;
}

fn move_rock_north(parsed: &mut Vec<Vec<Ground>>, i: usize, j: usize) -> usize {
    if parsed[i][j] != Ground::Movable {
        return i;
    }

    if i == 0 {
        return i;
    }

    let possible_new_position = i - 1;

    if parsed[possible_new_position][j] == Ground::Empty {
        parsed[possible_new_position][j] = Ground::Movable;
        parsed[i][j] = Ground::Empty;
        return possible_new_position;
    }

    return i;
}

fn move_rock_east(parsed: &mut Vec<Vec<Ground>>, i: usize, j: usize) -> usize {
    if parsed[i][j] != Ground::Movable {
        return j;
    }
    let width = parsed[0].len();
    if j == width - 1 {
        return j;
    }

    let possible_new_position = j + 1;

    if parsed[i][possible_new_position] == Ground::Empty {
        parsed[i][possible_new_position] = Ground::Movable;
        parsed[i][j] = Ground::Empty;
        return possible_new_position;
    }

    return j;
}

fn move_rock_south(parsed: &mut Vec<Vec<Ground>>, i: usize, j: usize) -> usize {
    if parsed[i][j] != Ground::Movable {
        return i;
    }
    let height = parsed.len();
    if i == height - 1 {
        return i;
    }

    let possible_new_position = i + 1;

    if parsed[possible_new_position][j] == Ground::Empty {
        parsed[possible_new_position][j] = Ground::Movable;
        parsed[i][j] = Ground::Empty;
        return possible_new_position;
    }

    return i;
}

fn move_rock_west(parsed: &mut Vec<Vec<Ground>>, i: usize, j: usize) -> usize {
    if parsed[i][j] != Ground::Movable {
        return j;
    }
    let width = parsed[0].len();
    if j == 0 {
        return j;
    }

    let possible_new_position = j - 1;

    if parsed[i][possible_new_position] == Ground::Empty {
        parsed[i][possible_new_position] = Ground::Movable;
        parsed[i][j] = Ground::Empty;
        return possible_new_position;
    }

    return j;
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Ground>> {
    let mut out: Vec<Vec<Ground>> = vec![];

    for line in lines {
        let mut tmp = vec![];
        for c in line.chars() {
            tmp.push(Ground::from(c));
        }

        out.push(tmp);
    }

    return out;
}

fn display(data: &Vec<Vec<Ground>>) {
    for row in data {
        for g in row {
            print!("{}", g);
        }
        print!("\n");
    }
}

#[derive(Debug, PartialEq)]
enum Ground {
    Movable,
    Immovable,
    Empty,
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        if value == 'O' {
            return Ground::Movable;
        } else if value == '#' {
            return Ground::Immovable;
        } else if value == '.' {
            return Ground::Empty;
        } else {
            panic!("Unknown symbol {} in input", value)
        }
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Immovable => write!(f, "#"),
            Self::Movable => write!(f, "O"),
        }
    }
}

#[test]
fn test_part1() {
    let path = "data/day14_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 136);
}

#[test]
fn test_part2() {
    let path = "data/day14_demo.txt";
    let sum = part2(path);
    assert_eq!(sum, 64);
}
