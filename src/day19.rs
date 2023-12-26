use std::collections::HashMap;

use crate::read_lines;

pub fn day19() {
    let path = "data/day19.txt";
    let sum = part1(path);
    println!("Day 19 Part 1 {}", sum);
    let sum = part2(path);
    println!("Day 19 Part 2 {}", sum);
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);

    let (workflows, workflows_in_order, items) = parse_input(&lines);
    let mut sum = 0;
    for item in items {
        if evaluate_rec(&item, &workflows, "in".to_string()) {
            println!("Accepted {:?}", item);
            sum += item.total();
        } else {
            println!("Rejected {:?}", item);
        }
    }

    return sum;
}

fn get_range_combination(ranges: &HashMap<char, [i64; 2]>) -> i64 {
    let mut total = 1;
    for (_, val) in ranges.iter() {
        total *= val[1] - val[0] + 1;
    }

    return total;
}

fn part2(path: &str) -> i64 {
    let lines = read_lines(path);
    let (workflows, _, _) = parse_input(&lines);

    let part_ranges: HashMap<char, [i64; 2]> = vec![
        ('x', [1, 4000]),
        ('m', [1, 4000]),
        ('a', [1, 4000]),
        ('s', [1, 4000]),
    ]
    .into_iter()
    .collect();
    let mut stack = vec![(part_ranges, "in".to_string())];
    let mut ttl = 0;
    while let Some((mut ranges, pipeline)) = stack.pop() {
        if pipeline == "A" {
            ttl += get_range_combination(&ranges);
            continue;
        } else if pipeline == "R" {
            continue;
        }

        let conditions = workflows
            .get(&pipeline)
            .unwrap_or_else(|| panic!("Unknown pipeline {}", pipeline));
        for condition in conditions {
            println!("Assessing condition {:?}", condition);
            if condition.dest == "A".to_string() && condition.property == Property::Any {
                ttl += get_range_combination(&ranges);
                continue;
            } else if condition.dest == "R".to_string() && condition.property == Property::Any {
                continue;
            }

            if condition.property == Property::Any {
                stack.push((ranges, condition.dest.clone()));
                break;
            }

            let a = condition.property.to_char();
            let val = condition.value;
            let vals = ranges.remove(&a).unwrap().clone();

            let low = vals[0];
            let high = vals[1];
            let mut new_ranges = ranges.clone();

            if condition.gt {
                new_ranges.insert(a, [val + 1, high]);
                ranges.insert(a, [low, val]);
            } else {
                new_ranges.insert(a, [low, val - 1]);
                ranges.insert(a, [val, high]);
            }
            stack.push((new_ranges, condition.dest.clone()));
        }
    }

    return ttl;
}

fn evaluate(
    item: &Item,
    workflows: &HashMap<String, Vec<Condition>>,
    workflows_in_order: &Vec<String>,
) -> bool {
    // Returns true if accepted
    let mut init = None;

    for name in workflows_in_order {
        if workflows.get(name).unwrap()[0].meets_condition(item) {
            init = Some(name.clone());
            break;
        }
    }

    if init.is_none() {
        panic!("Could not find initial for {:?}", item);
    }

    return evaluate_rec(item, workflows, init.unwrap());
}

fn evaluate_rec(
    item: &Item,
    workflows: &HashMap<String, Vec<Condition>>,
    workflow: String,
) -> bool {
    // returns true if accepted
    println!("Evaluating item {:?}", item);
    println!("I'm on workflow {}", workflow);
    let conditions = workflows.get(&workflow).unwrap();

    let mut dest = None;

    for condition in conditions {
        println!("Looking at condition {:?}", condition);
        let meets = condition.meets_condition(item);

        println!("This item meets this condition {}", meets);

        if meets && condition.dest == "A" {
            return true;
        }

        if meets && condition.dest == "R" {
            return false;
        }

        if !meets {
            continue;
        }

        if meets {
            dest = Some(condition.dest.clone());
            break;
        }
    }

    if dest.is_none() {
        panic!("Unreachable code");
    }

    return evaluate_rec(item, workflows, dest.unwrap());
}

