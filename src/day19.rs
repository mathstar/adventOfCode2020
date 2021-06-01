use crate::day::Day;
use crate::day19::Rule::*;
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
        let mut num_separator = rule.split(":");
        let rule_num = num_separator.next().unwrap().parse().unwrap();
        let rule_body = num_separator.next().unwrap().trim();
        if rule_body.starts_with("\"") {
            rules.insert(rule_num, Literal(rule_body[1..rule_body.len() - 1].to_string()));
            continue;
        }

        let raw_ors = rule_body.split("|");
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
        let (rules, messages) = parse_input(input);
        let possible = generate_possible_values(&rules.get(&0usize).unwrap(), &rules);

        let valid = messages.iter()
            .map(|m| possible.contains(&m.to_string()))
            .filter(|b| *b == true)
            .count();

        valid.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (rules, messages) = parse_input(input);
        let possible_42 = generate_possible_values(rules.get(&42).unwrap(), &rules);
        let possible_31 = generate_possible_values(rules.get(&31).unwrap(), &rules);

        // checking assumption that all possible values for rule 42 and rule 31 are the same length
        assert_eq!(possible_42.iter().map(|s| s.len()).min().unwrap(), possible_42.iter().map(|s| s.len()).max().unwrap());
        assert_eq!(possible_31.iter().map(|s| s.len()).min().unwrap(), possible_31.iter().map(|s| s.len()).max().unwrap());
        assert_eq!(possible_31.iter().map(|s| s.len()).min().unwrap(), possible_42.iter().map(|s| s.len()).max().unwrap());

        let chunk_size = possible_42.iter().map(|s| s.len()).min().unwrap();

        let mut valid = 0;
        for message in messages.iter() {
            if message.len() % chunk_size != 0 {
                continue;
            }

            let chunk_count = message.len() / chunk_size;
            let mut leading_42s = 0;
            for i in 0..chunk_count {
                if possible_42.contains(&message[i*chunk_size..(i+1)*chunk_size]) {
                    leading_42s += 1;
                } else {
                    break;
                }
            }

            let mut trailing_31s = 0;
            for i in 0..chunk_count {
                if possible_31.contains(&message[(chunk_count - i - 1)*chunk_size..(chunk_count - i)*chunk_size]) {
                    trailing_31s += 1;
                } else {
                    break;
                }
            }

            for prefix in 1..chunk_count {
                let suffix = chunk_count - prefix;
                if (suffix % 2 == 0) && (leading_42s >= prefix + suffix / 2) && (trailing_31s >= suffix / 2) {
                    valid += 1;
                    break;
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
        assert_eq!(Day19{}.part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), "12");
    }
}
