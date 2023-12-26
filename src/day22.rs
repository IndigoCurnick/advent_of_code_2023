use std::collections::{HashSet, VecDeque};

use crate::read_lines;

pub fn day22() {
    let path = "data/day22.txt";
    let count = part1(path);
    println!("Day 22 Part 1: {}", count);
    let count = part2(path);
    println!("Day 22 Part 2: {}", count);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);
    let mut bricks: Vec<Brick> = lines.iter().map(|line| line.into()).collect();

    bricks.sort_by_key(|b| b.first.z.min(b.second.z));
    assert!(bricks.iter().all(|b| b.first.x <= b.second.x));
    assert!(bricks.iter().all(|b| b.first.y <= b.second.y));
    assert!(bricks.iter().all(|b| b.first.z <= b.second.z));

    let mut dropped = vec![];

    for brick in bricks.iter_mut() {
        let lowest = brick.highest_z(&dropped);

        let diff = brick.second.z - brick.first.z;

        brick.first.z = lowest + 1;
        brick.second.z = brick.first.z + diff;
        dropped.push(brick.clone());
    }

    return dropped
        .iter()
        .filter(|b| b.safe_to_remove(&dropped))
        .count();
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);
    let mut bricks: Vec<Brick> = lines.iter().map(|line| line.into()).collect();

    bricks.sort_by_key(|b| b.first.z.min(b.second.z));
    assert!(bricks.iter().all(|b| b.first.x <= b.second.x));
    assert!(bricks.iter().all(|b| b.first.y <= b.second.y));
    assert!(bricks.iter().all(|b| b.first.z <= b.second.z));

    let mut dropped = vec![];

    for brick in bricks.iter_mut() {
        let lowest = brick.highest_z(&dropped);

        let diff = brick.second.z - brick.first.z;

        brick.first.z = lowest + 1;
        brick.second.z = brick.first.z + diff;
        dropped.push(brick.clone());
    }

    return dropped
        .iter()
        .fold(0, |acc, brick| acc + brick.falls(&dropped));
}

#[derive(PartialEq, Clone, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}

#[derive(PartialEq, Clone, Eq, Hash)]
struct Brick {
    pub first: Point,
    pub second: Point,
}

impl From<&String> for Brick {
    fn from(input: &String) -> Self {
        let mut split = input.split('~');
        let first = split.next().unwrap().into();
        let second = split.next().unwrap().into();

        Self { first, second }
    }
}

impl Brick {
    fn intersects_xy(&self, other: &Self) -> bool {
        return self.first.x <= other.second.x
            && self.second.x >= other.first.x
            && self.first.y <= other.second.y
            && self.second.y >= other.first.y;
    }

    fn highest_z(&self, others: &Vec<Self>) -> i32 {
        return others
            .iter()
            .filter(|b| *b != self)
            .filter(|&b| self.intersects_xy(b))
            .map(|b| b.second.z)
            .max()
            .unwrap_or(0);
    }

    fn above(&self, others: &Vec<Self>) -> Vec<Self> {
        return others
            .iter()
            .filter(|b| *b != self && self.intersects_xy(b) && b.first.z == self.second.z + 1)
            .cloned()
            .collect();
    }

    fn below(&self, others: &Vec<Self>) -> Vec<Self> {
        return others
            .iter()
            .filter(|b| *b != self && self.intersects_xy(b) && b.second.z == self.first.z - 1)
            .cloned()
            .collect();
    }

    fn safe_to_remove(&self, others: &Vec<Self>) -> bool {
        let above = self.above(others);

        if above.is_empty() {
            return true;
        }

        let to_check = others.iter().filter(|b| *b != self).cloned().collect();

        for above in above.iter() {
            let below = above.below(&to_check);
            if below.is_empty() {
                return false;
            }
        }

        return true;
    }

    fn falls(&self, others: &Vec<Self>) -> usize {
        let mut fallen = HashSet::new();
        let mut stack = VecDeque::new();

        stack.push_back(self.clone());

        let mut remaining = others.clone();

        while let Some(brick) = stack.pop_front() {
            if fallen.contains(&brick) {
                continue;
            }

            fallen.insert(brick.clone());
            remaining.retain(|p| *p != brick);

            let new_fallen: Vec<Brick> = brick
                .above(&remaining)
                .iter()
                .filter(|b| b.below(&remaining).is_empty())
                .cloned()
                .collect();

            stack.extend(new_fallen);
        }

        return fallen.len() - 1;
    }
}

#[test]
fn test_part1() {
    let path = "data_demo/day22_demo.txt";
    let count = part1(path);
    assert_eq!(5, count);
}

#[test]
fn test_part2() {
    let path = "data_demo/day22_demo.txt";
    let count = part2(path);
    assert_eq!(7, count);
}
