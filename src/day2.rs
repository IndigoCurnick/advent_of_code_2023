use crate::read_lines;

const NUM_RED: i32 = 12;
const NUM_GREEN: i32 = 13;
const NUM_BLUE: i32 = 14;

const BLUE: &str = "blue";
const RED: &str = "red";
const GREEN: &str = "green";

pub fn day2() {
    let data = "data/day2.txt";
    let sum = part1(data);
    println!("Day 2 Part One: {}", sum);
    let sum = part2(data);
    println!("Day 2 Part Two: {}", sum);
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    let mut sum = 0;

    for line in lines {
        // Find the game number - split on :

        let split: Vec<String> = line.split(":").map(|x| x.to_string()).collect();

        let game_split: Vec<&str> = split[0].split(" ").collect();

        let game_number = game_split[1].parse::<i32>().unwrap();

        let trimmed_scores = split[1].trim();

        let round_scores: Vec<String> = trimmed_scores.split(";").map(|x| x.to_string()).collect();

        let mut possible = true;

        for round_score in round_scores {
            if !possible {
                break;
            }

            let individual_scores: Vec<String> =
                round_score.split(",").map(|x| x.to_string()).collect();
            for individual_score in individual_scores {
                let unit = individual_score.trim();

                let unit_split: Vec<&str> = unit.split(" ").collect();

                let count = unit_split[0]
                    .parse::<i32>()
                    .expect("Number not found in score");

                possible = is_possible(unit, count);
                if !possible {
                    break;
                }
            }
        }

        if possible {
            sum += game_number;
        }
    }

    return sum;
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);

    let mut sum = 0;

    for line in lines {
        // Find the game number - split on :

        let split: Vec<String> = line.split(":").map(|x| x.to_string()).collect();

        // let game_split: Vec<&str> = split[0].split(" ").collect();

        // let game_number = game_split[1].parse::<i32>().unwrap();

        let trimmed_scores = split[1].trim();

        let round_scores: Vec<String> = trimmed_scores.split(";").map(|x| x.to_string()).collect();

        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        for round_score in round_scores {
            let individual_scores: Vec<String> =
                round_score.split(",").map(|x| x.to_string()).collect();
            for individual_score in individual_scores {
                let unit = individual_score.trim();

                let unit_split: Vec<&str> = unit.split(" ").collect();

                let count = unit_split[0]
                    .parse::<i32>()
                    .expect("Number not found in score");

                if unit.contains(BLUE) {
                    if count > blues {
                        blues = count;
                    }
                } else if unit.contains(RED) {
                    if count > reds {
                        reds = count;
                    }
                } else if unit.contains(GREEN) {
                    if count > greens {
                        greens = count;
                    }
                } else {
                    panic!("String matching colours has gone wrong")
                }
            }
        }

        sum += reds * blues * greens;
    }

    return sum;
}

fn is_possible(unit: &str, count: i32) -> bool {
    if unit.contains(BLUE) {
        if count > NUM_BLUE {
            return false;
        }
    } else if unit.contains(RED) {
        if count > NUM_RED {
            return false;
        }
    } else if unit.contains(GREEN) {
        if count > NUM_GREEN {
            return false;
        }
    } else {
        panic!("String matching colours has gone wrong")
    }

    return true;
}

// fn part2(path: &str) -> i32 {}

#[test]
fn test_part1() {
    let file_path = "data_demo/day2_demo.txt";
    let sum = part1(file_path);
    println!("Day 2 Part 1 Test Sum {}", sum);
    assert_eq!(sum, 8);
}

#[test]
fn test_part2() {
    let file_path = "data_demo/day2_demo.txt";
    let sum = part2(file_path);
    println!("Day 2 Part 2 Test Sum {}", sum);
    assert_eq!(sum, 2286);
}
