use std::collections::HashMap;

use crate::read_lines;

pub fn day1() {
    let file_path = "data/day1_part1.txt";
    let part1 = part_one(file_path);
    println!("Final Calibration Sum Part 1: {}", part1);

    let part2 = part_two(file_path);
    println!("Final Calibration Total Part 2: {}", part2);
}

fn part_one(file_path: &str) -> i32 {
    let mut sum = 0;

    let split = read_lines(file_path);

    for line in split {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in line.chars() {
            let digit = c.to_digit(10);

            if digit.is_some() {
                if first.is_none() {
                    first = Some(c);
                    last = Some(c);
                } else if last.is_some() {
                    last = Some(c);
                }
            }
        }

        if first.is_none() || last.is_none() {
            panic!("Could not find two digits");
        }

        let this_calibration = format!("{}{}", first.unwrap(), last.unwrap());

        let this_calibration = this_calibration.parse::<i32>().unwrap();

        sum += this_calibration;
    }

    return sum;
    // Correct Answer: 54081
}

fn part_two(path: &str) -> i32 {
    let lines = read_lines(path);

    let mut map: HashMap<&str, char> = HashMap::new();
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');
    map.insert("1", '1');
    map.insert("2", '2');
    map.insert("3", '3');
    map.insert("4", '4');
    map.insert("5", '5');
    map.insert("6", '6');
    map.insert("7", '7');
    map.insert("8", '8');
    map.insert("9", '9');

    let mut sum: i32 = 0;
    for line in lines {
        let mut first = usize::MAX;
        let mut first_len = 1;
        let mut last = usize::MIN;
        let mut last_len = 1;

        for key in map.keys() {
            match line.find(key) {
                Some(x) => {
                    if x < first {
                        first = x;
                        first_len = key.len()
                    }
                }
                None => {}
            }
            match line.rfind(key) {
                Some(x) => {
                    if x > last {
                        last = x;
                        last_len = key.len()
                    }
                }
                None => {}
            }
        }

        let first_key =
            String::from_utf8(line.as_bytes()[first..(first + first_len)].to_vec()).unwrap();
        let last_key =
            String::from_utf8(line.as_bytes()[last..(last + last_len)].to_vec()).unwrap();

        let calibration = format!(
            "{}{}",
            map.get(first_key.as_str()).unwrap(),
            map.get(last_key.as_str()).unwrap()
        );

        sum += calibration.parse::<i32>().unwrap();
    }

    return sum;
    // correct answer: 54649
}

#[test]
fn test_day1_part1() {
    let file_path = "data_demo/day1_part1_demo.txt";
    let sum = part_one(file_path);
    assert_eq!(sum, 142);
}

#[test]
fn test_day1_part2() {
    let file_path = "data_demo/day1_part2_demo.txt";
    let sum = part_two(file_path);
    assert_eq!(sum, 281);
}
