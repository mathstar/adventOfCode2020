use crate::day::Day;

pub struct Day10 {}

impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        let mut adapters = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<u32>>();
        adapters.push(0);
        adapters.sort();
        let mut diff1 = 0;
        let mut diff3 = 1;
        for i in 1..adapters.len() {
            let diff = adapters[i] - adapters[i-1];
            if diff == 1 {diff1 += 1;}
            if diff == 3 {diff3 += 1;}
        }

        (diff1 * diff3).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut adapters = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<u32>>();
        adapters.push(0);
        adapters.sort();
        adapters.push(adapters[adapters.len()-1] + 3);
        log::debug!("{:?}", adapters);
        let mut skippable = 0;
        let mut consecutive_unskippable = 0;
        for i in 1..adapters.len()-1 {
            let diff = adapters[i+1] - adapters[i-1];
            if diff <= 3 {
                skippable += 1;

                if i >= 2 {
                    if adapters[i-1] - adapters[i-2] <= 3 && adapters [i] - adapters[i-2] > 3 {
                        consecutive_unskippable += 1;
                    }
                }
            }
        }
        //log::debug!("{} {} {}", skippable, consecutive, twice_consecutive);
        (2u64.pow(skippable) - consecutive_unskippable).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        assert_eq!(Day10{}.part1("16
10
15
5
1
11
7
19
6
12
4"), "35")
    }

    #[test]
    fn part2_test1() {
        SimpleLogger::new().init();
        assert_eq!(Day10{}.part2("16
10
15
5
1
11
7
19
6
12
4"), "8")
    }


    #[test]
    fn part2_test2() {
        SimpleLogger::new().init();
        assert_eq!(Day10{}.part2("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"), "19208")
    }
}
