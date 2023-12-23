use std::collections::{HashMap, VecDeque};

use crate::read_lines;

pub fn day20() {
    let path = "data/day20.txt";
    let mult = part1(path);
    println!("Day 20 Part 1 {}", mult);
    let button = part2(path);
    println!("Day 20 Part 2 {}", button);
}

fn part1(path: &str) -> i64 {
    let lines = read_lines(path);
    let mut map = parse_input(lines);
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        // Button
        low_pulses += 1;

        let mut stack: VecDeque<Message> = VecDeque::new();

        stack.push_back(Message {
            source: "button".to_string(),
            destination: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(message) = stack.pop_front() {
            let module = match map.get_mut(&message.destination) {
                Some(x) => x,
                None => continue, // There are some "loose" modules that only act as sinks
            };

            match module {
                Module::Broadcast(b) => {
                    // We can kind of ignore pulses here because we know it's going to happen on low
                    for dest in &b.destinations {
                        stack.push_back(Message {
                            source: message.destination.clone(),
                            destination: dest.clone(),
                            pulse: Pulse::Low,
                        });
                        low_pulses += 1;
                    }
                }
                Module::FlipFlop(f) => {
                    if message.pulse == Pulse::High {
                        continue;
                    }

                    if f.on {
                        f.on = false;

                        for dest in &f.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::Low,
                            });
                            low_pulses += 1;
                        }
                    } else {
                        f.on = true;
                        for dest in &f.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::High,
                            });
                            high_pulses += 1;
                        }
                    }
                }
                Module::Conjuction(c) => {
                    let a = c.last_pulse.get_mut(&message.source).unwrap();
                    *a = message.pulse.clone();

                    if c.all_high() {
                        for dest in &c.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::Low,
                            });
                            low_pulses += 1;
                        }
                    } else {
                        for dest in &c.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::High,
                            });
                            high_pulses += 1;
                        }
                    }
                }
            }
        }
    }

    return high_pulses * low_pulses;
}

fn parse_input(lines: Vec<String>) -> HashMap<String, Module> {
    let mut map = HashMap::new();

    for line in lines {
        let big_split: Vec<&str> = line.split("->").collect();

        let name = big_split[0].trim();

        let dests = big_split[1].trim();
        let comma_split: Vec<&str> = dests.split(",").collect();

        let mut destinations = vec![];

        for item in comma_split {
            let destination = item.trim();
            destinations.push(destination.to_string());
        }

        let (module, real_name) = if name == "broadcaster" {
            // Broadcaster
            (
                Module::Broadcast(Broadcast {
                    name: name.to_string(),
                    destinations,
                }),
                name.to_string(),
            )
        } else if name.contains("%") {
            // FlipFlop
            let name_len = name.len();
            let real_name = &name[1..name_len];
            (
                Module::FlipFlop(FlipFlop {
                    name: real_name.to_string(),
                    on: false,
                    destinations,
                }),
                real_name.to_string(),
            )
        } else if name.contains("&") {
            // Conjunction
            let name_len = name.len();
            let real_name = &name[1..name_len];
            (
                Module::Conjuction(Conjunction {
                    name: real_name.to_string(),
                    last_pulse: HashMap::new(),
                    destinations,
                }),
                real_name.to_string(),
            )
        } else {
            panic!("Unknown input");
        };

        map.insert(real_name, module);
    }

    // Initialise the conjugation modules
    for (name, module) in map.clone().iter() {
        match module {
            Module::Broadcast(b) => {
                for dest in &b.destinations {
                    match map.get_mut(dest).unwrap() {
                        Module::Broadcast(_) => {}
                        Module::FlipFlop(_) => {}
                        Module::Conjuction(c) => {
                            c.last_pulse.insert(name.clone(), Pulse::Low);
                        }
                    }
                }
            }
            Module::FlipFlop(f) => {
                for dest in &f.destinations {
                    match map.get_mut(dest).unwrap() {
                        Module::Broadcast(_) => {}
                        Module::FlipFlop(_) => {}
                        Module::Conjuction(c) => {
                            c.last_pulse.insert(name.clone(), Pulse::Low);
                        }
                    }
                }
            }
            Module::Conjuction(c) => {
                for dest in &c.destinations {
                    let m = match map.get_mut(dest) {
                        Some(x) => x,
                        None => continue,
                    };

                    match m {
                        Module::Broadcast(_) => {}
                        Module::FlipFlop(_) => {}
                        Module::Conjuction(c) => {
                            c.last_pulse.insert(name.clone(), Pulse::Low);
                        }
                    }
                }
            }
        }
    }

    return map;
}

