use clap::{Parser, ValueEnum};
use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
use day16::day16;
use day17::day17;
use day18::day18;
use day19::day19;
use day2::day2;
use day20::day20;
use day21::day21;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;
use std::{fs::File, io::Read};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
}

fn main() {
    let cli = Cli::parse();
    let day = cli.day;

    match day {
        Day::Day1 => day1(),
        Day::Day2 => day2(),
        Day::Day3 => day3(),
        Day::Day4 => day4(),
        Day::Day5 => day5(),
        Day::Day6 => day6(),
        Day::Day7 => day7(),
        Day::Day8 => day8(),
        Day::Day9 => day9(),
        Day::Day10 => day10(),
        Day::Day11 => day11(),
        Day::Day12 => day12(),
        Day::Day13 => day13(),
        Day::Day14 => day14(),
        Day::Day15 => day15(),
        Day::Day16 => day16(),
        Day::Day17 => day17(),
        Day::Day18 => day18(),
        Day::Day19 => day19(),
        Day::Day20 => day20(),
        Day::Day21 => day21(),
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
