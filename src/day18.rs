use crate::day::Day;
use std::collections::VecDeque;
use crate::day18::CalculationToken::*;
use crate::day18::CalculationToken::CloseParen;

pub struct Day18 {}

#[derive(Copy, Clone, Debug)]
enum CalculationToken {
    OpenParen,
    CloseParen,
    Plus,
    Times,
    Num(i64)
}

enum Op {
    Add,
    Multiply
}

impl Op {
    fn apply(&self, acc:&mut i64, val:i64) {
        match self {
            Op::Add => *acc += val,
            Op::Multiply => *acc *= val
        }
    }
}

fn tokenize(input:&str) -> VecDeque<CalculationToken> {
    let mut tokens = VecDeque::new();
    for c in input.chars() {
        match c {
            '(' => tokens.push_back(OpenParen),
            ')' => tokens.push_back(CloseParen),
            '+' => tokens.push_back(Plus),
            '*' => tokens.push_back(Times),
            c if c.is_numeric() => tokens.push_back(Num(c .to_digit(10).unwrap() as i64)),
            _ => ()
        }
    }
    tokens
}

fn calculate(tokens:&mut VecDeque<CalculationToken>) -> i64 {
    let mut acc= vec![0];
    let mut op_stack = vec![Op::Add];

    while let Some(t) = tokens.pop_front() {
        match t {
            OpenParen => {
                acc.push(0);
                op_stack.push(Op::Add)
            },
            CloseParen => {
                let result = acc.pop().unwrap();
                op_stack.pop().unwrap().apply(&mut acc.last_mut().unwrap(), result);
            },
            Plus => op_stack.push(Op::Add),
            Times => op_stack.push(Op::Multiply),
            Num(n) => op_stack.pop().unwrap().apply(&mut acc.last_mut().unwrap(), n)
        }
    }
    acc.pop().unwrap()
}

fn calculate_prioritize_addition(tokens:&mut VecDeque<CalculationToken>) -> i64 {
    let mut without_parens = VecDeque::new();
    let mut sub = None;
    let mut paren_count = 0;
    while let Some(t) = tokens.pop_front() {
        match t {
            OpenParen if matches!(sub, None) => {
                paren_count += 1;
                sub = Some(VecDeque::new());
            }
            CloseParen if paren_count == 1 => {
                paren_count -= 1;
                without_parens.push_back(Num(calculate_prioritize_addition(&mut sub.unwrap())));
                sub = None;
            }
            t => {
                if let OpenParen = t {
                    paren_count += 1;
                } else if let CloseParen = t {
                    paren_count -= 1;
                }
                match sub {
                    Some(ref mut v) => v.push_back(t),
                    None => without_parens.push_back(t)
                }
            }
        }
    }

    let mut without_add = VecDeque::new();
    let mut op = None;
    let mut num = 0;
    while let Some(t) = without_parens.pop_front() {
        match t {
            Plus | Times => {op = Some(t)},
            Num(n) => match op {
                None => num = n,
                Some(Plus) => num = num + n,
                Some(Times) => {
                    without_add.push_back(Num(num));
                    without_add.push_back(Times);
                    num = n;
                }
                _ => panic!("Unexpected parens")
            },
            _ => panic!("Unexpected parens")
        }
    }
    without_add.push_back(Num(num));

    let mut num = 1;
    while let Some(t) = without_add.pop_front() {
        match t {
            Times => (),
            Num(n) => num *= n,
            _ => panic!("Unexpected add")
        }
    }

    num
}

impl Day for Day18 {
    fn part1(&self, input: &str) -> String {
        input.lines().map(|l| calculate(&mut tokenize(l))).sum::<i64>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        input.lines().map(|l| calculate_prioritize_addition(&mut tokenize(l))).sum::<i64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day18{}.part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), "13632");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day18{}.part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), "23340");
    }
}
