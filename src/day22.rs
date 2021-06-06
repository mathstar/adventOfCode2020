use crate::day::Day;
use std::collections::{VecDeque, HashSet};

pub struct Day22 {}

fn parse_input(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let (mut p1, mut p2) = (VecDeque::new(), VecDeque::new());
    let mut in_p1 = true;
    for line in input.lines().skip(1) {
        match line.parse() {
            Ok(n) => {
                if in_p1 {
                    p1.push_back(n);
                } else {
                    p2.push_back(n);
                }
            },
            Err(_) => {
                in_p1 = false;
            }
        }
    }
    (p1, p2)
}

fn recursive_combat(mut p1: VecDeque<u32>, mut p2: VecDeque<u32>) -> (bool, VecDeque<u32>) {
    let mut previous_hands = HashSet::new();
    while p1.len() > 0 && p2.len() > 0 {
        let hands = (p1.clone(), p2.clone());
        if previous_hands.contains(&hands) {
            return (true, p1);
        }
        previous_hands.insert(hands);

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        if p1_card <= p1.len() as u32 && p2_card <= p2.len() as u32 {
            match recursive_combat(p1.iter().map(|n| *n).take(p1_card as usize).collect(), p2.iter().map(|n| *n).take(p2_card as usize).collect()) {
                (true, _) => {
                    p1.push_back(p1_card);
                    p1.push_back(p2_card);
                },
                (false, _) => {
                    p2.push_back(p2_card);
                    p2.push_back(p1_card);
                }
            }
        } else {
            if p1_card > p2_card {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            } else {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        }
    }

    if p1.len() > 0 {(true, p1)} else {(false, p2)}
}

impl Day for Day22 {
    fn part1(&self, input: &str) -> String {
        let (mut p1, mut p2) = parse_input(input);
        while p1.len() > 0 && p2.len() > 0 {
            let p1_card = p1.pop_front().unwrap();
            let p2_card = p2.pop_front().unwrap();

            if p1_card > p2_card {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            } else {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        }

        let winner = if p1.len() > 0 {p1} else {p2};
        winner.iter().rev().enumerate().map(|(i,v)| (i+1) as u32 * v).sum::<u32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (p1, p2) = parse_input(input);

        let (_, winner) = recursive_combat(p1, p2);
        winner.iter().rev().enumerate().map(|(i,v)| (i+1) as u32 * v).sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day22{}.part1("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"), "306");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day22{}.part2("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"), "291");
    }

    #[test]
    fn part2_test2() {
        // testing for termination
        Day22{}.part2("Player 1:
43
19

Player 2:
2
29
14");
    }
}