fn parse_input(lines: &Vec<String>) -> (HashMap<String, Vec<Condition>>, Vec<String>, Vec<Item>) {
    let mut processing_rules = true;

    let mut workflows: HashMap<String, Vec<Condition>> = HashMap::new();
    let mut workflows_in_order: Vec<String> = vec![];
    let mut items = vec![];

    for line in lines {
        if line == "" {
            processing_rules = false;
            continue;
        }

        if processing_rules {
            let split: Vec<&str> = line.split("{").collect();

            let name = split[0];
            let mut rules = split[1].to_string();
            rules.pop(); // Get rid of final }

            let rules_split: Vec<&str> = rules.split(",").collect();

            let mut conditions = vec![];
            for rule in rules_split {
                if !rule.contains(":") {
                    conditions.push(Condition {
                        property: Property::Any,
                        gt: true,
                        value: i64::MIN,
                        dest: rule.to_string(),
                    });
                    continue;
                }
                let rules_subsplit: Vec<&str> = rule.split(":").collect();

                let property = &rules_subsplit[0][0..1];
                let gt = &rules_subsplit[0][1..2];

                let gt_split: Vec<&str> = rules_subsplit[0].split(gt).collect();

                let num = gt_split[1];

                conditions.push(Condition::new(property, gt, num, &rules_subsplit[1]));
            }

            workflows.insert(name.to_string(), conditions);
            workflows_in_order.push(name.to_string());
        } else {
            let mut item = line.clone();

            item.pop();

            let item_split: Vec<&str> = item.split(",").collect();
            items.push(Item {
                x: get_num(item_split[0]),
                m: get_num(item_split[1]),
                a: get_num(item_split[2]),
                s: get_num(item_split[3]),
            });
        }
    }

    println!("Here's the items {:?}", items);
    println!("Here's the workflows {:?}", workflows);

    return (workflows, workflows_in_order, items);
}

fn get_num(item: &str) -> i64 {
    let split: Vec<&str> = item.split("=").collect();

    return split[1].parse().unwrap();
}

#[derive(Debug)]
struct Condition {
    pub property: Property,
    pub gt: bool,
    pub value: i64,
    pub dest: String,
}

impl Condition {
    pub fn new(property: &str, gt: &str, value: &str, dest: &str) -> Condition {
        let gt = if gt == ">" {
            true
        } else if gt == "<" {
            false
        } else {
            panic!("Unknown gt symbol {}", gt)
        };

        let property = Property::from(property);

        let value = value.parse::<i64>().unwrap();

        let dest = dest.to_string();

        return Self {
            property,
            gt,
            value,
            dest,
        };
    }

    pub fn meets_condition(&self, item: &Item) -> bool {
        let comp = match self.gt {
            true => greater_than,
            false => less_than,
        };

        match self.property {
            Property::ExtremelyCool => comp(item.x, self.value),
            Property::Musical => comp(item.m, self.value),
            Property::Aerodynamic => comp(item.a, self.value),
            Property::Shiny => comp(item.s, self.value),
            Property::Any => return true,
        }
    }
}

fn greater_than(a: i64, b: i64) -> bool {
    return a > b;
}

fn less_than(a: i64, b: i64) -> bool {
    return a < b;
}

#[derive(Debug, PartialEq)]
enum Property {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
    Any,
}

impl Property {
    pub fn to_char(&self) -> char {
        return match self {
            Property::ExtremelyCool => 'x',
            Property::Musical => 'm',
            Property::Aerodynamic => 'a',
            Property::Shiny => 's',
            Property::Any => panic!("Unreachable code"),
        };
    }
}

impl From<&str> for Property {
    fn from(value: &str) -> Self {
        if value == "x" {
            return Self::ExtremelyCool;
        } else if value == "m" {
            return Self::Musical;
        } else if value == "a" {
            return Self::Aerodynamic;
        } else if value == "s" {
            return Self::Shiny;
        } else {
            panic!("Unknown input {}", value);
        }
    }
}

#[derive(Debug)]
struct Item {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

impl Item {
    pub fn total(&self) -> i64 {
        return self.x + self.m + self.a + self.s;
    }
}

#[test]
fn test_part1() {
    let path = "data_demo/day19_demo.txt";
    let sum = part1(path);
    assert_eq!(19114, sum);
}

#[test]
fn test_part2() {
    let path = "data_demo/day19_demo.txt";
    let sum = part2(path);
    assert_eq!(167409079868000, sum);
}
