use crate::read_lines;

pub fn day18() {
    let path = "data/day18.txt";
    let count = part1(path);
    println!("Day 18 Part 1 {}", count);
    let count = part2(path);
    println!("Day 18 Part 2 {}", count);
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    let insturctions = parse_input(&lines);
    let mut i = 0;
    let mut j = 0;

    let mut min_i = 0;
    let mut min_j = 0;
    let mut max_i = 0;
    let mut max_j = 0;

    let mut edge = vec![(i, j)];

    let ins = insturctions.len();
    for instruction in insturctions {
        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => i -= 1,
                Direction::Right => j += 1,
                Direction::Down => i += 1,
                Direction::Left => j -= 1,
            }

            edge.push((i, j));

            if i < min_i {
                min_i = i;
            }

            if i > max_i {
                max_i = i;
            }

            if j < min_j {
                min_j = j;
            }

            if j > max_j {
                max_j = j;
            }
        }
    }

    println!("There are {} instructions", ins);

    display_edge(min_i, min_j, max_i, max_j, &edge);

    let mut count = 0;

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if in_or_on_polygon((i, j), &edge) {
                count += 1;
            }
        }
    }

    return count;
}

fn part2(path: &str) -> i64 {
    let lines = read_lines(path);

    let insturctions = parse_input(&lines);
    let mut i = 0;
    let mut j = 0;

    let mut edge = vec![(i, j)];

    let mut boundary = 0;

    for instruction in insturctions {
        let distance_str = &instruction.colour[0..5];
        let distance = i64::from_str_radix(distance_str, 16).unwrap();
        let dir_str = &instruction.colour[5..6];
        let dir = Direction::from_str_alt(dir_str);

        boundary += distance;

        match dir {
            Direction::Up => i -= distance,
            Direction::Right => j += distance,
            Direction::Down => i += distance,
            Direction::Left => j -= distance,
        }

        edge.push((i, j));
    }

    let pseudoarea: i64 = edge
        .iter()
        .zip(edge.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    let area = pseudoarea.abs() / 2 + boundary / 2 + 1;

    return area;
}

fn in_or_on_polygon(point: (i32, i32), polygon: &Vec<(i32, i32)>) -> bool {
    if on_polygon(point, polygon) {
        return true;
    }

    return in_polygon(point, polygon);
}

fn on_polygon(point: (i32, i32), polygon: &Vec<(i32, i32)>) -> bool {
    if polygon.contains(&point) {
        return true;
    }

    return false;
}

fn in_polygon(point: (i32, i32), polygon: &Vec<(i32, i32)>) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;

        let vi = polygon[i];
        let vj = polygon[j];

        // Check if the ray intersects with the polygon edge
        if ((vi.0 > point.0) != (vj.0 > point.0))
            && (point.1 < (vj.1 - vi.1) * (point.0 - vi.0) / (vj.0 - vi.0) + vi.1)
        {
            inside = !inside;
        }
    }

    return inside;
}

fn display(min_i: i32, min_j: i32, max_i: i32, max_j: i32, polygon: &Vec<(i32, i32)>) {
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if on_polygon((i, j), polygon) {
                print!("#");
            } else if in_polygon((i, j), polygon) {
                print!("^");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
}

fn display_edge(min_i: i32, min_j: i32, max_i: i32, max_j: i32, polygon: &Vec<(i32, i32)>) {
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if on_polygon((i, j), polygon) {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<DiggerInstructions> {
    let mut out = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let dir = split[0];
        let dist = split[1];
        let colour = split[2][2..8].to_string();

        out.push(DiggerInstructions::new(dir, dist, colour));
    }

    return out;
}

struct DiggerInstructions {
    pub direction: Direction,
    pub distance: i32,
    pub colour: String,
}

impl DiggerInstructions {
    pub fn new(dir: &str, dist: &str, colour: String) -> Self {
        return Self {
            direction: Direction::from(dir),
            distance: dist.parse().unwrap(),
            colour: colour,
        };
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_str_alt(value: &str) -> Self {
        if value == "0" {
            return Self::Right;
        } else if value == "1" {
            return Self::Down;
        } else if value == "2" {
            return Self::Left;
        } else if value == "3" {
            return Self::Up;
        } else {
            panic!("Unknown dir {}", value);
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        if value == "U" {
            return Self::Up;
        } else if value == "R" {
            return Self::Right;
        } else if value == "D" {
            return Self::Down;
        } else if value == "L" {
            return Self::Left;
        } else {
            panic!("Can not make direction from {}", value);
        }
    }
}

#[test]
fn test_part1() {
    let path = "data/day18_demo.txt";
    let count = part1(path);
    assert_eq!(62, count);
}

#[test]
fn test_part2() {
    let path = "data/day18_demo.txt";
    let count = part2(path);
    assert_eq!(952408144115, count);
}
