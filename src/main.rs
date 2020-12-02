mod day1;
mod day2;

use simple_logger::SimpleLogger;
use std::fs;
use std::io;

fn main() {
    SimpleLogger::new().init().unwrap();

    println!("Choose day:");
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");

    match day.trim() {
        "1" => {
            let input = read_input(1);
            println!("{}", day1::part1(input.as_str()));
            println!("{}", day1::part2(input.as_str()));
        },
        "1a" => {
            let input = read_input(1);
            println!("{}", day1::part1(input.as_str()));
        },
        "1b" => {
            let input = read_input(1);
            println!("{}", day1::part2(input.as_str()));
        },
        "2" => {
            let input = read_input(2);
            println!("{}", day2::part1(input.as_str()));
            println!("{}", day2::part2(input.as_str()));
        },
        "2a" => {
            let input = read_input(2);
            println!("{}", day2::part1(input.as_str()));
        },
        "2b" => {
            let input = read_input(2);
            println!("{}", day2::part2(input.as_str()));
        },
        _ => println!("Unknown day")
    }
}

fn read_input(day: u8) -> String {
    fs::read_to_string(format!("res/day{}.txt", day)).expect("Error reading input")
}
