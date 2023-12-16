use std::fmt::Display;

use crate::read_lines;

pub fn day16() {
    let path = "data/day16.txt";
    let count = part1(path);
    println!("Day 16 Part 1 {}", count);
    let count = part2(path);
    println!("Day 16 Part 2 {}", count);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);

    let grid = parse_input(&lines);

    let initial_direction = (0, 0, Direction::East);

    return count_energised(initial_direction, &grid);
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);

    let grid = parse_input(&lines);

    let mut energised = vec![];

    let mut starting_positions = vec![];

    let height = grid.len();
    let width = grid[0].len();

    // South Facing Start Directions
    for i in 0..width {
        starting_positions.push((0 as i32, i as i32, Direction::South));
    }

    // East Facing Start Directions
    for i in 0..height {
        starting_positions.push((i as i32, 0 as i32, Direction::East));
    }

    // North Facing Start Directions
    for i in 0..width {
        starting_positions.push(((height - 1) as i32, i as i32, Direction::North));
    }

    // West Facing Start Directions
    for i in 0..height {
        starting_positions.push((i as i32, (width - 1) as i32, Direction::West));
    }

    for starting_position in starting_positions {
        energised.push(count_energised(starting_position, &grid));
    }

    return *energised.iter().max().unwrap();
}

fn count_energised(initial_direction: (i32, i32, Direction), grid: &Vec<Vec<Tile>>) -> usize {
    let mut energised_tiles = vec![];

    let mut beams = vec![initial_direction];

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    while beams.len() > 0 {
        let beam = &beams[0];
        let tile = &grid[beam.0 as usize][beam.1 as usize];

        energised_tiles.push(beam.clone());

        let (new_i, new_j, new_direction, secondary_i, secondary_j, secondary_direction) =
            beam_logic(tile, beam);

        let mut change = true;

        if new_i < 0 || new_i >= height {
            // Out of bounds
            beams.remove(0);
            change = false;
        }

        if (new_j < 0 || new_j >= width) && change {
            // Out of bounds
            beams.remove(0);
            change = false;
        }

        if (energised_tiles.contains(&(new_i, new_j, new_direction))) && change {
            // Already been here - in a loop
            beams.remove(0);
            change = false;
        }

        if change {
            beams[0] = (new_i, new_j, new_direction);
        }

        if secondary_i.is_some() {
            let i = secondary_i.unwrap();
            let j = secondary_j.unwrap();
            let d = secondary_direction.unwrap();

            if i < 0 || i >= height {
                // Out of bounds
                continue;
            }

            if j < 0 || j >= width {
                continue;
            }

            if energised_tiles.contains(&(i, j, d)) {
                continue;
            }

            beams.push((i, j, d));
        }
    }

    // display_energised(&energised_tiles, height as usize, width as usize);

    let mut locs = vec![];

    for loc in energised_tiles {
        if !locs.contains(&(loc.0, loc.1)) {
            locs.push((loc.0, loc.1));
        }
    }

    return locs.iter().count();
}

