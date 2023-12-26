use crate::read_lines;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn day5() {
    let path = "data/day5.txt";
    let lowest = part1(path);
    println!("Day 5 Part 1: {}", lowest);
    let lowest = part2(path);
    println!("Day 5 Part 2: {}", lowest);
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);

    let mut seeds = vec![];

    let mut maps = prepare_hashmap();

    let mut current_map = "".to_string();

    for line in lines {
        if line.contains("seeds") {
            seeds = parse_seeds(&line);
        } else if line == "" {
            continue;
        } else if line.contains("map") {
            current_map = line.clone();
        } else {
            let almanac_row = parse_mapping(&line);
            let current_map = maps.get_mut(&current_map).unwrap();
            current_map.push(almanac_row);
        }
    }

    let locations = evaluate_locations(&maps, &seeds);

    return *locations.iter().min().unwrap();
}

fn part2(path: &str) -> i64 {
    let lines = read_lines(path);

    let mut seeds = vec![];

    let mut maps = prepare_hashmap();

    let mut current_map = "".to_string();

    for line in lines {
        if line.contains("seeds") {
            seeds = parse_seeds3(&line);
        } else if line == "" {
            continue;
        } else if line.contains("map") {
            current_map = line.clone();
        } else {
            let almanac_row = parse_mapping(&line);
            let current_map = maps.get_mut(&current_map).unwrap();
            current_map.push(almanac_row);
        }
    }

    let locations = evaluate_locations(&maps, &seeds);

    return *locations.iter().min().unwrap();
}

fn evaluate_locations(map: &HashMap<String, Vec<AlmanacRow>>, seeds: &Vec<i64>) -> Vec<i64> {
    fn evaluate(map: &HashMap<String, Vec<AlmanacRow>>, seed: i64) -> i64 {
        let soil_almanacs = map.get("seed-to-soil map:").unwrap();
        let soil = evaluate_almanac(seed, soil_almanacs);

        let fertaliser_almanacs = map.get("soil-to-fertilizer map:").unwrap();
        let fertaliser = evaluate_almanac(soil, fertaliser_almanacs);

        let water_almanacs = map.get("fertilizer-to-water map:").unwrap();
        let water = evaluate_almanac(fertaliser, water_almanacs);

        let light_almanacs = map.get("water-to-light map:").unwrap();
        let light = evaluate_almanac(water, light_almanacs);

        let temp_almanacs = map.get("light-to-temperature map:").unwrap();
        let temp = evaluate_almanac(light, temp_almanacs);

        let humidity_almanacs = map.get("temperature-to-humidity map:").unwrap();
        let humidity = evaluate_almanac(temp, humidity_almanacs);

        let location_almanacs = map.get("humidity-to-location map:").unwrap();
        let location = evaluate_almanac(humidity, location_almanacs);

        return location;
    }

    let locations: Vec<i64> = seeds.par_iter().map(|&x| evaluate(map, x)).collect();

    return locations;
}

fn evaluate_locations2(map: &HashMap<String, Vec<AlmanacRow>>, seeds: &Vec<SeedRange>) -> Vec<i64> {
    fn evaluate_seed(map: &HashMap<String, Vec<AlmanacRow>>, seed: i64) -> i64 {
        let soil_almanacs = map.get("seed-to-soil map:").unwrap();
        let soil = evaluate_almanac(seed, soil_almanacs);

        let fertaliser_almanacs = map.get("soil-to-fertilizer map:").unwrap();
        let fertaliser = evaluate_almanac(soil, fertaliser_almanacs);

        let water_almanacs = map.get("fertilizer-to-water map:").unwrap();
        let water = evaluate_almanac(fertaliser, water_almanacs);

        let light_almanacs = map.get("water-to-light map:").unwrap();
        let light = evaluate_almanac(water, light_almanacs);

        let temp_almanacs = map.get("light-to-temperature map:").unwrap();
        let temp = evaluate_almanac(light, temp_almanacs);

        let humidity_almanacs = map.get("temperature-to-humidity map:").unwrap();
        let humidity = evaluate_almanac(temp, humidity_almanacs);

        let location_almanacs = map.get("humidity-to-location map:").unwrap();
        let location = evaluate_almanac(humidity, location_almanacs);

        return location;
    }

    let mut locations = vec![];

    for seed_range in seeds {
        let low = evaluate_seed(map, seed_range.low);
        let high = evaluate_seed(map, seed_range.high);

        let tmp = vec![low, high];

        let min = tmp.iter().min().unwrap();
        locations.push(*min);
    }

    return locations;
}

