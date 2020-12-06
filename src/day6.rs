use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day6 {}

impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        let mut present = HashSet::new();
        let mut count = 0;
        for line in input.lines() {
            if line.trim().len() == 0 {
                count += present.len();
                present = HashSet::new();
            } else {
                for char in line.chars() {
                    present.insert(char);
                }
            }
        }
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut present = HashMap::new();
        let mut group_size = 0;
        let mut count = 0;
        for line in input.lines() {
            if line.trim().len() == 0 {
                for (_, i) in present {
                    if i == group_size {
                        count += 1;
                    }
                }
                group_size = 0;
                present = HashMap::new();
            } else {
                group_size += 1;
                for char in line.chars() {
                    present.entry(char).and_modify(|i| *i += 1).or_insert(1);
                }
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day6{}.part1("abc

a
b
c

ab
ac

a
a
a
a

b

"), "11")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day6{}.part2("abc

a
b
c

ab
ac

a
a
a
a

b

"), "6")
    }
}
