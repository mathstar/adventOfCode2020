use crate::day::Day;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::{Regex, Captures};

pub struct Day7 {}

#[derive(Debug)]
struct BagType {
    color: String,
    contents: HashMap<String, u32>,
}

fn parse_rules(input: &str) -> HashMap<String, BagType> {
    let regex = Regex::new(r"([a-z ]+) bags? contain ((no other bags)|((?P<bag1num>[0-9]+) (?P<bag1color>[a-z ]+) bags?(?P<other_bags>(, ([0-9]+) ([a-z ]+) bags?)*)))\.")
        .expect("Failed to compile regex");
    input.lines().map(|line| regex.captures(line).expect(format!("Failed to parse rule: {}", line).as_str()))
        .map(|captures| BagType {
            color: String::from(captures.get(1).expect("Missing bag color").as_str()),
            contents: parse_bag_contents(&captures)
        })
        .map(|bag_type| (String::from(bag_type.color.as_str()), bag_type))
        .collect()
}

fn parse_bag_contents(captures: &Captures) -> HashMap<String, u32> {
    let regex = Regex::new(r"([0-9]+) ([a-z ]+) bags?").expect("Failed to compile regex");
    match captures.name("bag1num") {
        Some(num) => {
            let mut contents = HashMap::new();
            contents.insert(String::from(captures.name("bag1color").expect("Invalid rule").as_str()), num.as_str().parse().unwrap());
            match captures.name("other_bags") {
                Some(other_bags) => {
                    for bag in regex.find_iter(other_bags.as_str()) {
                        let split = bag.as_str().split_whitespace().collect::<Vec<&str>>();
                        contents.insert(String::from(split[1..split.len()-1].join(" ")), split[0].parse().unwrap());
                    }
                    contents
                },
                None => contents
            }
        },
        None => HashMap::new()
    }
}

fn reverse_rules(rules: &HashMap<String, BagType>) -> HashMap<String, Vec<String>> {
    let mut reverse = HashMap::new();
    rules.iter().for_each(|(c, b)| b.contents.keys()
        .for_each(|k| reverse.entry(String::from(k))
            .or_insert(Vec::new())
            .push(String::from(c))));
    reverse
}

fn reachable(target: &str, reverse: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut to_check = VecDeque::new();
    let mut checked = HashSet::new();
    reverse.get(target).unwrap().iter().for_each(|a| to_check.push_front(a.to_string()));
    while let Some(c) = to_check.pop_front() {
        match checked.insert(c.to_string()) {
            true => if let Some(list) = reverse.get(c.as_str()) {
                list.iter().for_each(|a| to_check.push_front(a.to_string()))
            }
            false => ()
        }
    }

    checked
}

impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        let rules = parse_rules(input);
        let reverse = reverse_rules(&rules);

        rules.iter().for_each(|v| log::debug!("{:?}", v));
        reverse.iter().for_each(|v| log::debug!("{:?}", v));
        reachable("shiny gold", &reverse).len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rules = parse_rules(input);

        let mut to_count = VecDeque::new();
        to_count.push_front((1, rules.get("shiny gold").unwrap()));

        let mut count = 0;
        while let Some((i, bag)) = to_count.pop_front() {
            count += i;
            bag.contents.iter().for_each(|(inner_bag, j)| to_count.push_back((j * i, rules.get(inner_bag).unwrap())));
        }
        count -= 1;

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day7 {}.part1("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."), "4")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day7 {}.part2("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."), "32")
    }
}
