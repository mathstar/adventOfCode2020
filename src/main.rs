mod day1;

use std::fs;
use std::io;

fn main() {
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
        _ => println!("Unknown day")
    }
}

fn read_input(day: u8) -> String {
    fs::read_to_string(format!("day{}.txt", day)).expect("Error reading input")
}
