mod day1;

use std::io;

fn main() {
    println!("Choose day:");
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");

    match day.trim() {
        "1" => {
            day1::part1();
            day1::part2();
        },
        "1a" => day1::part1(),
        "1b" => day1::part2(),
        _ => println!("Unknown day")
    }
}
