use crate::day::Day;

pub struct Day5 {}

fn seat_id(input: &str) -> i32 {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut seat_min = 0;
    let mut seat_max = 7;
    for char in input.chars() {
        match char {
            'F' => row_max = row_max - (row_max - row_min) / 2 - 1,
            'B' => row_min = ((row_max - row_min) / 2) + row_min + 1,
            'L' => seat_max = seat_max - (seat_max - seat_min) / 2 - 1,
            'R' => seat_min = ((seat_max - seat_min) / 2) + seat_min + 1,
            _ => panic!("Invalid character in seat code")
        }
        //log::debug!("{} {} {} {} {}", char, row_min, row_max, seat_min, seat_max);
    }
    assert_eq!(row_min, row_max);
    assert_eq!(seat_min, seat_max);
    row_min * 8 + seat_min
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
