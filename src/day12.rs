use crate::day::Day;
use crate::day12::Instruction::*;

pub struct Day12 {}

enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32)
}

fn parse_instruction(s:&str) -> Instruction {
    let mut s = s.chars();
    match s.next().unwrap() {
        'N' => N(s.as_str().parse().unwrap()),
        'S' => S(s.as_str().parse().unwrap()),
        'E' => E(s.as_str().parse().unwrap()),
        'W' => W(s.as_str().parse().unwrap()),
        'L' => L(s.as_str().parse().unwrap()),
        'R' => R(s.as_str().parse().unwrap()),
        'F' => F(s.as_str().parse().unwrap()),
        i => panic!(format!("unrecognized instruction {}", i))
    }
}

struct Position {
    north:i32,
    east:i32,
    heading:i32
}

impl Position {
    fn new() -> Position {
        Position{north:0,east:0,heading:90}
    }

    fn apply(&mut self, i:Instruction) {
        match i {
            N(n) => self.north += n,
            S(n) => self.north -= n,
            E(n) => self.east += n,
            W(n) => self.east -= n,
            L(n) => self.heading = (self.heading - n + 360) % 360,
            R(n) => self.heading = (self.heading + n) % 360,
            F(n) => match self.heading {
                0 => self.north += n,
                90 => self.east += n,
                180 => self.north -= n,
                270 => self.east -= n,
                h => panic!(format!("heading not aligned to grid {}", h))
            }
        }
    }

    fn distance_from_origin(&self) -> i32{
        self.north.abs() + self.east.abs()
    }
}

struct PositionWithWaypoint {
    north:i32,
    east:i32,
    waypoint_north:i32,
    waypoint_east:i32,
}

impl PositionWithWaypoint {
    fn new() -> PositionWithWaypoint {
        PositionWithWaypoint{north:0,east:0,waypoint_north:1,waypoint_east:10}
    }

    fn apply(&mut self, i:Instruction) {
        match i {
            N(n) => self.waypoint_north += n,
            S(n) => self.waypoint_north -= n,
            E(n) => self.waypoint_east += n,
            W(n) => self.waypoint_east -= n,
            L(n) => match n {
                0 => (),
                270 => {
                    let new_east = self.waypoint_north;
                    let new_north = -1 * self.waypoint_east;
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north
                },
                180 => {
                    let new_east = -1 * self.waypoint_east;
                    let new_north = -1 * self.waypoint_north;
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north
                },
                90 => {
                    let new_east = -1 * self.waypoint_north;
                    let new_north = self.waypoint_east;
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north
                },
                n => panic!(format!("Complicated rotate {}", n))
            },
            R(n) => match n {
                0 => (),
                90 => {
                    let new_east = self.waypoint_north;
                    let new_north = -1 * self.waypoint_east;
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north
                },
                180 => {
                    let new_east = -1 * self.waypoint_east;
                    let new_north = -1 * self.waypoint_north;
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north
                },
                270 => {
                    let new_east = -1 * self.waypoint_north;
                    let new_north = self.waypoint_east;
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north
                },
                n => panic!(format!("Complicated rotate {}", n))
            },
            F(n) => {
                self.north += self.waypoint_north * n;
                self.east += self.waypoint_east * n;
            }
        }
    }

    fn distance_from_origin(&self) -> i32{
        self.north.abs() + self.east.abs()
    }
}

impl Day for Day12 {
    fn part1(&self, input: &str) -> String {
        let mut position = Position::new();
        input.lines().map(|line| parse_instruction(line)).for_each(|i| position.apply(i));
        position.distance_from_origin().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut position = PositionWithWaypoint::new();
        input.lines().map(|line| parse_instruction(line)).for_each(|i| position.apply(i));
        position.distance_from_origin().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        SimpleLogger::new().init().unwrap();
        assert_eq!(Day12{}.part1("F10
N3
F7
R90
F11"), "25")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day12{}.part2("F10
N3
F7
R90
F11"), "286")
    }
}