fn beam_logic(
    tile: &Tile,
    beam: &(i32, i32, Direction),
) -> (
    i32,
    i32,
    Direction,
    Option<i32>,
    Option<i32>,
    Option<Direction>,
) {
    let (new_i, new_j, new_direction, secondary_i, secondary_j, secondary_direction) = match beam.2
    {
        Direction::North => match tile {
            Tile::EmptySpace => (beam.0 - 1, beam.1, beam.2, None, None, None),
            Tile::ForwardSlash => (beam.0, beam.1 + 1, Direction::East, None, None, None),
            Tile::BackSlash => (beam.0, beam.1 - 1, Direction::West, None, None, None),
            Tile::VerticalSplitter => (beam.0 - 1, beam.1, beam.2, None, None, None),
            Tile::HorizontalSplitter => (
                beam.0,
                beam.1 + 1,
                Direction::East,
                Some(beam.0),
                Some(beam.1 - 1),
                Some(Direction::West),
            ),
        },
        Direction::East => match tile {
            Tile::EmptySpace => (beam.0, beam.1 + 1, beam.2, None, None, None),
            Tile::ForwardSlash => (beam.0 - 1, beam.1, Direction::North, None, None, None),
            Tile::BackSlash => (beam.0 + 1, beam.1, Direction::South, None, None, None),
            Tile::VerticalSplitter => (
                beam.0 - 1,
                beam.1,
                Direction::North,
                Some(beam.0 + 1),
                Some(beam.1),
                Some(Direction::South),
            ),
            Tile::HorizontalSplitter => (beam.0, beam.1 + 1, beam.2, None, None, None),
        },
        Direction::South => match tile {
            Tile::EmptySpace => (beam.0 + 1, beam.1, beam.2, None, None, None),
            Tile::ForwardSlash => (beam.0, beam.1 - 1, Direction::West, None, None, None),
            Tile::BackSlash => (beam.0, beam.1 + 1, Direction::East, None, None, None),
            Tile::VerticalSplitter => (beam.0 + 1, beam.1, beam.2, None, None, None),
            Tile::HorizontalSplitter => (
                beam.0,
                beam.1 - 1,
                Direction::West,
                Some(beam.0),
                Some(beam.1 + 1),
                Some(Direction::East),
            ),
        },
        Direction::West => match tile {
            Tile::EmptySpace => (beam.0, beam.1 - 1, beam.2, None, None, None),
            Tile::ForwardSlash => (beam.0 + 1, beam.1, Direction::South, None, None, None),
            Tile::BackSlash => (beam.0 - 1, beam.1, Direction::North, None, None, None),
            Tile::VerticalSplitter => (
                beam.0 + 1,
                beam.1,
                Direction::South,
                Some(beam.0 - 1),
                Some(beam.1),
                Some(Direction::North),
            ),
            Tile::HorizontalSplitter => (beam.0, beam.1 - 1, beam.2, None, None, None),
        },
    };

    return (
        new_i,
        new_j,
        new_direction,
        secondary_i,
        secondary_j,
        secondary_direction,
    );
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Tile>> {
    let mut out = vec![];

    for line in lines {
        let mut tmp = vec![];

        for c in line.chars() {
            tmp.push(Tile::from(c));
        }

        out.push(tmp);
    }

    return out;
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Tile {
    EmptySpace,
    ForwardSlash,
    BackSlash,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        return if value == '.' {
            Self::EmptySpace
        } else if value == '/' {
            Self::ForwardSlash
        } else if value == '\\' {
            Self::BackSlash
        } else if value == '|' {
            Self::VerticalSplitter
        } else if value == '-' {
            Self::HorizontalSplitter
        } else {
            panic!("Could not parse input char {}", value);
        };
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::EmptySpace => write!(f, "."),
            Tile::ForwardSlash => write!(f, "/"),
            Tile::BackSlash => write!(f, "\\"),
            Tile::VerticalSplitter => write!(f, "|"),
            Tile::HorizontalSplitter => write!(f, "-"),
        }
    }
}

fn display_grid(grid: &Vec<Vec<Tile>>) {
    for line in grid {
        for tile in line {
            print!("{}", tile);
        }
        print!("\n");
    }
}

fn display_energised(energised: &Vec<(i32, i32, Direction)>, height: usize, width: usize) {
    let mut locs = vec![];

    for loc in energised {
        locs.push((loc.0 as usize, loc.1 as usize));
    }

    for i in 0..height {
        for j in 0..width {
            if locs.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
}

#[test]
fn test_part1() {
    let path = "data/day16_demo.txt";
    let energised = part1(path);
    assert_eq!(energised, 46);
}

#[test]
fn test_part2() {
    let path = "data/day16_demo.txt";
    let energised = part2(path);
    assert_eq!(energised, 51);
}
