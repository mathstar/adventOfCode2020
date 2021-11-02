use crate::day::Day;

pub struct Day23 {
    pub move_count: u32
}

#[allow(dead_code)]
fn pretty_print(cups: &Vec<usize>, current: usize) -> String {
    let mut result;
    if current == 1 {
        result = "(1),".to_owned();
    } else {
        result = "1,".to_owned();
    }
    let mut cup = cups[1];
    while cup != 1 {
        if current == cup {
            result += format!("({}),", cup).as_str();
        } else {
            result += format!("{},", cup).as_str();
        }
        cup = cups[cup];
    }
    result
}

impl Day for Day23 {
    fn part1(&self, input: &str) -> String {
        let input = input.lines().next().unwrap();
        let mut input_iter = input.chars().map(|c| c as usize - '0' as usize);

        // adjacency vector - cups[i] = <cup clockwise from cup i>
        let mut cups = vec![0usize; input.len() + 1];

        // generate initial cup
        let mut current = input_iter.next().unwrap();
        let mut prev = current;
        cups[current] = current;

        // insert cups from input
        for val in input_iter {
            cups[prev] = val;
            prev = val;
            cups[val] = current;
        }

        // perform moves
        for _ in 0..self.move_count {
            // identify cups to "pick up"
            let next_1 = cups[current];
            let next_2 = cups[next_1];
            let next_3 = cups[next_2];

            // "remove" cups by pointing current's next at cup after picked up cups
            cups[current] = cups[next_3];

            // identify destination
            let mut destination = current - 1;
            if destination < 1 {
                destination = cups.len() - 1;
            }
            while destination == next_1 || destination == next_2 || destination == next_3 {
                destination -= 1;
                if destination < 1 {
                    destination = cups.len() - 1;
                }
            }

            // insert cups after destination
            let after_destination = cups[destination];
            cups[destination] = next_1;
            cups[next_3] = after_destination;

            // pick new current cup
            current = cups[current];
        }

        let mut result = "".to_owned();
        let mut next = cups[1];
        while next != 1 {
            result += next.to_string().as_str();
            next = cups[next];
        }
        result
    }

    fn part2(&self, input: &str) -> String {
        let mut input_iter = input.lines().next().unwrap().chars().map(|c| c as usize - '0' as usize);

        // adjacency vector - cups[i] = <cup clockwise from cup i>
        let mut cups = vec![0usize; 1000001];

        // generate initial cup
        let mut current = input_iter.next().unwrap();
        let mut prev = current;
        cups[current] = current;

        // insert cups from input
        let mut max = current;
        for val in input_iter {
            cups[prev] = val;
            prev = val;
            cups[val] = current;

            max = std::cmp::max(max, val);
        }

        // insert remaining cups
        for val in max+1..1000001 {
            cups[prev] = val;
            prev = val;
            cups[val] = current;
        }

        // perform moves
        for _ in 0..10000000 {
            // identify cups to "pick up"
            let next_1 = cups[current];
            let next_2 = cups[next_1];
            let next_3 = cups[next_2];

            // "remove" cups by pointing current's next at cup after picked up cups
            cups[current] = cups[next_3];

            // identify destination
            let mut destination = current - 1;
            if destination < 1 {
                destination = 1000000;
            }
            while destination == next_1 || destination == next_2 || destination == next_3 {
                destination -= 1;
                if destination < 1 {
                    destination = 1000000;
                }
            }

            // insert cups after destination
            let after_destination = cups[destination];
            cups[destination] = next_1;
            cups[next_3] = after_destination;

            // pick new current cup
            current = cups[current];
        }

        let result = cups[1];
        (result as u64 * cups[result] as u64).to_string()
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
    fn part1_test2() {
        assert_eq!(Day23{move_count: 100}.part1("389125467"), "67384529");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day23{move_count: 10000000}.part2("389125467"), "149245887792");
    }
}
