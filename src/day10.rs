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
        let mut combinations = 1u64;
        let mut since_diff_3 = 0;
        for i in 1..adapters.len() {
            let diff = adapters[i] - adapters[i-1];
            if diff == 3 {
                combinations *= match since_diff_3 {
                    4 => 7, // (3 choose 2) + (3 choose 1) + (3 choose 0)
                    3 => 4,
                    2 => 2,
                    1 => 1,
                    0 => 1,
                    n => panic!("unhandled case {}", n)
                };
                since_diff_3 = 0;
            } else {
                since_diff_3 += 1;
            }
        }
        combinations.to_string()
    }
}

#[allow(unused_must_use)]
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
