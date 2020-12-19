use crate::day::Day;
use std::collections::{HashSet, HashMap};

pub struct Day16 {}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    ranges: Vec<(i32, i32)>
}

impl Field {
    fn valid(&self, value:i32) -> bool {
        self.ranges.iter().any(|(low, high)| *low <= value && value <= *high)
    }
}

#[derive(Debug)]
struct Ticket {
    field_values: Vec<i32>
}

impl Ticket {
    fn from_line(line:&str) -> Ticket {
        Ticket {field_values: line.split(",").map(|n| n.parse().unwrap()).collect()}
    }

    fn error_rate(&self, field_rules:&Vec<Field>) -> i32 {
        self.field_values.iter().filter(|v| !field_rules.iter().any(|f| f.valid(**v))).sum()
    }

    fn is_valid(&self, field_rules:&Vec<Field>) -> bool {
        self.field_values.iter().filter(|v| !field_rules.iter().any(|f| f.valid(**v))).count() == 0
    }
}

impl Day for Day16 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut fields = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let name = line.split(":").next().unwrap().to_string();
            let ranges = line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" or ")
                .map(|r| r.split("-"))
                .map(|mut s| (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap()))
                .collect();
            fields.push(Field{name, ranges});
        }
        lines.next().unwrap(); // your ticket header

        let _my_ticket = Ticket::from_line(lines.next().unwrap());

        lines.next().unwrap(); // blank
        lines.next().unwrap(); // nearby tickets header

        let mut nearby_tickets = Vec::new();
        while let Some(line) = lines.next() {
            nearby_tickets.push(Ticket::from_line(line));
        }

        nearby_tickets.iter().map(|t| t.error_rate(&fields)).sum::<i32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut fields = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let name = line.split(":").next().unwrap().to_string();
            let ranges = line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" or ")
                .map(|r| r.split("-"))
                .map(|mut s| (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap()))
                .collect();
            fields.push(Field{name, ranges});
        }
        lines.next().unwrap(); // your ticket header

        let my_ticket = Ticket::from_line(lines.next().unwrap());

        lines.next().unwrap(); // blank
        lines.next().unwrap(); // nearby tickets header

        let mut nearby_tickets = Vec::new();
        while let Some(line) = lines.next() {
            nearby_tickets.push(Ticket::from_line(line));
        }
        nearby_tickets.retain(|t| t.is_valid(&fields));

        let mut field_possibilities:Vec<HashSet<&Field>> = Vec::new();
        my_ticket.field_values.iter()
            .for_each(|v| field_possibilities
                .push(fields.iter()
                          .filter(|f| f.valid(*v)).collect()));

        for ticket in nearby_tickets {
            ticket.field_values.iter()
                .enumerate()
                .for_each(|(i,v)| field_possibilities[i].retain(|f| f.valid(*v)));
        }
        log::debug!("{:?}", field_possibilities.iter().map(|f| f.len()).collect::<Vec<usize>>());

        let mut assignments = HashMap::new();

        // assign locations with only one field possibility
        while field_possibilities.iter().any(|f| f.len() == 1) {
            for i in 0..field_possibilities.len() {
                if field_possibilities[i].len() == 1 {
                    let field = *field_possibilities[i].iter().next().unwrap();
                    assignments.insert(field, i);
                    //changes_made = true;
                    for j in 0..field_possibilities.len() {
                        field_possibilities[j].remove(field);
                    }
                }
            }
        }

        log::debug!("{:?}", field_possibilities);
        log::debug!("{:?}", field_possibilities.iter().map(|f| f.len()).collect::<Vec<usize>>());
        log::debug!("{:?}", assignments);

        fields.iter()
            .filter(|f| f.name.starts_with("departure"))
            .map(|f| assignments.get(f).unwrap())
            .map(|p| my_ticket.field_values[*p] as i64)
            .product::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day16{}.part1("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"), "71");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day16{}.part2("class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"), "1");
    }
}
