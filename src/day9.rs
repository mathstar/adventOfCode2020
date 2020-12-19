use crate::day::Day;
use std::collections::VecDeque;

pub struct Day9 {
    pub preamble_length: usize
}

fn find_mismatch(buffer: &mut VecDeque<i64>, current: &i64) -> Option<i64> {
    for i in 0..buffer.len() {
        for j in i+1..buffer.len() {
            let a = buffer[i];
            let b = buffer[j];
            if a != b && a + b == *current {
                buffer.pop_front();
                buffer.push_back(*current);
                return None;
            }
        }
    }
    return Some(*current);
}

fn find_invalid(input: &str, preamble_length: usize) -> i64 {
    let mut buffer: VecDeque<i64> = VecDeque::new();
    for line in input.lines() {
        if buffer.len() < preamble_length {
            buffer.push_back(line.parse().unwrap());
        } else {
            let current = line.parse().unwrap();
            match find_mismatch(&mut buffer, &current) {
                Some(n) => return n,
                None => ()
            }
        }
    }
    panic!("no invalid found")
}

impl Day for Day9 {
    fn part1(&self, input: &str) -> String {
        find_invalid(input, self.preamble_length).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let invalid = find_invalid(input, self.preamble_length);
        let all_values = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<i64>>();
        for i in 0..all_values.len() {
            for j in i+1..all_values.len() {
                let sum:i64 = all_values[i..j].iter().sum();
                if sum == invalid {
                    return (all_values[i..j].iter().min().unwrap() + all_values[i..j].iter().max().unwrap()).to_string()
                } else if sum > invalid {
                    break;
                }
            }
        }
        String::new()
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
        assert_eq!(Day9{preamble_length: 5}.part1("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"), "127")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day9{preamble_length: 5}.part2("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"), "62")
    }
}
