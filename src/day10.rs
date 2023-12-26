use crate::read_lines;

pub fn day10() {
    let path = "data/day10.txt";
    let count = part1(path);
    println!("Day 10 Part 1: {}", count);
    let count = part2(path);
    println!("Day 10 Part 2: {}", count);
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    let (grid, (start_i, start_j)) = parse_input(&lines);

    let mut running = true;

    let mut last_direction = find_starting_direction(&grid, start_i, start_j);
    let mut i = start_i;
    let mut j = start_j;

    let mut moves = 0;

    while running {
        moves += 1;

        let (new_i, new_j) = last_direction.get_fn()(i, j);

        let new_pipe = grid[new_i][new_j].clone();

        println!("{:?}", new_pipe);

        if new_pipe == Pipe::Start {
            running = false;
            break;
        }

        let new_direction = new_pipe.next_direction(last_direction.clone());

        last_direction = new_direction;

        i = new_i;
        j = new_j;
    }

    return moves / 2;
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);

    let (grid, (start_i, start_j)) = parse_input(&lines);

    let mut running = true;

    let mut last_direction = find_starting_direction(&grid, start_i, start_j);
    let mut i = start_i;
    let mut j = start_j;

    let mut points_on_line = vec![];
    while running {
        points_on_line.push((i as i32, j as i32));

        let (new_i, new_j) = last_direction.get_fn()(i, j);

        let new_pipe = grid[new_i][new_j].clone();

        println!("{:?}", new_pipe);

        if new_pipe == Pipe::Start {
            running = false;
            break;
        }

        let new_direction = new_pipe.next_direction(last_direction.clone());

        last_direction = new_direction;

        i = new_i;
        j = new_j;
    }

    let height = grid.len();

    let mut points_in_polygon = 0;

    println!("Points on line");
    println!("{:?}", points_on_line);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if points_on_line.contains(&(i as i32, j as i32)) {
                // Point on line
                continue;
            }

            if in_polygon((i as i32, j as i32), &points_on_line) {
                points_in_polygon += 1;
            }
        }
    }

    return points_in_polygon;
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

fn parse_input(lines: &Vec<String>) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut output = vec![];

    let mut start_i = 0;
    let mut start_j = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut tmp = vec![];

        for (j, c) in line.chars().enumerate() {
            let pipe = Pipe::new_from_char(c);

            if pipe == Pipe::Start {
                start_i = i;
                start_j = j;
            }

            tmp.push(pipe);
        }

        output.push(tmp);
    }

    return (output, (start_i, start_j));
}

#[derive(Debug, PartialEq, Clone)]
enum Pipe {
    Vertical,      // |
    Horizontal,    // -
    NorthEastPipe, // L
    NorthWestPipe, // J
    SouthWestPipe, // 7
    SouthEastPipe, // F
    Ground,        // .
    Start,         // S
}

impl Pipe {
    fn next_direction(self, previous_direction: Direction) -> Direction {
        if previous_direction == Direction::North {
            if self == Self::Vertical {
                return Direction::North;
            } else if self == Self::SouthEastPipe {
                return Direction::East;
            } else if self == Self::SouthWestPipe {
                return Direction::West;
            } else {
                panic!(
                    "Incompatible directions {:?} & {:?}",
                    self, previous_direction
                );
            }
        } else if previous_direction == Direction::East {
            if self == Self::Horizontal {
                return Direction::East;
            } else if self == Self::NorthWestPipe {
                return Direction::North;
            } else if self == Self::SouthWestPipe {
                return Direction::South;
            } else {
                panic!(
                    "Incompatible directions {:?} & {:?}",
                    self, previous_direction
                );
            }
        } else if previous_direction == Direction::South {
            if self == Self::Vertical {
                return Direction::South;
            } else if self == Self::NorthEastPipe {
                return Direction::East;
            } else if self == Self::NorthWestPipe {
                return Direction::West;
            } else {
                panic!(
                    "Incompatible directions {:?} & {:?}",
                    self, previous_direction
                );
            }
        } else if previous_direction == Direction::West {
            if self == Self::Horizontal {
                return Direction::West;
            } else if self == Self::NorthEastPipe {
                return Direction::North;
            } else if self == Self::SouthEastPipe {
                return Direction::South;
            } else {
                panic!(
                    "Incompatible directions {:?} & {:?}",
                    self, previous_direction
                );
            }
        }

        panic!("Bad directions");
    }
}
#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_fn(&self) -> &dyn Fn(usize, usize) -> (usize, usize) {
        match self {
            Direction::North => return &go_north,
            Direction::East => return &go_east,
            Direction::South => return &go_south,
            Direction::West => return &go_west,
        }
    }
}

fn find_starting_direction(grid: &Vec<Vec<Pipe>>, start_i: usize, start_j: usize) -> Direction {
    // North

    let height = grid.len();
    let width = grid[0].len();

    if start_i > 0 {
        let (ni, nj) = go_north(start_i, start_j);

        match grid[ni][nj] {
            Pipe::Vertical | Pipe::SouthEastPipe | Pipe::SouthWestPipe => return Direction::North,
            _ => {}
        };
    }

    // East
    if start_j < width {
        let (ei, ej) = go_east(start_i, start_j);

        match grid[ei][ej] {
            Pipe::Horizontal | Pipe::NorthWestPipe | Pipe::SouthWestPipe => return Direction::East,
            _ => {}
        };
    }

    // South
    if start_i < height {
        let (si, sj) = go_south(start_i, start_j);

        match grid[si][sj] {
            Pipe::Vertical | Pipe::NorthWestPipe | Pipe::NorthEastPipe => return Direction::South,
            _ => {}
        };
    }

    // West
    if start_j > 0 {
        let (wi, wj) = go_west(start_i, start_j);

        match grid[wi][wj] {
            Pipe::Horizontal | Pipe::NorthEastPipe | Pipe::SouthEastPipe => return Direction::West,
            _ => {}
        };
    }

    panic!("Could not find starting direction");
}

fn go_north(i: usize, j: usize) -> (usize, usize) {
    return (i - 1, j);
}

fn go_east(i: usize, j: usize) -> (usize, usize) {
    return (i, j + 1);
}

fn go_south(i: usize, j: usize) -> (usize, usize) {
    return (i + 1, j);
}

fn go_west(i: usize, j: usize) -> (usize, usize) {
    return (i, j - 1);
}

impl Pipe {
    pub fn new_from_char(c: char) -> Self {
        return match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEastPipe,
            'J' => Self::NorthWestPipe,
            '7' => Self::SouthWestPipe,
            'F' => Self::SouthEastPipe,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Unrecgonised char in input {}", c),
        };
    }
}

#[test]
fn test_part1() {
    let path = "data_demo/day10_demo.txt";
    let count = part1(path);
    assert_eq!(count, 8);
}

#[test]
fn test_part2() {
    let path = "data_demo/day10_demo2.txt";
    let count = part2(path);
    assert_eq!(count, 10);
}
