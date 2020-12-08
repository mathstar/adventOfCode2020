use crate::day::Day;
use crate::day8::Instruction::*;
use std::collections::HashSet;

pub struct Day8 {}

#[derive(Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

fn apply_instruction(instruction: &Instruction, pc: &mut i32, acc: &mut i32) {
    match instruction {
        Acc(i) => {
            *acc += i;
            *pc += 1;
        },
        Jmp(i) => {
            *pc += i;
        }
        Nop(_) => {
            *pc += 1;
        }
    }
}

fn parse_instruction(input: &str) -> Instruction {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    match split[0] {
        "acc" => Acc(split[1].parse().unwrap()),
        "jmp" => Jmp(split[1].parse().unwrap()),
        "nop" => Nop(split[1].parse().unwrap()),
        _ => panic!("Invalid instruction")
    }
}

fn check_termination(instructions: &Vec<Instruction>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut pc = 0;
    let mut acc = 0;
    while (pc as usize) < instructions.len() {
        let instruction = &(instructions[pc as usize]);
        if visited.insert(pc) {
            apply_instruction(&instruction, &mut pc, &mut acc);
        } else {
            return None;
        }
    }
    return Some(acc);
}

impl Day for Day8 {
    fn part1(&self, input: &str) -> String {
        let instructions = input.lines().map(parse_instruction).collect::<Vec<Instruction>>();

        let mut visited = HashSet::new();
        let mut pc = 0;
        let mut acc = 0;
        loop {
            let instruction = &(instructions[pc as usize]);
            if visited.insert(pc) {
                apply_instruction(&instruction, &mut pc, &mut acc);
            } else {
                return acc.to_string();
            }
        }
    }

    fn part2(&self, input: &str) -> String {
        let instructions = input.lines().map(parse_instruction).collect::<Vec<Instruction>>();

        for i in 0..instructions.len() {
            let mut mutated = instructions.iter().map(|i| i.clone()).collect::<Vec<Instruction>>();
            match mutated[i] {
                Acc(_) => continue,
                Jmp(n) => mutated[i] = Nop(n),
                Nop(n) => mutated[i] = Jmp(n)
            }
            match check_termination(&mutated) {
                Some(i) => return i.to_string(),
                None => ()
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day8{}.part1("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"), "5")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day8{}.part2("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"), "8")
    }
}
