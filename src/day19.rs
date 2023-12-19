use std::collections::HashMap;

use crate::read_lines;

pub fn day19() {}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    let (workflows, items) = parse_input(&lines);

    for item in items {}

    todo!();
}

fn evaluate(item: &Item, workflows: &HashMap<String, Vec<Condition>>) -> bool {}

fn parse_input(lines: &Vec<String>) -> (HashMap<String, Vec<Condition>>, Vec<Item>) {
    let mut processing_rules = true;

    let mut workflows: HashMap<String, Vec<Condition>> = HashMap::new();
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
                    // TODO: Handle special end case
                    conditions.push(Condition {
                        property: Property::Any,
                        gt: true,
                        value: i32::MIN,
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

    return (workflows, items);
}

fn get_num(item: &str) -> i32 {
    let split: Vec<&str> = item.split("=").collect();

    return split[1].parse().unwrap();
}

struct Condition {
    pub property: Property,
    pub gt: bool,
    pub value: i32,
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

        let value = value.parse::<i32>().unwrap();

        let dest = dest.to_string();

        return Self {
            property,
            gt,
            value,
            dest,
        };
    }

    pub fn meets_condition(&self, item: &Item) -> bool {
        match self.property {
            Property::ExtremelyCool => todo!(),
            Property::Musical => todo!(),
            Property::Aerodynamic => todo!(),
            Property::Shiny => todo!(),
            Property::Any => return true,
        }
    }
}

enum Property {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
    Any,
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

struct Item {
    pub x: i32,
    pub m: i32,
    pub a: i32,
    pub s: i32,
}
