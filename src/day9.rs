use crate::read_lines;

pub fn day9() {
    let path = "data/day9.txt";
    let sum = part1(path);
    println!("Day 9 Part 1 {}", sum);
    let sum = part2(path);
    println!("Day 9 Part 2 {}", sum);
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);

    let number_lines = parse_num_lines(&lines);

    let mut sums = 0;
    for sequence in number_lines {
        let mut going_down = true;

        let mut diffs = vec![sequence.clone()];

        while going_down {
            let diff = find_differences(diffs.last().unwrap());

            if is_all_zero(&diff) {
                going_down = false;
            }

            diffs.push(diff);
        }

        let mut increase = 0;

        for diff in diffs.iter().rev() {
            increase += diff.last().unwrap();
        }

        sums += increase;
    }

    return sums;
}

fn part2(path: &str) -> i64 {
    let lines = read_lines(path);

    let number_lines = parse_num_lines(&lines);
    let number_lines = reverse_all_num_lines(&number_lines);

    let mut sums = 0;
    for sequence in number_lines {
        let mut going_down = true;

        let mut diffs = vec![sequence.clone()];

        while going_down {
            let diff = find_differences(diffs.last().unwrap());

            if is_all_zero(&diff) {
                going_down = false;
            }

            diffs.push(diff);
        }

        let mut increase = 0;

        for diff in diffs.iter().rev() {
            increase += diff.last().unwrap();
        }

        sums += increase;
    }

    return sums;
}

fn reverse_all_num_lines(nums: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut out = vec![];

    for line in nums {
        let tmp: Vec<i64> = line.iter().rev().cloned().collect();

        out.push(tmp);
    }

    return out;
}

fn find_differences(nums: &Vec<i64>) -> Vec<i64> {
    let mut out = vec![];

    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        out.push(diff);
    }

    return out;
}

fn is_all_zero(nums: &Vec<i64>) -> bool {
    for i in nums {
        if *i != 0 {
            return false;
        }
    }

    return true;
}

fn parse_num_lines(lines: &Vec<String>) -> Vec<Vec<i64>> {
    let mut out = Vec::with_capacity(lines.len());

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        let mut tmp = Vec::with_capacity(split.len());

        for s in split {
            tmp.push(s.parse::<i64>().unwrap());
        }

        out.push(tmp);
    }

    return out;
}

#[test]
fn test_part1() {
    let path = "data_demo/day9_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 114)
}

#[test]
fn test_diff() {
    let seq: Vec<i64> = vec![1, 3, 6, 10, 15, 21, 28];
    let diff = find_differences(&seq);

    let correct: Vec<i64> = vec![2, 3, 4, 5, 6, 7];

    for (i, j) in diff.iter().zip(correct.iter()) {
        assert_eq!(i, j);
    }
}

#[test]
fn test_part2() {
    let path = "data_demo/day9_demo.txt";
    let sum = part2(path);
    assert_eq!(sum, 2);
}
