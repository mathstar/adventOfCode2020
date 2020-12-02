use crate::day::Day;

pub struct Day2 {}

impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let lines = input.split("\n");
        let mut valid = 0;
        for line in lines {
            let line_split = line.split_whitespace().collect::<Vec<&str>>();
            let range = line_split[0];
            let range_split = range.split("-").collect::<Vec<&str>>();
            let low = range_split[0].parse::<i32>().unwrap();
            let high = range_split[1].parse::<i32>().unwrap();
            let rule_character = line_split[1].chars().next().unwrap();
            let password = line_split[2];

            let mut count = 0;
            for character in password.chars() {
                if character == rule_character {
                    count = count + 1;
                }
            }

            if count >= low && count <= high {
                valid = valid + 1;
            }
        }
        valid.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let lines = input.split("\n");
        let mut valid = 0;
        for line in lines {
            let line_split = line.split_whitespace().collect::<Vec<&str>>();
            let range = line_split[0];
            let range_split = range.split("-").collect::<Vec<&str>>();
            let low = range_split[0].parse::<usize>().unwrap();
            let high = range_split[1].parse::<usize>().unwrap();
            let rule_character = line_split[1].chars().next().unwrap();
            let password = line_split[2];

            let characters = password.chars().collect::<Vec<char>>();
            log::debug!("{} {} {} {}", password, low, high, rule_character);
            if low - 1 < characters.len() {
                let low_char = characters[low - 1];
                log::debug!("-- low {}", low_char);
                if high - 1 < characters.len() {
                    let high_char = characters[high - 1];
                    log::debug!("-- high {}", high_char);
                    if (low_char == rule_character && high_char != rule_character) || (high_char == rule_character && low_char != rule_character) {
                        valid = valid + 1;
                        log::debug!("-- valid");
                    }
                } else if low_char == rule_character {
                    valid = valid + 1;
                    log::debug!("-- valid");
                }
            }
        }
        valid.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day2{}.part1("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), "2")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day2{}.part2("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), "1")
    }
}