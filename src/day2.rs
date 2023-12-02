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

                if unit.contains(BLUE) {
                    if count > NUM_BLUE {
                        possible = false;
                        break;
                    }
                } else if unit.contains(RED) {
                    if count > NUM_RED {
                        possible = false;
                        break;
                    }
                } else if unit.contains(GREEN) {
                    if count > NUM_GREEN {
                        possible = false;
                        break;
                    }
                } else {
                    panic!("String matching colours has gone wrong")
                }
            }
        }

        if possible {
            sum += game_number;
        }
    }

    return sum;
}

#[test]
fn test_part1() {
    let file_path = "data/day2_part1_demo.txt";
    let sum = part1(file_path);
    println!("Day 2 Test Sum {}", sum);
    assert_eq!(sum, 8);
}
