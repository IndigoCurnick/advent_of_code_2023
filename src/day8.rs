use std::{collections::HashMap, env::temp_dir};

use crate::read_lines;

pub fn day8() {
    let path = "data/day8.txt";
    let count = part1(path);
    println!("Day 8 Part 1 {}", count);
    let count = part2(path);
    println!("Day 8 Part 2 {}", count);
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);

    let lr = &lines[0];

    let mut turns = vec![];
    for c in lr.chars() {
        if c == 'L' {
            turns.push(LR::Left);
        } else if c == 'R' {
            turns.push(LR::Right)
        } else {
            panic!("Char {} in LR", c);
        }
    }

    let mut directions: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        let source = lines[i][0..=2].to_string();
        let left = lines[i][7..=9].to_string();
        let right = lines[i][12..=14].to_string();

        directions.insert(source, (left, right));
    }

    let mut count = 0;
    let mut current = "AAA".to_string();
    let mut met_goal = false;
    let goal = "ZZZ".to_string();

    while !met_goal {
        for turn in turns.iter() {
            let direction = directions.get(&current).unwrap();

            match turn {
                LR::Left => current = direction.0.clone(),
                LR::Right => current = direction.1.clone(),
            }

            count += 1;

            if current == goal {
                met_goal = true;
                break;
            }
        }
    }

    return count;
}

fn part2(path: &str) -> i64 {
    let lines = read_lines(path);

    let lr = &lines[0];

    let mut turns = vec![];
    for c in lr.chars() {
        if c == 'L' {
            turns.push(LR::Left);
        } else if c == 'R' {
            turns.push(LR::Right)
        } else {
            panic!("Char {} in LR", c);
        }
    }

    let mut current = vec![];

    let mut directions: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in 2..lines.len() {
        let source = &lines[i][0..=2];
        let left = &lines[i][7..=9];
        let right = &lines[i][12..=14];

        if source.ends_with("A") {
            current.push(source);
        }

        directions.insert(source, (left, right));
    }

    let mut times = vec![];

    for source in current {
        let mut count = 0;
        let mut met_goal = false;
        let mut current = source;

        while !met_goal {
            for turn in turns.iter() {
                let direction = directions.get(&current).unwrap();

                match turn {
                    LR::Left => current = direction.0.clone(),
                    LR::Right => current = direction.1.clone(),
                }

                count += 1;

                if current.ends_with("Z") {
                    met_goal = true;
                    break;
                }
            }
        }

        times.push(count);
    }

    println!("{:?}", times);

    let lcm = rec_lmc(&times);

    return lcm;
}

fn rec_lmc(nums: &[i64]) -> i64 {
    let n = nums.len();
    if n == 2 {
        return lcm(nums[0], nums[1]);
    }

    return lcm(nums[0], rec_lmc(&nums[1..n]));
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn has_met_goal(current: &Vec<&str>) -> bool {
    for cur in current {
        if !cur.ends_with("Z") {
            return false;
        }
    }

    return true;
}

enum LR {
    Left,
    Right,
}

#[test]
fn test_part1() {
    let data = "data_demo/day8_demo1.txt";
    let count = part1(data);
    assert_eq!(count, 2);
    let data = "data_demo/day8_demo2.txt";
    let count = part1(data);
    assert_eq!(count, 6);
}

#[test]
fn test_part2() {
    let path = "data_demo/day8_demo3.txt";
    let count = part2(path);
    assert_eq!(count, 6);
}

#[test]
fn test_lcm() {
    let nums = vec![4, 6, 8];
    let lcm = rec_lmc(&nums);
    assert_eq!(lcm, 24);
}
