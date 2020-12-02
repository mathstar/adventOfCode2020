use crate::day::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, input: &str) -> String {
        let list = input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                if list[i] + list[j] == 2020 {
                    return (list[i] * list[j]).to_string();
                }
            }
        }
        String::new()
    }

    fn part2(&self, input: &str) -> String {
        let list = input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                for k in j + 1..list.len() {
                    if list[i] + list[j] + list[k] == 2020 {
                        return (list[i] * list[j] * list[k]).to_string();
                    }
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day1{}.part1("1721
979
366
299
675
1456"), "514579");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day1{}.part2("1721
979
366
299
675
1456"), "241861950");
    }
}
