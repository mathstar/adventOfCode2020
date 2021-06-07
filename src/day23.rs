use crate::day::Day;
use std::cmp::min;

pub struct Day23 {
    pub move_count: u32
}

fn find_cup(cup: u32, cups: &Vec<u32>) -> usize {
    cups.iter().enumerate().filter(|(_,c)| **c == cup).next().unwrap().0
}

impl Day for Day23 {
    fn part1(&self, input: &str) -> String {
        let mut cups = input.lines().next().unwrap().chars().map(|c| c as u32 - '0' as u32).collect::<Vec<u32>>();
        let mut current_cup = cups[0];
        let min_cup = *cups.iter().min().unwrap();
        let max_cup = *cups.iter().max().unwrap();

        for _ in 0..self.move_count {
            let current_cup_index = find_cup(current_cup, &cups);
            let next_cup_index = current_cup_index + 1;
            let cup0 = if next_cup_index < cups.len() {cups.remove(next_cup_index)} else {cups.remove(0)};
            let cup1 = if next_cup_index < cups.len() {cups.remove(next_cup_index)} else {cups.remove(0)};
            let cup2 = if next_cup_index < cups.len() {cups.remove(next_cup_index)} else {cups.remove(0)};

            let mut destination_cup = current_cup - 1;
            if destination_cup < min_cup { destination_cup = max_cup; }
            while destination_cup == cup0 || destination_cup == cup1 || destination_cup == cup2 {
                destination_cup -= 1;
                if destination_cup < min_cup {
                    destination_cup = max_cup;
                }
            }
            let mut destination_index = find_cup(destination_cup, &cups) + 1;
            if destination_index > cups.len() {
                cups.push(cup0);
                cups.push(cup1);
                cups.push(cup2);
            } else {
                cups.insert(destination_index, cup0);
                cups.insert(destination_index + 1, cup1);
                cups.insert(destination_index + 2, cup2);

            }

            let next_current_cup_index = find_cup(current_cup, &cups) + 1;
            current_cup = if next_current_cup_index < cups.len() {cups[next_current_cup_index]} else {cups[0]}
        }

        let mut result = "".to_owned();
        let mut initial = find_cup(1, &cups);
        let mut i = initial + 1;
        if i >= cups.len() {i = 0};
        while i != initial {
            result += cups[i].to_string().as_str();
            i += 1;
            if i >= cups.len() {i = 0};
        }
        result
    }

    fn part2(&self, input: &str) -> String {

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day23{move_count: 10}.part1("389125467"), "92658374");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day23{move_count: 10}.part2(""), "");
    }
}
