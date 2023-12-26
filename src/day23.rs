use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

use crate::read_lines;

// These are our cardinal directions we'll use this to short-circuit
// slopes.
static UP: Point = Point::new(0, -1);
static DOWN: Point = Point::new(0, 1);
static LEFT: Point = Point::new(-1, 0);
static RIGHT: Point = Point::new(1, 0);

// In other cases, we iterate over these to find neighbors.
static DIRECTIONS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

pub fn day23() {
    let path = "data/day23.txt";
    let max = part1(path);
    println!("Day 23 Part 1 {}", max);
    let max = part2(path);
    println!("Day 23 Part 2 {}", max);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);
    let map = Map::new(&lines);

    return map.longest_path_dfs();
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);
    let map = Map::new(&lines);

    return map.longest_bath_branches_dfs();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> Vec<Point> {
        DIRECTIONS.iter().map(|d| *self + *d).collect()
    }
}

struct Map {
    map: HashMap<Point, char>,
    start: Point,
    end: Point,
}

impl Map {
    fn new(input: &Vec<String>) -> Self {
        let map = input
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            c,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();

        let start = Point { x: 1, y: 0 };

        let end = Point {
            x: max_x - 1,
            y: max_y,
        };

        return Self { map, start, end };
    }

    fn neighbours(&self, p: &Point) -> Vec<Point> {
        match self.map.get(p).unwrap() {
            '>' => return vec![*p + RIGHT],
            '<' => return vec![*p + LEFT],
            '^' => return vec![*p + UP],
            'v' => return vec![*p + DOWN],
            _ => {}
        }

        let mut neighbours = Vec::new();
        for d in DIRECTIONS.iter() {
            let p = *p + *d;
            match self.map.get(&p) {
                None => continue,
                Some(c) => match (c, d.x, d.y) {
                    // We can't go back up a slope, so I wrote it sort
                    // of the opposite as one normally would. The 5
                    // conditions below are the invalid neighbors and
                    // then all others would be valid.
                    ('#', _, _) => continue,
                    ('>', -1, 0) => continue,
                    ('<', 1, 0) => continue,
                    ('^', 0, 1) => continue,
                    ('v', 0, -1) => continue,
                    _ => neighbours.push(p),
                },
            }
        }
        return neighbours;
    }

    fn dfs(&self, p: Point, seen: &mut HashSet<Point>, steps: usize, ends: &mut Vec<usize>) {
        if p == self.end {
            ends.push(steps);
            return;
        }

        if seen.contains(&p) {
            return;
        }

        seen.insert(p);

        for n in self.neighbours(&p) {
            if seen.contains(&n) {
                continue;
            }

            self.dfs(n, seen, steps + 1, ends);
        }

        seen.remove(&p);
    }

    fn longest_path_dfs(&self) -> usize {
        let mut ends = vec![];
        let mut seen = HashSet::new();

        self.dfs(self.start, &mut seen, 0, &mut ends);
        return *ends.iter().max().unwrap();
    }

    fn neighbours_pt2(&self, p: &Point) -> Vec<Point> {
        let mut neighbours = vec![];

        for d in p.neighbours() {
            match self.map.get(&d) {
                None => continue,
                Some(c) => match c {
                    '#' => continue,
                    _ => neighbours.push(d),
                },
            }
        }

        return neighbours;
        todo!();
    }

    fn find_branching_edges(&self) -> HashMap<Point, Vec<(Point, usize)>> {
        let map = self
            .map
            .iter()
            .filter(|(_, c)| **c != '#')
            .map(|(p, _)| {
                let n = self.neighbours_pt2(p).len();
                (*p, n)
            })
            .collect::<HashMap<_, _>>();

        let nodes = map
            .iter()
            .filter(|(_, n)| **n != 2)
            .map(|(p, _)| *p)
            .collect::<HashSet<_>>();

        let mut edges: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();

        for node in nodes.iter() {
            for mut neighbour in self.neighbours_pt2(node) {
                let mut prev = *node;

                let mut dist = 0;

                loop {
                    dist += 1;

                    let neighbours = self.neighbours_pt2(&neighbour);
                    let neighbours = neighbours
                        .iter()
                        .filter(|&n| *n != prev)
                        .collect::<Vec<_>>();

                    if neighbours.len() != 1 {
                        edges.entry(*node).or_default().push((neighbour, dist));
                        break;
                    }

                    prev = neighbour;
                    neighbour = *neighbours[0];
                }
            }
        }

        return edges;
    }

    fn longest_bath_branches_dfs(&self) -> usize {
        let edges = self.find_branching_edges();

        let mut seen = HashSet::new();
        let mut ends = Vec::new();
        self.dfs_pt2(&edges, self.start, &mut seen, 0, &mut ends);
        return *ends.iter().max().unwrap();
    }

    fn dfs_pt2(
        &self,
        edges: &HashMap<Point, Vec<(Point, usize)>>,
        p: Point,
        seen: &mut HashSet<Point>,
        steps: usize,
        ends: &mut Vec<usize>,
    ) {
        if p == self.end {
            ends.push(steps);
            return;
        }

        if seen.contains(&p) {
            return;
        }

        seen.insert(p);

        let neighbours = edges.get(&p).unwrap();

        for (neighbour, cost) in neighbours {
            if seen.contains(neighbour) {
                continue;
            }
            self.dfs_pt2(edges, *neighbour, seen, steps + cost, ends);
        }

        seen.remove(&p);
    }
}

#[test]
fn test_part1() {
    let path = "data_demo/day23_demo.txt";
    let max = part1(path);
    assert_eq!(max, 94);
}

#[test]
fn test_part2() {
    let path = "data_demo/day23_demo.txt";
    let max = part2(path);
    assert_eq!(max, 154);
}
