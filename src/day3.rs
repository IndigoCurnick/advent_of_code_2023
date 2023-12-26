use crate::read_lines;

pub fn day3() {
    let data = "data/day3.txt";
    let sum = part1(data);
    println!("Day 3 Part 1: {}", sum); // correct: 546563
    let sum = part2(data);
    println!("Day 3 Part 2: {}", sum); // correct: 546563
}

fn part1(path: &str) -> i32 {
    let mut engine_sum = 0;
    let lines = read_lines(path);

    let mut chars: Vec<Vec<char>> = vec![];

    let height = lines.len();

    for line in lines {
        chars.push(line.chars().collect());
    }

    let width = chars[0].len();

    let mut coords: Vec<Coord> = vec![];
    let mut number = vec![];

    for (i, line) in chars.iter().enumerate() {
        for (j, unit) in line.iter().enumerate() {
            if unit.is_digit(10) {
                number.push(unit);
                coords.push(Coord(i as i32, j as i32));
            } else {
                let num_len = number.len();

                if num_len == 0 {
                    // This is e.g. two . in a row, do nothing!
                    continue;
                } else {
                    // We've hit a . or something following a number. So
                    // we need to turn the Vec<char> into a String, parse it,
                    // then figure out which indicies to check to determine if
                    // this number is an engine part

                    let num_string: String = number.iter().map(|x| *x).collect();
                    let num_number = num_string
                        .parse::<i32>()
                        .expect("You did not correctly identify a number");

                    let coords_to_check =
                        find_coords_to_check(&coords, height as i32, width as i32);

                    for coord in coords_to_check {
                        let char_in_spot = chars[coord.0 as usize][coord.1 as usize];

                        if !char_in_spot.is_digit(10) && char_in_spot != '.' {
                            // Then it is some symbol which isn't a number or a ., so it is an engine part!
                            engine_sum += num_number;
                            break; // Don't want to add the number multiple times!
                        }
                    }

                    coords = vec![];
                    number = vec![];
                }
            }
        }
    }

    return engine_sum;
}

/// (i, j)
#[derive(PartialEq, Clone)]
struct Coord(i32, i32);

fn find_coords_to_check(num_coords: &Vec<Coord>, height: i32, width: i32) -> Vec<Coord> {
    let mut output = vec![];

    // The first element can handle the left hand coordinates

    for coord in num_coords {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_i = coord.0 + dy;
                let new_j = coord.1 + dx;

                let potential_adjacent = Coord(new_i, new_j);

                if new_j >= 0
                    && new_j < width
                    && new_i >= 0
                    && new_i < height
                    && !num_coords.contains(&potential_adjacent)
                    && !output.contains(&potential_adjacent)
                {
                    output.push(potential_adjacent);
                }
            }
        }
    }

    return output;
}

fn part2(path: &str) -> i32 {
    let mut engine_sum = 0;
    let lines = read_lines(path);

    let mut chars: Vec<Vec<char>> = vec![];

    let height = lines.len();

    for line in lines {
        chars.push(line.chars().collect());
    }

    let width = chars[0].len();

    for (i, line) in chars.iter().enumerate() {
        for (j, unit) in line.iter().enumerate() {
            if *unit != '*' {
                continue;
            }

            let coord = vec![Coord(i as i32, j as i32)];

            let coords_to_check = find_coords_to_check(&coord, height as i32, width as i32);

            let mut valid_coords = vec![];

            for c in coords_to_check {
                if chars[c.0 as usize][c.1 as usize].is_digit(10) {
                    valid_coords.push(c.clone());
                }
            }

            if !(valid_coords.len() >= 2) {
                continue;
            }

            let numbers = find_numbers(&chars, &valid_coords);

            if numbers.len() == 2 {
                let ratio = numbers.iter().fold(1, |acc, x| acc * x);

                engine_sum += ratio;
            }
        }
    }

    return engine_sum;
}

fn find_numbers(chars: &Vec<Vec<char>>, starting_points: &Vec<Coord>) -> Vec<i32> {
    let mut valid_nums = vec![];

    for coord in starting_points {
        let mut js = vec![];

        js.push(coord.1);

        // Look up to two to the right, and then up to two to the left

        let char_to_right = chars[coord.0 as usize][(coord.1 + 1) as usize];

        let char_to_right_is_digit = char_to_right.is_digit(10);

        if char_to_right_is_digit {
            js.push(coord.1 + 1);

            let char_to_right = chars[coord.0 as usize][(coord.1 + 2) as usize];

            let char_to_right_is_digit = char_to_right.is_digit(10);

            if char_to_right_is_digit {
                js.push(coord.1 + 2);
            }
        }

        let char_to_left = chars[coord.0 as usize][(coord.1 - 1) as usize];

        let char_to_left_is_digit = char_to_left.is_digit(10);

        if char_to_left_is_digit {
            js.push(coord.1 - 1);

            let char_to_left = chars[coord.0 as usize][(coord.1 - 2) as usize];

            let char_to_left_is_digit = char_to_left.is_digit(10);

            if char_to_left_is_digit {
                js.push(coord.1 - 2);
            }
        }

        js.sort();

        let mut char_nums = vec![];

        for j in js {
            char_nums.push(chars[coord.0 as usize][j as usize]);
        }

        let num_string: String = char_nums.iter().map(|x| *x).collect();
        let num_number = num_string
            .parse::<i32>()
            .expect("You did not correctly identify a number");

        if !valid_nums.contains(&num_number) {
            valid_nums.push(num_number);
        }
    }

    return valid_nums;
}

#[test]
fn test_part1() {
    let file = "data_demo/day3_demo.txt";
    let sum = part1(file);
    assert_eq!(sum, 4361);
}

#[test]
fn test_part2() {
    let file = "data_demo/day3_demo.txt";
    let sum = part2(file);
    assert_eq!(sum, 467835);
}
