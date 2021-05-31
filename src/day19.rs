use crate::day::Day;
use crate::day19::Rule::*;
use std::borrow::{BorrowMut, Borrow};
use std::collections::{HashSet, HashMap};

pub struct Day19 {}

#[derive(Clone, Debug)]
enum Rule {
    Literal(String),
    Reference(usize),
    Concat(Vec<Box<Rule>>),
    Or(Vec<Box<Rule>>)
}

fn parse_input(input: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
    let mut split = input.split("\n\n");
    if split.clone().count() == 1 {
        split = input.split("\r\n\r\n")
    }

    let raw_rules = split.next().unwrap();
    let mut rules = HashMap::new();
    for rule in raw_rules.lines() {
        println!("{:?}", rule);
        let mut num_separator = rule.split(":");
        let rule_num = num_separator.next().unwrap().parse().unwrap();
        let rule_body = num_separator.next().unwrap().trim();
        if rule_body.starts_with("\"") {
            rules.insert(rule_num, Literal(rule_body[1..rule_body.len() - 1].to_string()));
            continue;
        }

        let mut raw_ors = rule_body.split("|");
        let mut or_clauses = Vec::new();
        for raw_or in raw_ors {
            or_clauses.push(Concat(
                raw_or.trim()
                    .split(" ")
                    .map(|n| Reference(n.parse().unwrap()))
                    .map(|r| Box::new(r))
                    .collect()
            ));
        }
        if or_clauses.len() > 1 {
            rules.insert(rule_num, Or(or_clauses.into_iter().map(|r| Box::new(r)).collect()));
        } else {
            rules.insert(rule_num, or_clauses.into_iter().next().unwrap());
        }
    }

    let raw_messages = split.next().unwrap();

    (rules, raw_messages.lines().collect())
}

fn simplify_rules(mut rules: Vec<Rule>) -> Vec<Rule> {
    let mut changes_made = true;
    while changes_made {
        changes_made = false;

        for i in 0..rules.len() {
            let rule = &rules[i];
            match simplify_rule(rule, &rules) {
                Some(modified_rule) => {
                    changes_made = true;
                    rules[i] = modified_rule;
                }
                None => ()
            }
        }
    }

    rules
}

fn simplify_rule(rule: &Rule, rules: &Vec<Rule>) -> Option<Rule> {
    match rule {
        Reference(n) => Some(rules[*n].clone())/*match &rules[*n] {
            Literal(s) => Some(Literal(s.to_string())),
            _ => None
        }*/,
        Concat(sub) if sub.len() == 1 => match &*sub[0] {
            Literal(s) => Some(Literal(s.to_string())),
            Reference(n) => Some(Reference(*n)),
            _ => None
        }
        Concat(sub) if sub.iter().all(|r| matches!(**r, Literal(_))) => {
            let mut concat = "".to_string();
            for sub_rule in sub {
                match &**sub_rule {
                    Literal(s) => concat += s.as_str(),
                    _ => ()
                }
            }
            Some(Literal(concat.to_string()))
        }
        Concat(sub) => {
            let mut changed = false;
            let mut modified_sub = Vec::new();
            for sub_rule in sub {
                match &**sub_rule {
                    Literal(s) => modified_sub.push(Box::new(Literal(s.to_string()))),
                    Reference(n) => {
                        changed = true;
                        modified_sub.push(Box::new(rules[*n].clone()))
                    } /*match &rules[*n] {
                        Literal(s) => {
                            changed = true;
                            modified_sub.push(Box::new(Literal(s.to_string())))
                        },
                        _ => modified_sub.push(Box::new(Reference(*n)))
                    }*/
                    Or(_) => modified_sub.push(Box::new(*sub_rule.clone())),
                    _ => panic!("unexpected")
                }
            }
            if changed {
                Some(Concat(modified_sub))
            } else {
                None
            }
        },
        Or(sub) => {
            let mut changed = false;
            let mut modified_sub = Vec::new();
            for sub_rule in sub {
                match simplify_rule(&*sub_rule, rules) {
                    Some(modified) => {
                        changed = true;
                        modified_sub.push(Box::new(modified))
                    },
                    None => modified_sub.push(Box::new(*sub_rule.clone()))
                }
            }
            if changed {Some(Or(modified_sub))} else {None}
        }
        _ => None
    }
}

fn generate_possible_values(rule: &Rule, rules: &HashMap<usize, Rule>) -> HashSet<String> {
    match rule {
        Literal(s) => {
            let mut set = HashSet::new();
            set.insert(s.to_string());
            set
        },
        Reference(n) => generate_possible_values(&rules.get(n).unwrap(), rules),
        Concat(sub) => {
            let mut iter = sub.iter();
            let values = generate_possible_values(&*iter.next().unwrap(), rules);
            let remaining_rules:Vec<Box<Rule>> = iter.map(|b| b.clone()).collect();
            if remaining_rules.len() == 0 {
                values
            } else {
                let rest_rule = Concat(remaining_rules);
                let rest = generate_possible_values(&rest_rule, rules);

                let mut set = HashSet::new();
                for a in &values {
                    for b in &rest {
                        set.insert((a.as_str().to_owned() + b.as_str()).to_string());
                    }
                }
                set
            }
        },
        Or(sub) => {
            let mut value_iter = sub.iter().map(|r| generate_possible_values(&*r, rules));
            let first = value_iter.next().unwrap();
            value_iter.fold(first, |acc, x| acc.union(&x)
                .map(|s| s.to_string())
                .collect())
        }
    }
}

impl Day for Day19 {
    fn part1(&self, input: &str) -> String {
        let (mut rules, messages) = parse_input(input);
        let possible = generate_possible_values(&rules.get(&0usize).unwrap(), &rules);

        let valid = messages.iter()
            .map(|m| possible.contains(&m.to_string()))
            .filter(|b| *b == true)
            .count();

        valid.to_string()
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
        assert_eq!(Day19{}.part1("0: 8 1 5
1: 2 3 | 3 2
2: 8 8 | 5 5
3: 8 5 | 5 8
8: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"), "2");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day19{}.part2(""), "");
    }
}
