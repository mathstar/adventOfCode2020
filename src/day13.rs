use crate::day::Day;

pub struct Day13 {}

impl Day for Day13 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let start = lines.next().unwrap().parse::<u32>().unwrap();
        let buses = lines.next().unwrap().split(",").map(|b| b.parse()).collect::<Vec<Result<u32,_>>>();
        let mut least_wait_bus = None;
        let mut least_wait = None;
        for i in 0..buses.len() {
            if let Ok(b) = buses[i] {
                let wait = b - (start % b);
                if least_wait_bus == None || wait < least_wait.unwrap() {
                    least_wait = Some(wait);
                    least_wait_bus = Some(b);
                }
            }
        }
        (least_wait.unwrap() * least_wait_bus.unwrap()).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        lines.next().unwrap();
        let mut buses = lines
            .next()
            .unwrap()
            .split(",")
            .map(|b| b.parse())
            .enumerate()
            .filter(|(_,b)| b.is_ok())
            .map(|(i,b)| (i as i64,b.unwrap()))
            .collect::<Vec<(i64,i64)>>();

        // sieve in order of highest bus number (modulus)
        buses.sort_by(|(_,a), (_,b)| b.cmp(a));

        // use sieve-based approach
        let mut time = buses[0].1 - buses[0].0;
        let mut aggregate_modulus = buses[0].1;
        for &(inc, bus) in buses.iter().skip(1) {
            while (time + inc) % bus != 0 {
                time += aggregate_modulus;
            }
            aggregate_modulus *= bus;
        }

        time.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day13{}.part1("939
7,13,x,x,59,x,31,19"), "295");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day13{}.part2("939
7,13,x,x,59,x,31,19"), "1068781");
    }

    #[test]
    fn part2_test2() {
        assert_eq!(Day13{}.part2("939
17,x,13,19"), "3417");
    }

    #[test]
    fn part2_test3() {
        assert_eq!(Day13{}.part2("939
1789,37,47,1889"), "1202161486");
    }
}