fn part2(path: &str) -> i64 {
    // lh, fk, ff, mm
    let lines = read_lines(path);
    let mut map = parse_input(lines);

    let mut running = true;
    let mut button_press = 0;

    let mut button_press_map = HashMap::new();

    let conj1 = "lh".to_string();
    let conj2 = "fk".to_string();
    let conj3 = "ff".to_string();
    let conj4 = "mm".to_string();

    while running {
        // Button
        button_press += 1;

        let mut stack: VecDeque<Message> = VecDeque::new();

        stack.push_back(Message {
            source: "button".to_string(),
            destination: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(message) = stack.pop_front() {
            // if message.destination == "rx".to_string() && message.pulse == Pulse::Low {
            //     running = false;
            //     break;
            // }

            if button_press_map.contains_key(&conj1)
                && button_press_map.contains_key(&conj2)
                && button_press_map.contains_key(&conj3)
                && button_press_map.contains_key(&conj4)
            {
                running = false;
                break;
            }

            let module = match map.get_mut(&message.destination) {
                Some(x) => x,
                None => continue, // There are some "loose" modules that only act as sinks
            };

            match module {
                Module::Broadcast(b) => {
                    // We can kind of ignore pulses here because we know it's going to happen on low
                    for dest in &b.destinations {
                        stack.push_back(Message {
                            source: message.destination.clone(),
                            destination: dest.clone(),
                            pulse: Pulse::Low,
                        });
                    }
                }
                Module::FlipFlop(f) => {
                    if message.pulse == Pulse::High {
                        continue;
                    }

                    if f.on {
                        f.on = false;

                        for dest in &f.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::Low,
                            });
                        }
                    } else {
                        f.on = true;
                        for dest in &f.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::High,
                            });
                        }
                    }
                }
                Module::Conjuction(c) => {
                    let a = c.last_pulse.get_mut(&message.source).unwrap();
                    *a = message.pulse.clone();

                    if c.all_high() {
                        for dest in &c.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::Low,
                            });
                        }
                    } else {
                        if *c.name == conj1 && !button_press_map.contains_key(&conj1) {
                            button_press_map.insert(conj1.clone(), button_press);
                        } else if *c.name == conj2 && !button_press_map.contains_key(&conj2) {
                            button_press_map.insert(conj2.clone(), button_press);
                        } else if *c.name == conj3 && !button_press_map.contains_key(&conj3) {
                            button_press_map.insert(conj3.clone(), button_press);
                        } else if *c.name == conj4 && !button_press_map.contains_key(&conj4) {
                            button_press_map.insert(conj4.clone(), button_press);
                        }
                        for dest in &c.destinations {
                            stack.push_back(Message {
                                source: message.destination.clone(),
                                destination: dest.clone(),
                                pulse: Pulse::High,
                            });
                        }
                    }
                }
            }
        }
    }

    let min = button_press_map.get(&conj1).unwrap()
        * button_press_map.get(&conj2).unwrap()
        * button_press_map.get(&conj3).unwrap()
        * button_press_map.get(&conj4).unwrap();

    return min;
}

#[derive(Clone, Debug)]
enum Module {
    Broadcast(Broadcast),
    FlipFlop(FlipFlop),
    Conjuction(Conjunction),
}

#[derive(Clone, Debug)]
struct Broadcast {
    pub name: String,
    pub destinations: Vec<String>,
}

#[derive(Clone, Debug)]
struct FlipFlop {
    pub name: String,
    pub on: bool,
    pub destinations: Vec<String>,
}

#[derive(Clone, Debug)]
struct Conjunction {
    pub name: String,
    pub last_pulse: HashMap<String, Pulse>,
    pub destinations: Vec<String>,
}

impl Conjunction {
    pub fn all_high(&self) -> bool {
        for (_, input) in self.last_pulse.iter() {
            if *input == Pulse::Low {
                return false;
            }
        }

        return true;
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Pulse {
    High,
    Low,
}

struct Message {
    pub source: String,
    pub destination: String,
    pub pulse: Pulse,
}

#[test]
fn test_part1() {
    let path = "data/day20_demo.txt";
    let mult = part1(path);
    assert_eq!(mult, 32000000);
}
