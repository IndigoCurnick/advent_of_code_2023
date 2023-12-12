use std::collections::HashMap;

use crate::read_lines;

pub fn day12() {
    let path = "data/day12.txt";
    let sum = part1(path);
    println!("Day 12 Part 1 {}", sum);
    let sum = part2(path);
    println!("Day 12 Part 2 {}", sum);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);

    let mut sum = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        let c: Vec<char> = split[0].chars().collect();

        let nums: Vec<usize> = split[1]
            .split(",")
            .map(|z| z.parse::<usize>().unwrap())
            .collect();

        let mut cache = HashMap::new();

        let possible_ways = possible_ways(&mut cache, &c, None, &nums);

        sum += possible_ways;
    }

    return sum;
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);

    let mut sum = 0;

    let multiplier = 5;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        let c: Vec<char> = split[0].chars().collect();

        let mut new_c = vec![];
        for _ in 0..5 {
            new_c.push(c.clone());
            new_c.push(vec!['?']);
        }

        new_c.pop();
        let new_c = new_c.concat();

        let nums: Vec<usize> = split[1]
            .split(",")
            .map(|z| z.parse::<usize>().unwrap())
            .collect();

        let nums: Vec<usize> = nums
            .iter()
            .cloned()
            .cycle()
            .take(nums.len() * multiplier)
            .collect();

        let mut cache = HashMap::new();

        let possible_ways = possible_ways(&mut cache, &new_c, None, &nums);

        sum += possible_ways;
    }

    return sum;
}

fn possible_ways(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    s: &[char],
    within: Option<usize>,
    remaining: &[usize],
) -> usize {
    if s.is_empty() {
        return match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0,
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (s[0], within) {
        ('.', Some(x)) if x != remaining[0] => 0,
        ('.', Some(_)) => possible_ways(cache, &s[1..], None, &remaining[1..]),
        ('.', None) => possible_ways(cache, &s[1..], None, remaining),
        ('#', Some(_)) => possible_ways(cache, &s[1..], within.map(|x| x + 1), remaining),
        ('#', None) => possible_ways(cache, &s[1..], Some(1), remaining),
        ('?', Some(x)) => {
            let mut ans = possible_ways(cache, &s[1..], within.map(|x| x + 1), remaining);
            if x == remaining[0] {
                ans += possible_ways(cache, &s[1..], None, &remaining[1..])
            }
            ans
        }
        ('?', None) => {
            possible_ways(cache, &s[1..], Some(1), remaining)
                + possible_ways(cache, &s[1..], None, remaining)
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);

    return ways;
}

#[test]
fn test_part1() {
    let path = "data/day12_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 21);
}

#[test]
fn test_part2() {
    let path = "data/day12_demo.txt";
    let sum = part2(path);
    assert_eq!(sum, 525152);
}
