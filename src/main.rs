mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
    days.insert("3", Box::new(day3::Day3{}));
    days.insert("4", Box::new(day4::Day4{}));
    days.insert("5", Box::new(day5::Day5{}));
    days.insert("6", Box::new(day6::Day6{}));
    days.insert("7", Box::new(day7::Day7{}));
    days.insert("8", Box::new(day8::Day8{}));
    days.insert("9", Box::new(day9::Day9{preamble_length: 25}));
    days.insert("10", Box::new(day10::Day10{}));
    days.insert("11", Box::new(day11::Day11{}));
    days.insert("12", Box::new(day12::Day12{}));
    days.insert("13", Box::new(day13::Day13{}));
    days.insert("14", Box::new(day14::Day14{}));
    days.insert("15", Box::new(day15::Day15{}));
    days.insert("16", Box::new(day16::Day16{}));
    days.insert("17", Box::new(day17::Day17{}));
    days.insert("18", Box::new(day18::Day18{}));
    days.insert("19", Box::new(day19::Day19{}));
    days.insert("20", Box::new(day20::Day20{}));
    days.insert("21", Box::new(day21::Day21{}));
    days.insert("22", Box::new(day22::Day22{}));
    days.insert("23", Box::new(day23::Day23{move_count: 100}));
    days.insert("24", Box::new(day24::Day24{}));
    days.insert("25", Box::new(day25::Day25{}));

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
