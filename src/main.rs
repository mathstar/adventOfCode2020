mod day;
mod day1;
mod day2;

use simple_logger::SimpleLogger;
use std::fs;
use std::io;
use std::collections::HashMap;
use crate::day::Day;

fn main() {
    SimpleLogger::new().init().unwrap();

    println!("Choose day:");
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");

    let mut days:HashMap<&str, Box<dyn Day>> = HashMap::new();
    days.insert("1", Box::new(day1::Day1{}));
    days.insert("2", Box::new(day2::Day2{}));

    let trimmed_day = day.trim();
    match days.get(trimmed_day) {
        Some(day) => {
            let input = read_input(trimmed_day);
            println!("{}", day.part1(input.as_str()));
            println!("{}", day.part2(input.as_str()));
        }
        None => println!("Unknown day")
    }
}

fn read_input(day: &str) -> String {
    fs::read_to_string(format!("res/day{}.txt", day)).expect("Error reading input")
}
