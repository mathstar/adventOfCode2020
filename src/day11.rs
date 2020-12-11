use crate::day::Day;
use crate::day11::Cell::{Seat, Empty};
use std::fmt::Debug;

pub struct Day11 {}

#[derive(PartialEq, Debug)]
enum Cell {
    Seat(bool),
    Empty
}

fn num_adjacent_occupied_seats(map: &Vec<Vec<Cell>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            } else {
                match map.get((x as i32 + i) as usize) {
                    Some(row) => {
                        match row.get((y as i32 + j) as usize) {
                            Some(Seat(true)) => count += 1,
                            Some(Seat(false)) => (),
                            Some(Empty) => (),
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
    }
    count
}

fn num_adjacent_occupied_seats_farsighted(map: &Vec<Vec<Cell>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            } else {
                match map.get((x as i32 + i) as usize) {
                    Some(row) => {
                        match row.get((y as i32 + j) as usize) {
                            Some(Seat(true)) => count += 1,
                            Some(Seat(false)) => (),
                            Some(Empty) => {
                                for k in 2.. {
                                    match map.get((x as i32 + i * k) as usize) {
                                        Some(row) => {
                                            match row.get((y as i32 + j * k) as usize) {
                                                Some(Seat(true)) => {
                                                    count += 1;
                                                    break;
                                                },
                                                Some(Seat(false)) => break,
                                                Some(Empty) => (),
                                                None => break
                                            }
                                        }
                                        None => break
                                    }
                                }
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
    }
    count
}

impl Cell {
    fn step(&self, map: &Vec<Vec<Cell>>, x: usize, y: usize) -> Cell {
        match self {
            Seat(b ) => {
                if *b {
                    if num_adjacent_occupied_seats(&map, x, y) >= 4 {
                        Seat(false)
                    } else {
                        Seat(true)
                    }
                } else {
                    if num_adjacent_occupied_seats(&map, x, y) == 0 {
                        Seat(true)
                    } else {
                        Seat(false)
                    }
                }
            },
            Empty => Empty
        }
    }

    fn step_farsighted(&self, map: &Vec<Vec<Cell>>, x: usize, y: usize) -> Cell {
        match self {
            Seat(b ) => {
                if *b {
                    if num_adjacent_occupied_seats_farsighted(&map, x, y) >= 5 {
                        Seat(false)
                    } else {
                        Seat(true)
                    }
                } else {
                    if num_adjacent_occupied_seats_farsighted(&map, x, y) == 0 {
                        Seat(true)
                    } else {
                        Seat(false)
                    }
                }
            },
            Empty => Empty
        }
    }
}

fn pretty_print(map: &Vec<Vec<Cell>>) -> String {
    map.iter().map(|row| row.iter().map(|cell| match cell {
        Seat(true) => "#".to_string(),
        Seat(false) => "L".to_string(),
        Empty => ".".to_string()
    }).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n")
}

impl Day for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut map = input.lines().map(|line| line.chars().map(|c| match c {
            'L' => Seat(false),
            '.' => Empty,
            _ => panic!("invalid character")
        }).collect::<Vec<Cell>>()).collect::<Vec<Vec<Cell>>>();
        log::debug!("\n{}", pretty_print(&map));

        loop {
            let mut step = Vec::new();
            for i in 0..map.len() {
                let mut row = Vec::new();
                for j in 0..map[0].len() {
                    row.push(map[i][j].step(&map, i, j));
                }
                step.push(row);
            }
            log::debug!("\n{}", pretty_print(&step));

            if step == map {
                return map.iter().map(|row| row.iter().filter(|cell| **cell == Seat(true)).count()).sum::<usize>().to_string();
            } else {
                map = step;
            }
        }
    }

    fn part2(&self, input: &str) -> String {
        let mut map = input.lines().map(|line| line.chars().map(|c| match c {
            'L' => Seat(false),
            '.' => Empty,
            _ => panic!("invalid character")
        }).collect::<Vec<Cell>>()).collect::<Vec<Vec<Cell>>>();
        log::debug!("\n{}", pretty_print(&map));

        loop {
            let mut step = Vec::new();
            for i in 0..map.len() {
                let mut row = Vec::new();
                for j in 0..map[0].len() {
                    row.push(map[i][j].step_farsighted(&map, i, j));
                }
                step.push(row);
            }
            log::debug!("\n{}", pretty_print(&step));

            if step == map {
                return map.iter().map(|row| row.iter().filter(|cell| **cell == Seat(true)).count()).sum::<usize>().to_string();
            } else {
                map = step;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        SimpleLogger::new().init().unwrap();
        assert_eq!(Day11{}.part1("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"), "37")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day11{}.part2("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"), "26")
    }
}
