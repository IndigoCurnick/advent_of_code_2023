use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let path = "data/day1.txt";

    let lines = read_lines(path);

    let mut map: HashMap<&str, char> = HashMap::new();
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');
    map.insert("1", '1');
    map.insert("2", '2');
    map.insert("3", '3');
    map.insert("4", '4');
    map.insert("5", '5');
    map.insert("6", '6');
    map.insert("7", '7');
    map.insert("8", '8');
    map.insert("9", '9');

    let mut sum: i32 = 0;
    for line in lines {
        let mut first = usize::MAX;
        let mut first_len = 1;
        let mut last = usize::MIN;
        let mut last_len = 1;

        for key in map.keys() {
            match line.find(key) {
                Some(x) => {
                    if x < first {
                        first = x;
                        first_len = key.len()
                    }
                }
                None => {}
            }
            match line.rfind(key) {
                Some(x) => {
                    if x > last {
                        last = x;
                        last_len = key.len()
                    }
                }
                None => {}
            }
        }

        let first_key =
            String::from_utf8(line.as_bytes()[first..(first + first_len)].to_vec()).unwrap();
        let last_key =
            String::from_utf8(line.as_bytes()[last..(last + last_len)].to_vec()).unwrap();

        let calibration = format!(
            "{}{}",
            map.get(first_key.as_str()).unwrap(),
            map.get(last_key.as_str()).unwrap()
        );

        sum += calibration.parse::<i32>().unwrap();
    }

    println!("Calibration Total: {}", sum);
}

fn read_lines(path: &str) -> Vec<String> {
    // Attempt to open the file in read-only mode
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error opening file: {}", e);
        }
    };

    // Create a buffer to hold the file contents
    let mut contents = String::new();

    // Read the file contents into the buffer
    file.read_to_string(&mut contents).unwrap();

    let split: Vec<String> = contents
        .split("\n")
        .into_iter()
        .map(|c| c.to_string())
        .collect();

    return split;
}

fn part_one() {
    let mut sum = 0;

    // Specify the path to the file
    let file_path = "data/day1_part1.txt";

    let split = read_lines(file_path);

    for line in split {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in line.chars() {
            let digit = c.to_digit(10);

            if digit.is_some() {
                if first.is_none() {
                    first = Some(c);
                    last = Some(c);
                } else if last.is_some() {
                    last = Some(c);
                }
            }
        }

        if first.is_none() || last.is_none() {
            panic!("Could not find two digits");
        }

        let this_calibration = format!("{}{}", first.unwrap(), last.unwrap());

        let this_calibration = this_calibration.parse::<i32>().unwrap();

        sum += this_calibration;
    }

    println!("Final Calibration Sum: {}", sum);
    // Correct Answer: 54081
}
