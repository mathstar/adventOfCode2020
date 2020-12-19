use crate::day::Day;
use std::collections::HashMap;

pub struct Day15 {}

impl Day for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut numbers : Vec<i32> = input.lines().next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let mut occurrences:HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..2020 {
            let next;
            if i < numbers.len() {
                next = numbers[i];
            } else {
                next = occurrences.get(&numbers[i-1])
                    .map_or_else(|| 0, |v| if v.len() == 1 {
                        0
                    } else {
                        i as i32 - v[v.len() - 2] - 1
                    });
                numbers.push(next);
            };
            occurrences.entry(next).or_insert(Vec::new()).push(i as i32);
        }
        log::debug!("{:?}", numbers);
        numbers[2019].to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut numbers : Vec<i32> = input.lines().next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let mut occurrences:HashMap<i32, (i32,i32)> = HashMap::new();
        for i in 0..30000000 {
            let next;
            if i < numbers.len() {
                next = numbers[i];
            } else {
                next = occurrences.get(&numbers[i-1])
                    .map_or_else(|| 0, |v| if v.0 == -1 {
                        0
                    } else {
                        i as i32 - v.0 - 1
                    });
                numbers.push(next);
            };
            let mut entry = occurrences.entry(next).or_insert((-1,-1));
            entry.0 = entry.1;
            entry.1 = i as i32;
        }
        numbers[29999999].to_string()
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        SimpleLogger::new().init();
        assert_eq!(Day15{}.part1("0,3,6"), "436");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day15{}.part2("0,3,6"), "175594");
    }
}
