use crate::day::Day;
use std::collections::VecDeque;
use crate::day18::CalculationToken::*;
use crate::day18::Process::Token;

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

enum Tree {
    Node(Box<Tree>, Box<Tree>, Op),
    Leaf(i64)
}

enum Process {
    Tree(Box<Tree>),
    Token(CalculationToken)
}

// fn priority_addition_tree(tokens:&mut VecDeque<CalculationToken>) ->  {
//     let mut process = Vec::new();
//     while let Some(t) = tokens.pop_front() {
//         process.push(match (t) {
//             Num(n) => Process::Tree(Box::new(Tree::Leaf(n))),
//             t => Process::Token(t)
//         })
//     }
//
//     while process.len() > 1 {
//         let mut next_step = Vec::new();
//         for i in 0..process.len() {
//             match process[i] {
//                 Token(Plus) => if let Some(Process::Tree(_)) = next_step.last() {
//                     if let Process::Tree(_)
//                 }
//             }
//         }
//     }
// }

fn calculate_prioritize_addition(tokens:VecDeque<CalculationToken>) -> i64 {
    let mut mutated = Some(Vec::from(tokens));
    let mut post = Vec::new();
    while let Some(tokens) = mutated {
        mutated = None;
        for i in 0..tokens.len() {
            if let OpenParen = tokens[i] {
                let mut inner_parens = 0;
                let mut closed_paren_index = 0;
                for j in i+1..tokens.len() {
                    if let OpenParen = tokens[j] {
                        inner_parens += 1;
                    } else if let CloseParen = tokens[j] {
                        if inner_parens > 0 {
                            inner_parens -= 1;
                        } else {
                            closed_paren_index = j;
                            break;
                        }
                    }
                }
                assert_ne!(closed_paren_index, 0);

                let mut m = Vec::new();
                if i > 0 {
                    for i in 0..i - 1 { m.push(tokens[i]) }
                }
                let mut inside = VecDeque::new();
                for i in i+1..closed_paren_index {inside.push_back(tokens[i])}
                println!("in: {:?}", tokens);
                println!("inner: {:?}", inside);
                m.push(Num(calculate_prioritize_addition(inside)));
                for i in closed_paren_index+1..tokens.len() {m.push(tokens[i])}
                println!("out: {:?}", m);
                mutated = Some(m);
            }
        }
        if let None = mutated {
            post = tokens;
        }
    }

    let mut mutated = Some(Vec::from(post));
    while let Some(tokens) = mutated {
        //println!("{:?}", tokens);
        mutated = None;
        for i in 0..tokens.len() {
            if let Plus = tokens[i] {
                if let Num(a) = tokens[i - 1] {
                    if let Num(b) = tokens[i + 1] {
                        let mut m = Vec::new();
                        for i in 0..i - 1 { m.push(tokens[i]) }
                        m.push(Num(a + b));
                        for i in i + 2..tokens.len() { m.push(tokens[i]) }
                        mutated = Some(m);
                    }
                }
            }
        }
        if let None = mutated {
            for i in 0..tokens.len() {
                if let Times = tokens[i] {
                    if let Num(a) = tokens[i - 1] {
                        if let Num(b) = tokens[i + 1] {
                            let mut m = Vec::new();
                            for i in 0..i - 1 { m.push(tokens[i]) }
                            m.push(Num(a * b));
                            for i in i + 2..tokens.len() { m.push(tokens[i]) }
                            mutated = Some(m);
                        }
                    }
                }
            }

            if let None = mutated {
                for i in 0..tokens.len() {
                    if let OpenParen = tokens[i] {
                        if let Num(a) = tokens[i+1] {
                            if let CloseParen = tokens[i+2] {
                                let mut m = Vec::new();
                                for i in 0..i {m.push(tokens[i])}
                                m.push(Num(a));
                                for i in i+3..tokens.len() {m.push(tokens[i])}
                                mutated = Some(m);
                            }
                        }
                    }
                }
                if let None = mutated {
                    if tokens.len() == 1 {
                        if let Num(n) = tokens[0] {
                            return n;
                        }
                    }
                    panic!("Calculation failed");
                }
            }
        }
    }
    0
}

impl Day for Day18 {
    fn part1(&self, input: &str) -> String {
        input.lines().map(|l| calculate(&mut tokenize(l))).sum::<i64>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        input.lines().map(|l| calculate_prioritize_addition(tokenize(l))).sum::<i64>().to_string()
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
