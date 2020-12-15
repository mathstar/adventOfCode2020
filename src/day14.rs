use crate::day::Day;
use std::collections::{HashMap, HashSet};
use std::ops::{BitOr, BitAnd, BitXor};

pub struct Day14 {}

fn parse_mask(mask_line:&str) -> Vec<Option<u8>>{
    mask_line
        .split(" = ")
        .nth(1)
        .unwrap()
        .chars()
        .map(|c| match c {
            '0' => Some(0),
            '1' => Some(1),
            _ => None
        })
        .collect()
}

fn update_mask(mask_line:&str, on_mask:&mut u64, off_mask:&mut u64) {
    let mask = parse_mask(mask_line);

    *on_mask = mask.iter().fold(0u64, |a,b| (a << 1) + match b {
        Some(1) => 1,
        _ => 0
    });

    *off_mask = mask.iter().fold(0u64, |a,b| (a << 1) + match b {
        Some(0) => 0,
        _ => 1
    });
}

fn apply_address_mask(mask:&Vec<Option<u8>>, address:u64) -> HashSet<u64> {
    let on_mask = mask.iter().fold(0u64, |a,b| (a << 1) + match b {
        Some(1) => 1,
        _ => 0
    });
    let address = address.bitor(on_mask);
    let mut addresses = HashSet::new();
    addresses.insert(address);

    for i in 0..mask.len() {
        if let None = mask[i] {
            let on_mask = 1 << (mask.len() - i - 1);
            let off_mask = u64::MAX.bitxor(on_mask);

            let mut new_addresses = HashSet::new();
            addresses.iter().for_each(|a| {
                new_addresses.insert(a.bitor(on_mask));
                new_addresses.insert(a.bitand(off_mask));
            });
            addresses = new_addresses;
        }
    }

    addresses
}

impl Day for Day14 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut on_mask = 0;
        let mut off_mask = 0;

        let mut values = HashMap::new();
        for line in lines {
            if line.starts_with("mask") {
                update_mask(line, &mut on_mask, &mut off_mask);
            } else {
                let address: u32 = line.split("[").nth(1).unwrap().split("]").next().unwrap().parse().unwrap();
                let value: u64 = line.split(" = ").nth(1).unwrap().parse().unwrap();
                values.insert(address, value.bitor(on_mask).bitand(off_mask));
            }
        }

        values.values().sum::<u64>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut mask = Vec::new();

        let mut values = HashMap::new();
        for line in lines {
            if line.starts_with("mask") {
                mask = parse_mask(line);
            } else {
                let address: u64 = line.split("[").nth(1).unwrap().split("]").next().unwrap().parse().unwrap();
                let addresses = apply_address_mask(&mask, address);
                let value: u64 = line.split(" = ").nth(1).unwrap().parse().unwrap();
                for address in addresses {
                    values.insert(address, value);
                }
            }
        }

        values.values().sum::<u64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        assert_eq!(Day14{}.part1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"), "165");
    }

    #[test]
    fn part2_test1() {
        SimpleLogger::new().init().unwrap();
        assert_eq!(Day14{}.part2("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"), "208");
    }
}
