use crate::day::Day;

pub struct Day5 {}

fn seat_id(input: &str) -> i32 {
    input.chars().fold(0, |a, b| (a << 1) + match b {
        'F' => 0,
        'B' => 1,
        'L' => 0,
        'R' => 1,
        _ => panic!("Invalid character in seat code")
    })
}

impl Day for Day5 {
    fn part1(&self, input: &str) -> String {
        input.lines().map(|line| seat_id(line)).max().unwrap().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut sorted = input.lines().map(|line| seat_id(line)).collect::<Vec<i32>>();
        sorted.sort();
        let mut iter = sorted.iter();
        let mut previous = iter.next().unwrap();
        for i in iter {
            if i > &(previous + 1) {
                return (previous + 1).to_string();
            }
            previous = i;
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        SimpleLogger::new().init();
        assert_eq!(Day5{}.part1("FBFBBFFRLR"), "357")
    }
}