fn evaluate_almanac(key: i64, almanacs: &Vec<AlmanacRow>) -> i64 {
    for almanac in almanacs {
        if key >= almanac.source && key <= almanac.source + almanac.range {
            let diff = key - almanac.source;
            return almanac.destination + diff;
        }
    }

    return key;
}

fn prepare_hashmap() -> HashMap<String, Vec<AlmanacRow>> {
    let mut map = HashMap::new();

    map.insert("seed-to-soil map:".to_string(), Vec::new());
    map.insert("soil-to-fertilizer map:".to_string(), Vec::new());
    map.insert("fertilizer-to-water map:".to_string(), Vec::new());
    map.insert("water-to-light map:".to_string(), Vec::new());
    map.insert("light-to-temperature map:".to_string(), Vec::new());
    map.insert("temperature-to-humidity map:".to_string(), Vec::new());
    map.insert("humidity-to-location map:".to_string(), Vec::new());

    return map;
}

fn parse_mapping(line: &str) -> AlmanacRow {
    let split: Vec<&str> = line.split(" ").collect();

    assert_eq!(split.len(), 3, "Incorrect map line parse");

    return AlmanacRow {
        source: split[1].parse::<i64>().unwrap(),
        destination: split[0].parse::<i64>().unwrap(),
        range: split[2].parse::<i64>().unwrap(),
    };
}

fn insert_ranges(map: &mut HashMap<i64, i64>, almanac_row: &AlmanacRow) {
    for i in 0..almanac_row.range {
        map.insert(almanac_row.source + i, almanac_row.destination + i);
    }
}

struct AlmanacRow {
    pub source: i64,
    pub destination: i64,
    pub range: i64,
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let big_split: Vec<&str> = line.split(":").collect();

    assert_eq!(big_split.len(), 2, "Incorrect splitting");

    let nums_as_str = big_split[1].trim();

    let num_split: Vec<&str> = nums_as_str.split(" ").collect();

    let mut output = vec![];

    for num in num_split {
        // println!("Num {}", num);
        output.push(
            num.parse::<i64>()
                .expect(&format!("Could not parse {}", num)),
        );
    }

    return output;
}

struct SeedRange {
    pub low: i64,
    pub high: i64,
}

fn parse_seeds2(line: &str) -> Vec<SeedRange> {
    let big_split: Vec<&str> = line.split(":").collect();

    assert_eq!(big_split.len(), 2, "Incorrect splitting");

    let nums_as_str = big_split[1].trim();

    let num_split: Vec<&str> = nums_as_str.split(" ").collect();

    let mut output = vec![];

    let mut pair = vec![];

    for num in num_split {
        // println!("Num {}", num);

        let numeral = num
            .parse::<i64>()
            .expect(&format!("Could not parse {}", num));

        if pair.len() == 0 {
            pair.push(numeral);
        } else if pair.len() == 1 {
            pair.push(numeral);

            output.push(SeedRange {
                low: pair[0],
                high: pair[0] + pair[1] - 1,
            });

            pair = vec![];
        }
    }

    return output;
}

fn parse_seeds3(line: &str) -> Vec<i64> {
    let big_split: Vec<&str> = line.split(":").collect();

    assert_eq!(big_split.len(), 2, "Incorrect splitting");

    let nums_as_str = big_split[1].trim();

    let num_split: Vec<&str> = nums_as_str.split(" ").collect();

    let mut output = vec![];

    let mut pair = vec![];

    for num in num_split {
        // println!("Num {}", num);

        let numeral = num
            .parse::<i64>()
            .expect(&format!("Could not parse {}", num));

        if pair.len() == 0 {
            pair.push(numeral);
        } else if pair.len() == 1 {
            pair.push(numeral);

            for i in 0..pair[1] {
                output.push(pair[0] + i);
            }

            pair = vec![];
        }
    }

    return output;
}

#[test]
fn test_part1() {
    let path = "data_demo/day5_demo.txt";
    let lowest = part1(path);
    assert_eq!(lowest, 35);
}

#[test]
fn test_part2() {
    let path = "data_demo/day5_demo.txt";
    let lowest = part2(path);
    assert_eq!(lowest, 46);
}
