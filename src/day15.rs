use std::{collections::HashMap, hash};

use crate::read_lines;

pub fn day15() {
    let path = "data/day15.txt";
    let sum = part1(path);
    println!("Day 15 Part 1 {}", sum);
    let sum = part2(path);
    println!("Day 15 Part 2 {}", sum);
}

fn part1(path: &str) -> usize {
    let lines = read_lines(path);
    let line = lines.concat();

    let split: Vec<&str> = line.split(",").collect();
    let mut sum = 0;
    for chunk in split {
        sum += get_hash(chunk)
    }

    return sum;
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);
    let line = lines.concat();

    let split: Vec<&str> = line.split(",").collect();

    let mut cache: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    for chunk in split {
        if chunk.contains('=') {
            // add
            let focal_split: Vec<&str> = chunk.split('=').collect();
            let focal_power = focal_split[1].parse::<usize>().unwrap();

            let hash = get_hash(focal_split[0]);
            let label = focal_split[0].to_string();

            if cache.contains_key(&hash) {
                let this_box = cache.get_mut(&hash).unwrap();

                let mut index = None;

                for (i, v) in this_box.iter().enumerate() {
                    if v.0 == label {
                        index = Some(i);
                        break;
                    }
                }

                if index.is_none() {
                    this_box.push((label, focal_power));
                } else {
                    let this_lense = this_box.get_mut(index.unwrap()).unwrap();
                    this_lense.1 = focal_power;
                }
            } else {
                cache.insert(hash, vec![(label, focal_power)]);
            }
        } else if chunk.contains('-') {
            // remove
            let mut label = chunk.to_string();
            label.pop();
            let hash = get_hash(&label);

            if cache.contains_key(&hash) {
                let this_box = cache.get_mut(&hash).unwrap();

                let mut index = None;

                for (i, v) in this_box.iter().enumerate() {
                    if v.0 == label {
                        index = Some(i);
                        break;
                    }
                }

                if index.is_some() {
                    this_box.remove(index.unwrap());
                }
            }
        } else {
            panic!("Invalid chunk {}", chunk);
        }

        for (box_num, lenses) in cache.iter() {
            println!("Box {} contains {:?}", box_num, lenses);
        }
    }

    let focal_power = get_focusing_power(&cache);

    return focal_power;
}

fn get_hash(chunk: &str) -> usize {
    let mut hash = 0;

    for c in chunk.chars() {
        let ascii = c as u8;

        hash += ascii as usize;

        hash *= 17;

        hash %= 256;
    }
    println!("hashing {} with value {}", chunk, hash);
    return hash;
}

fn get_focusing_power(cache: &HashMap<usize, Vec<(String, usize)>>) -> usize {
    let mut sum = 0;
    for (box_num, lenses) in cache.iter() {
        println!("Box {} contains {:?}", box_num, lenses);
        for (i, lens) in lenses.iter().enumerate() {
            let focal_power = (box_num + 1) * (i + 1) * lens.1;
            sum += focal_power;
        }
    }

    return sum;
}

#[test]
fn test_part1() {
    let path = "data/day15_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 1320)
}

#[test]
fn test_part2() {
    let path = "data/day15_demo.txt";
    let sum = part2(path);
    assert_eq!(sum, 145)
}
