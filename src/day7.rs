use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use crate::read_lines;

pub fn day7() {
    let data = "data/day7.txt";
    let sum = part1(data);
    println!("Day 7 Part 1: {}", sum);
    let sum = part2(data);
    println!("Day 7 Part 2: {}", sum);
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    let mut hands = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        assert_eq!(split.len(), 2);

        let bid = split[1].parse::<i32>().unwrap();

        let hand = split[0].to_string();

        hands.push(Hand::new(hand, bid, false));
    }

    hands.sort_by(|a, b| a.cmp(b));

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as i32 * hand.bid;
    }

    return sum;
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);

    let mut hands = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        assert_eq!(split.len(), 2);

        let bid = split[1].parse::<i32>().unwrap();

        let hand = split[0].to_string();

        hands.push(Hand::new(hand, bid, true));
    }

    hands.sort_by(|a, b| a.alt_cmp(b));

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as i32 * hand.bid;
    }

    return sum;
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Hand {
    pub hand: String,
    pub cards: Vec<i32>,
    pub hand_type: HandType,
    pub bid: i32,
}

impl Hand {
    pub fn new(hand: String, bid: i32, alt: bool) -> Self {
        let mut cards: Vec<i32> = vec![];

        for c in hand.chars() {
            if c.is_digit(10) {
                cards.push(c.to_digit(10).unwrap() as i32);
            } else if c == 'T' {
                cards.push(10);
            } else if c == 'J' {
                cards.push(11);
            } else if c == 'Q' {
                cards.push(12);
            } else if c == 'K' {
                cards.push(13)
            } else if c == 'A' {
                cards.push(14);
            } else {
                panic!("Could not parse hand {}", hand);
            }
        }

        assert_eq!(cards.len(), 5);

        let hand_type = if alt {
            HandType::new_alt(&cards)
        } else {
            HandType::new(&cards)
        };

        return Self {
            hand: hand,
            cards: cards,
            hand_type: hand_type,
            bid: bid,
        };
    }

    pub fn alt_cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card == other_card {
                    continue;
                } else if *self_card == 11 {
                    return Ordering::Less;
                } else if *other_card == 11 {
                    return Ordering::Greater;
                } else if self_card > other_card {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }

        return self.hand_type.cmp(&other.hand_type);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card == other_card {
                    continue;
                } else if self_card > other_card {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }

        return self.hand_type.cmp(&other.hand_type);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) ({:?})", self.hand, self.hand_type, self.cards)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "Five of a Kind"),
            HandType::FourOfAKind => write!(f, "Four of a Kind"),
            HandType::FullHouse => write!(f, "Full House"),
            HandType::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::OnePair => write!(f, "One Pair"),
            HandType::HighCard => write!(f, "High Card"),
        }
    }
}

impl HandType {
    pub fn new(cards: &Vec<i32>) -> Self {
        assert_eq!(cards.len(), 5);
        let mut hash: HashMap<i32, i32> = HashMap::new();

        for card in cards {
            if hash.contains_key(card) {
                let c = hash.get_mut(card).unwrap();
                *c += 1;
            } else {
                hash.insert(card.clone(), 1);
            }
        }

        let mut vals: Vec<i32> = hash.values().cloned().collect();
        vals.sort_by(|a, b| b.cmp(a));

        if vals[0] == 5 {
            return Self::FiveOfAKind;
        } else if vals[0] == 4 {
            return Self::FourOfAKind;
        } else if vals[0] == 3 && vals[1] == 2 {
            return Self::FullHouse;
        } else if vals[0] == 3 {
            return Self::ThreeOfAKind;
        } else if vals[0] == 2 && vals[1] == 2 {
            return Self::TwoPair;
        } else if vals[0] == 2 {
            return Self::OnePair;
        } else {
            return Self::HighCard;
        }
    }

    pub fn new_alt(cards: &Vec<i32>) -> Self {
        assert_eq!(cards.len(), 5);
        if !cards.contains(&11) {
            // If there's no Jack, nothing changes
            return Self::new(cards);
        }

        let num_jacks = cards.iter().filter(|&&num| num == 11).count() as i32;

        let mut hash: HashMap<i32, i32> = HashMap::new();

        for card in cards {
            if *card == 11 {
                continue;
            }

            if hash.contains_key(card) {
                let c = hash.get_mut(card).unwrap();
                *c += 1;
            } else {
                hash.insert(card.clone(), 1);
            }
        }

        let mut vals: Vec<i32> = hash.values().cloned().collect();
        vals.sort_by(|a, b| b.cmp(a));

        // Special case - all jacks
        if num_jacks == 5 {
            return Self::FiveOfAKind;
        }

        let zeroth = vals.get_mut(0).unwrap();
        *zeroth += num_jacks;

        if vals[0] == 5 {
            return Self::FiveOfAKind;
        } else if vals[0] == 4 {
            return Self::FourOfAKind;
        } else if vals[0] == 3 && vals[1] == 2 {
            return Self::FullHouse;
        } else if vals[0] == 3 {
            return Self::ThreeOfAKind;
        } else if vals[0] == 2 && vals[1] == 2 {
            return Self::TwoPair;
        } else if vals[0] == 2 {
            return Self::OnePair;
        } else {
            return Self::HighCard;
        }
    }
}

#[test]
fn test_part1() {
    let data = "data_demo/day7_demo.txt";
    let hand = part1(data);
    assert_eq!(hand, 6440);
}

#[test]
fn test_part2() {
    let data = "data_demo/day7_demo.txt";
    let hand = part2(data);
    assert_eq!(hand, 5905);
}
