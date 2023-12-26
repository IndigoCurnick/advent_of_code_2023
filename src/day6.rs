use crate::read_lines;

pub fn day6() {
    let path = "data/day6.txt";
    let product = part1(path);
    println!("Day 6 Part 1 {}", product);
    let total = part2(path);
    println!("Day 6 Part 2 {}", total);
}

fn part2(path: &str) -> i64 {
    let lines = read_lines(path);

    assert_eq!(lines.len(), 2);

    let time_split: Vec<&str> = lines[0].split(":").collect();
    let distance_split: Vec<&str> = lines[1].split(":").collect();

    let time_nums = extract_nums(time_split[1]);
    let distance_nums = extract_nums(distance_split[1]);

    let big_time = convert_nums_to_big_num(time_nums);
    let big_distance = convert_nums_to_big_num(distance_nums);

    let mut total_victories = 0;

    for time_held in 0..=big_time {
        let time_remaining = big_time - time_held;
        let speed = time_held;

        let my_distance = calculate_distance(speed, time_remaining);

        if my_distance > big_distance {
            total_victories += 1;
        }
    }

    return total_victories;
}

fn convert_nums_to_big_num(nums: Vec<i64>) -> i64 {
    let mut string = String::new();

    for num in nums {
        string.push_str(&num.to_string());
    }

    return string.parse().unwrap();
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);

    assert_eq!(lines.len(), 2);

    let time_split: Vec<&str> = lines[0].split(":").collect();
    let distance_split: Vec<&str> = lines[1].split(":").collect();

    let time_nums = extract_nums(time_split[1]);
    let distance_nums = extract_nums(distance_split[1]);

    assert_eq!(time_nums.len(), distance_nums.len());

    let mut victories = vec![];

    for (time, distance) in time_nums.iter().zip(distance_nums.iter()) {
        let mut total_victories = 0;

        for time_held in 0..=*time {
            let time_remaining = *time - time_held;
            let speed = time_held;

            let my_distance = calculate_distance(speed, time_remaining);

            if my_distance > *distance {
                total_victories += 1;
            }
        }

        victories.push(total_victories);
    }
    return victories.iter().product();
}

fn extract_nums(line: &str) -> Vec<i64> {
    let str_vals = line.trim();

    let strs: Vec<&str> = str_vals.split_whitespace().collect();

    let mut output = vec![];
    for val in strs {
        let num = val.parse::<i64>().unwrap();
        output.push(num);
    }

    return output;
}

fn calculate_distance(speed: i64, time: i64) -> i64 {
    // Speed in milimetres per milisecond
    // Time in miliseconds remaining

    return speed * time;
}

#[test]
fn test_part1() {
    let path = "data_demo/day6_demo.txt";
    let product = part1(path);
    assert_eq!(product, 288);
}

#[test]
fn test_part2() {
    let path = "data_demo/day6_demo.txt";
    let total = part2(path);
    assert_eq!(total, 71503);
}
