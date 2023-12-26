use crate::read_lines;

pub fn day4() {
    let path = "data/day4.txt";
    let sum = part1(path);
    println!("Day 4 Part 1: {}", sum);
    let sum = part2(path);
    println!("Day 4 Part 2: {}", sum);
}

fn part1(path: &str) -> i32 {
    let lines = read_lines(path);

    let mut sum = 0;

    for line in lines {
        let game_split: Vec<&str> = line.split(":").collect();

        assert_eq!(game_split.len(), 2, "The splitting has gone wrong!");

        let total_wins = handle_game(game_split[1]);

        sum += total_wins_to_score(total_wins);
    }

    return sum;
}

fn total_wins_to_score(total_wins: i32) -> i32 {
    if total_wins == 0 {
        return 0;
    }

    let index = total_wins as u32 - 1_u32;
    let score: i32 = 2_i32.pow(index);

    return score;
}

fn handle_game(game_split: &str) -> i32 {
    let mut total_wins = 0;

    let winning_split: Vec<&str> = game_split.split("|").collect();

    assert_eq!(winning_split.len(), 2, "The splitting has gone wrong!");

    let winning_numbers = winning_split[0];

    let my_numbers = winning_split[1];

    let winning_numbers = extract_numbers(winning_numbers);
    let my_numbers = extract_numbers(my_numbers);

    for my_number in my_numbers {
        if !winning_numbers.contains(&my_number) {
            continue;
        }

        total_wins += 1;
    }

    return total_wins;
}

fn extract_numbers(nums: &str) -> Vec<i32> {
    let nums = nums.trim();

    let mut numbers = vec![];

    let num_split: Vec<&str> = nums.split(" ").collect();

    for num in num_split {
        if num == "" {
            // This handles the case of single digit number in the list, which has an additional whitespace
            continue;
        }
        let number = num.parse::<i32>().expect("Did not find number to parse");
        numbers.push(number);
    }

    return numbers;
}

fn part2(path: &str) -> i32 {
    let lines = read_lines(path);

    let total_games = lines.len();

    // Goes from card number to total number held
    // let mut total_scratch_cards: HashMap<i32, i32> = HashMap::new();

    let mut total_scratch_cards = Vec::with_capacity(total_games);

    // Goes from card number to number of game of cards you win
    // let mut scratch_card_winnings: HashMap<i32, i32> = HashMap::new();

    let mut scratch_card_winnings = Vec::with_capacity(total_games);

    for (i, line) in lines.iter().enumerate() {
        total_scratch_cards.push(1);

        let game_split: Vec<&str> = line.split(":").collect();

        assert_eq!(game_split.len(), 2, "The splitting has gone wrong!");

        let total_wins = handle_game(game_split[1]);

        scratch_card_winnings.push(total_wins);
    }

    for (game_number, cards_won) in scratch_card_winnings.iter().enumerate() {
        if *cards_won == 0 {
            continue;
        }

        let multiplier = total_scratch_cards.get(game_number).unwrap().clone();

        for i in 1..=*cards_won {
            let winning_card_number = game_number + i as usize;

            let cur = total_scratch_cards.get_mut(winning_card_number).unwrap();
            *cur += multiplier;
        }
    }

    let sum = total_scratch_cards.iter().sum();

    return sum;
}

#[test]
fn test_part1() {
    let path = "data_demo/day4_demo.txt";
    let sum = part1(path);
    assert_eq!(sum, 13);
}

#[test]
fn test_part2() {
    let path = "data_demo/day4_demo.txt";
    let sum = part2(path);
    assert_eq!(sum, 30);
}
