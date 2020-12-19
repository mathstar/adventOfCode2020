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
        let buses = lines
            .next()
            .unwrap()
            .split(",")
            .map(|b| b.parse())
            .enumerate()
            .filter(|(_,b)| b.is_ok())
            .map(|(i,b)| (i,b.unwrap()))
            .collect::<Vec<(usize,i64)>>();

        let mut current_bus = 0;
        let mut start = 0;
        while current_bus < buses.len() {
            let target_remainder = buses[current_bus].0 as i64;
            let frequency = buses[current_bus].1;
            if (start + target_remainder) % frequency == 0 {
                current_bus += 1;
            } else {
                let mut step = (frequency - (start % frequency)) - target_remainder;
                while step < 0 {step += frequency;}
                start = start + step;
                current_bus = 0;
            }
        }

        start.to_string()
    }
}

// fn find_align_time(buses:&Vec<(usize, u64)>, start:u64) -> u64 {
//     if buses.len() == 1 {
//         let target_remainder = buses[0].0 as u64;
//         let frequency = buses[0].1;
//         start + (target_remainder - start % frequency)
//     } else {
//         let target_remainder = buses[buses.len() - 1].0 as u64;
//         let frequency = buses[buses.len() - 1].1;
//
//     }
//     0
// }

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
