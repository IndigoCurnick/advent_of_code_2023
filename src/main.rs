use clap::{Parser, ValueEnum};
use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use std::{fs::File, io::Read};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

#[derive(Parser)]
#[command(
    author = "Nathaniel Curnick nathaniel.curnick@gmail.com",
    version = "0.0.1"
)]
struct Cli {
    #[clap(long, short)]
    /// Day you wish to run
    pub day: Day,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
}

fn main() {
    let cli = Cli::parse();
    let day = cli.day;

    match day {
        Day::Day1 => day1(),
        Day::Day2 => day2(),
        Day::Day3 => day3(),
        Day::Day4 => day4(),
    }
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
