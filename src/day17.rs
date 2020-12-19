use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day17 {}

enum State {
    Active
}

struct Cube {
    values: HashMap<(i32,i32,i32), State>
}

struct HyperCube {
    values: HashMap<(i32,i32,i32,i32), State>
}

impl Cube {
    fn from_str(input:&str) -> Cube {
        Cube {
            values : input.lines()
                .enumerate()
                .flat_map(|(i,line)| line.chars().enumerate()
                    .filter(|(_,c)| matches!(c, '#'))
                    .map(move |(j,_)| (i as i32,j as i32,0)))
                .map(|p|(p,State::Active))
                .collect()
        }
    }

    fn active_cell_count(&self) -> usize {
        self.values.iter().filter(|(_,s)| matches!(s, State::Active)).count()
    }

    fn neighbors(pos:(i32,i32,i32)) -> Vec<(i32,i32,i32)> {
        let mut output = Vec::new();
        for x_diff in -1..2 {
            for y_diff in -1..2 {
                for z_diff in -1..2 {
                    if x_diff != 0 || y_diff != 0 || z_diff != 0 {
                        output.push((pos.0 + x_diff, pos.1 + y_diff, pos.2 + z_diff));
                    }
                }
            }
        }
        output
    }

    fn active_neighbors(&self, pos:(i32,i32,i32)) -> Vec<(i32,i32,i32)>{
        Self::neighbors(pos).iter().filter(|n| matches!(self.values.get(n), Some(State::Active))).map(|n| *n).collect()
    }

    fn inactive_neighbors(&self, pos:(i32,i32,i32)) -> Vec<(i32,i32,i32)>{
        Self::neighbors(pos).iter().filter(|n| matches!(self.values.get(n), None)).map(|n| *n).collect()
    }

    fn step(&mut self) {
        let mut next_state = HashMap::new();
        let mut inactive_cells_to_process = HashSet::new();
        for (pos, state) in self.values.iter() {
            match state {
                State::Active => {
                    let active_neighbors = self.active_neighbors(*pos).len();
                    if active_neighbors == 2 || active_neighbors == 3 {
                        next_state.insert(*pos, State::Active);
                    }
                    for c in self.inactive_neighbors(*pos) {
                        inactive_cells_to_process.insert(c);
                    }
                }
            }
        }

        for pos in inactive_cells_to_process {
            let active_neighbors = self.active_neighbors(pos).len();
            if active_neighbors == 3 {
                next_state.insert(pos, State::Active);
            }
        }

        self.values = next_state
    }

    #[allow(dead_code)]
    fn pretty_print(&self) -> String {
        let min_x = *self.values.iter().map(|((x,_,_),_)| x).min().unwrap();
        let max_x = *self.values.iter().map(|((x,_,_),_)| x).max().unwrap();
        let min_y = *self.values.iter().map(|((_,y,_),_)| y).min().unwrap();
        let max_y = *self.values.iter().map(|((_,y,_),_)| y).max().unwrap();
        let min_z = *self.values.iter().map(|((_,_,z),_)| z).min().unwrap();
        let max_z = *self.values.iter().map(|((_,_,z),_)| z).max().unwrap();

        let mut output = String::new();
        for z in min_z..max_z+1 {
            for y in min_y..max_y+1 {
                for x in min_x..max_x+1 {
                    output += match self.values.get(&(x,y,z)) {
                        Some(State::Active) => "#",
                        None => "."
                    }
                }
                output += "\n";
            }
            output += "\n\n";
        }
        output
    }
}

impl HyperCube {
    fn from_str(input:&str) -> HyperCube {
        HyperCube {
            values : input.lines()
                .enumerate()
                .flat_map(|(i,line)| line.chars().enumerate()
                    .filter(|(_,c)| matches!(c, '#'))
                    .map(move |(j,_)| (i as i32,j as i32,0,0)))
                .map(|p|(p,State::Active))
                .collect()
        }
    }

    fn active_cell_count(&self) -> usize {
        self.values.iter().filter(|(_,s)| matches!(s, State::Active)).count()
    }

    fn neighbors(pos:(i32,i32,i32,i32)) -> Vec<(i32,i32,i32,i32)> {
        let mut output = Vec::new();
        for x_diff in -1..2 {
            for y_diff in -1..2 {
                for z_diff in -1..2 {
                    for w_diff in -1..2 {
                        if x_diff != 0 || y_diff != 0 || z_diff != 0 || w_diff != 0{
                            output.push((pos.0 + x_diff, pos.1 + y_diff, pos.2 + z_diff, pos.3 + w_diff));
                        }
                    }
                }
            }
        }
        output
    }

    fn active_neighbors(&self, pos:(i32,i32,i32,i32)) -> Vec<(i32,i32,i32,i32)>{
        Self::neighbors(pos).iter().filter(|n| matches!(self.values.get(n), Some(State::Active))).map(|n| *n).collect()
    }

    fn inactive_neighbors(&self, pos:(i32,i32,i32,i32)) -> Vec<(i32,i32,i32,i32)>{
        Self::neighbors(pos).iter().filter(|n| matches!(self.values.get(n), None)).map(|n| *n).collect()
    }

    fn step(&mut self) {
        let mut next_state = HashMap::new();
        let mut inactive_cells_to_process = HashSet::new();
        for (pos, state) in self.values.iter() {
            match state {
                State::Active => {
                    let active_neighbors = self.active_neighbors(*pos).len();
                    if active_neighbors == 2 || active_neighbors == 3 {
                        next_state.insert(*pos, State::Active);
                    }
                    for c in self.inactive_neighbors(*pos) {
                        inactive_cells_to_process.insert(c);
                    }
                }
            }
        }

        for pos in inactive_cells_to_process {
            let active_neighbors = self.active_neighbors(pos).len();
            if active_neighbors == 3 {
                next_state.insert(pos, State::Active);
            }
        }

        self.values = next_state
    }
}

impl Day for Day17 {
    fn part1(&self, input: &str) -> String {
        let mut cube = Cube::from_str(input);
        for _ in 0..6 {
            cube.step();
        }
        cube.active_cell_count().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut hyper_cube = HyperCube::from_str(input);
        for _ in 0..6 {
            hyper_cube.step();
        }
        hyper_cube.active_cell_count().to_string()
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;

    #[test]
    fn part1_test1() {
        SimpleLogger::new().init();
        assert_eq!(Day17{}.part1(".#.
..#
###"), "112");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day17{}.part2(".#.
..#
###"), "848");
    }
}
