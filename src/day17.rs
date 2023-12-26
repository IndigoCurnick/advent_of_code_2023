use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

use crate::read_lines;

pub fn day17() {
    let path = "data/day17.txt";
    let heat = part1(path);
    println!("Day 17 Part 1 {}", heat);
    let heat = part2(path);
    println!("Day 17 Part 2 {}", heat);
}

fn part1(path: &str) -> u64 {
    let lines = read_lines(path);
    let grid = parse_input(&lines);

    let heat = dijkstra_shortest_path(&grid, 3, 0);

    return heat;
}

fn part2(path: &str) -> u64 {
    let lines = read_lines(path);
    let grid = parse_input(&lines);

    let heat = dijkstra_shortest_path(&grid, 10, 4);

    return heat;
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl Direction {
    fn repr(&self) -> String {
        match self {
            Direction::South => String::from("\x1b[31mv\x1b[0m"),
            Direction::North => String::from("\x1b[31m^\x1b[0m"),
            Direction::East => String::from("\x1b[31m>\x1b[0m"),
            Direction::West => String::from("\x1b[31m<\x1b[0m"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Point { i, j }
    }
}

#[derive(Debug, Clone)]
struct Node {
    location: Point,
    weight: u32,
    direction: Direction,
    count: u8,
    previous: Option<Box<Node>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.direction.hash(state);
        self.count.hash(state);
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
            && self.direction == other.direction
            && self.count == other.count
    }
}

impl Eq for Node {}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut out = vec![];

    for line in lines {
        let mut tmp = vec![];

        for c in line.chars() {
            tmp.push(c.to_digit(10).unwrap());
        }

        out.push(tmp);
    }

    return out;
}

fn dijkstra_shortest_path(grid: &Vec<Vec<u32>>, max_step_limit: u8, min_step_limit: u8) -> u64 {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let height = grid.len();
    let width = grid[0].len();
    let end_location = Point::new(height - 1, width - 1);
    let mut visited: HashSet<Node> = HashSet::new();
    let start_node = Node {
        location: Point::new(0, 0),
        weight: 0,
        direction: Direction::East,
        count: 0,
        previous: None,
    };
    heap.push(start_node);
    let mut end_node = None;
    while let Some(node) = heap.pop() {
        if node.location == end_location {
            if node.count <= min_step_limit {
                continue;
            }
            end_node = Some(node);
            break;
        }
        if visited.contains(&node) {
            continue;
        }
        // east
        if node.location.j < width - 1 && node.direction != Direction::West {
            if node.direction == Direction::East {
                if node.count < max_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.i, node.location.j + 1),
                        weight: node.weight + grid[node.location.i][node.location.j + 1],
                        direction: Direction::East,
                        count: node.count + 1,
                        previous: Some(Box::new(node.clone())),
                    };
                    heap.push(next_node);
                }
            } else if node.count >= min_step_limit {
                let next_node = Node {
                    location: Point::new(node.location.i, node.location.j + 1),
                    weight: node.weight + grid[node.location.i][node.location.j + 1],
                    direction: Direction::East,
                    count: 1,
                    previous: Some(Box::new(node.clone())),
                };
                heap.push(next_node);
            }
        }
        // west
        if node.location.j >= 1 && node.direction != Direction::East {
            if node.direction == Direction::West {
                if node.count < max_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.i, node.location.j - 1),
                        weight: node.weight + grid[node.location.i][node.location.j - 1],
                        direction: Direction::West,
                        count: node.count + 1,
                        previous: Some(Box::new(node.clone())),
                    };
                    heap.push(next_node);
                }
            } else if node.count >= min_step_limit {
                let next_node = Node {
                    location: Point::new(node.location.i, node.location.j - 1),
                    weight: node.weight + grid[node.location.i][node.location.j - 1],
                    direction: Direction::West,
                    count: 1,
                    previous: Some(Box::new(node.clone())),
                };
                heap.push(next_node);
            }
        }
        // north
        if node.location.i >= 1 && node.direction != Direction::South {
            if node.direction == Direction::North {
                if node.count < max_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.i - 1, node.location.j),
                        weight: node.weight + grid[node.location.i - 1][node.location.j],
                        direction: Direction::North,
                        count: node.count + 1,
                        previous: Some(Box::new(node.clone())),
                    };
                    heap.push(next_node);
                }
            } else if node.count >= min_step_limit {
                let next_node = Node {
                    location: Point::new(node.location.i - 1, node.location.j),
                    weight: node.weight + grid[node.location.i - 1][node.location.j],
                    direction: Direction::North,
                    count: 1,
                    previous: Some(Box::new(node.clone())),
                };
                heap.push(next_node);
            }
        }
        // south
        if node.location.i < height - 1 && node.direction != Direction::North {
            if node.direction == Direction::South {
                if node.count < max_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.i + 1, node.location.j),
                        weight: node.weight + grid[node.location.i + 1][node.location.j],
                        direction: Direction::South,
                        count: node.count + 1,
                        previous: Some(Box::new(node.clone())),
                    };
                    heap.push(next_node);
                }
            } else if node.count >= min_step_limit {
                let next_node = Node {
                    location: Point::new(node.location.i + 1, node.location.j),
                    weight: node.weight + grid[node.location.i + 1][node.location.j],
                    direction: Direction::South,
                    count: 1,
                    previous: Some(Box::new(node.clone())),
                };
                heap.push(next_node);
            }
        }
        visited.insert(node);
    }
    drop(visited);

    if let Some(node) = end_node {
        let mut data = grid
            .iter()
            .map(|row| row.iter().map(|val| val.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut curr: &Node = &node;
        data[curr.location.i][curr.location.j] = curr.direction.repr();
        while let Some(ref prev) = curr.previous {
            if !(prev.location.i == 0 && prev.location.j == 0) {
                data[prev.location.i][prev.location.j] = prev.direction.repr();
            }
            curr = prev;
        }

        for x in data {
            for y in x {
                print!("{y}");
            }
            println!();
        }

        node.weight as u64
    } else {
        panic!("Cannot reach end in the provided graph");
    }
}

#[test]
fn test_part1() {
    let path = "data_demo/day17_demo.txt";
    let heat = part1(path);
    assert_eq!(heat, 102);
}

#[test]
fn test_part2() {
    let path = "data_demo/day17_demo.txt";
    let heat = part2(path);
    assert_eq!(heat, 94);
}
