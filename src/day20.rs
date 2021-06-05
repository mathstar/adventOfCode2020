use crate::day::Day;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub struct Day20 {}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Tile {
    id: u64,
    pixels: Vec<Vec<char>>,
    left_edge: Vec<char>,
    right_edge: Vec<char>,
    bottom_edge: Vec<char>
}

impl Tile {
    fn new(id: u64, pixels: Vec<Vec<char>>) -> Tile {
        let left_edge = pixels.iter().map(|v| v[0]).rev().collect();
        let right_edge = pixels.iter().map(|v| v[v.len() - 1]).collect();
        let bottom_edge = pixels.last().unwrap().iter().rev().map(|c| *c).collect();
        Tile {
            id,
            pixels,
            left_edge,
            right_edge,
            bottom_edge
        }
    }

    fn pretty_print(&self) -> String {
        self.pixels.iter().map(|v| v.iter().collect::<String>() + "\n").collect()
    }

    fn rotate_clockwise(&mut self) {
        let mut new_pixels = Vec::new();
        for i in 0..self.pixels[0].len() {
            let mut row = Vec::new();
            for j in (0..self.pixels.len()).rev() {
                row.push(self.pixels[j][i]);
            }
            new_pixels.push(row);
        }
        self.pixels = new_pixels;
        self.left_edge = self.pixels.iter().map(|v| v[0]).rev().collect();
        self.right_edge = self.pixels.iter().map(|v| v[v.len() - 1]).collect();
        self.bottom_edge = self.pixels.last().unwrap().iter().rev().map(|c| *c).collect();
    }

    fn flip_vertically(&mut self) {
        let mut new_pixels = Vec::new();
        for i in (0..self.pixels.len()).rev() {
            new_pixels.push(self.pixels[i].clone());
        }
        self.pixels = new_pixels;
        self.left_edge = self.pixels.iter().map(|v| v[0]).rev().collect();
        self.right_edge = self.pixels.iter().map(|v| v[v.len() - 1]).collect();
        self.bottom_edge = self.pixels.last().unwrap().iter().rev().map(|c| *c).collect();
    }

    fn flip_horizontally(&mut self) {
        let mut new_pixels = Vec::new();
        for i in 0..self.pixels.len() {
            let mut row = Vec::new();
            for j in (0..self.pixels[i].len()).rev() {
                row.push(self.pixels[i][j]);
            }
            new_pixels.push(row);
        }
        self.pixels = new_pixels;
        self.left_edge = self.pixels.iter().map(|v| v[0]).rev().collect();
        self.right_edge = self.pixels.iter().map(|v| v[v.len() - 1]).collect();
        self.bottom_edge = self.pixels.last().unwrap().iter().rev().map(|c| *c).collect();
    }

    fn top_edge(&self) -> &Vec<char> {
        &self.pixels[0]
    }

    fn top_edge_id(&self) -> EdgeId {
        EdgeId{pixels: self.top_edge().to_owned()}
    }

    fn bottom_edge(&self) -> &Vec<char> {
        &self.bottom_edge
    }

    fn bottom_edge_id(&self) -> EdgeId {
        EdgeId{pixels: self.bottom_edge().to_owned()}
    }

    fn left_edge(&self) -> &Vec<char> {
        &self.left_edge
    }

    fn left_edge_id(&self) -> EdgeId {
        EdgeId{pixels: self.left_edge().to_owned()}
    }

    fn right_edge(&self) -> &Vec<char> {
        &self.right_edge
    }

    fn right_edge_id(&self) -> EdgeId {
        EdgeId{pixels: self.right_edge().to_owned()}
    }
}

#[derive(Debug)]
struct EdgeId {
    pixels: Vec<char>
}

impl PartialEq for EdgeId {
    fn eq(&self, other: &Self) -> bool {
        self.pixels == other.pixels || self.pixels.iter().rev().map(|c| *c).collect::<Vec<char>>() == other.pixels
    }
}

impl Hash for EdgeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pixels.iter().filter(|c| **c == '#').count().hash(state)
    }
}

impl Eq for EdgeId {}

fn parse_input(input: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut id = 0;
    let mut pixels = Vec::new();

    for line in input.lines() {
        if line.starts_with("Tile") {
            id = line.strip_prefix("Tile ").unwrap()
                .strip_suffix(":").unwrap()
                .parse().unwrap();
        } else if line.is_empty() {
            tiles.push(Tile::new(id, pixels.to_owned()));
            pixels = Vec::new();
        } else {
            pixels.push(line.chars().collect());
        }
    }
    if pixels.len() > 0 {
        tiles.push(Tile::new(id, pixels.to_owned()));
    }
    tiles
}

fn num_unique_sides(tile: &Tile, edge_map: &HashMap<EdgeId, Vec<&Tile>>) -> usize {
    let mut num = 0;
    if edge_map.get(&tile.top_edge_id()).unwrap().len() == 1 {
        num += 1;
    }
    if edge_map.get(&tile.bottom_edge_id()).unwrap().len() == 1 {
        num += 1;
    }
    if edge_map.get(&tile.left_edge_id()).unwrap().len() == 1 {
        num += 1;
    }
    if edge_map.get(&tile.right_edge_id()).unwrap().len() == 1 {
        num += 1;
    }

    num
}

impl Day for Day20 {
    fn part1(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let mut edge_map : HashMap<EdgeId, Vec<&Tile>> = HashMap::new();

        for tile in tiles.iter() {
            let top_edge_id = tile.top_edge_id();
            match edge_map.get_mut(&top_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(top_edge_id, v);
                }
            }

            let bottom_edge_id = tile.bottom_edge_id();
            match edge_map.get_mut(&bottom_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(bottom_edge_id, v);
                }
            }

            let left_edge_id = tile.left_edge_id();
            match edge_map.get_mut(&left_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(left_edge_id, v);
                }
            }

            let right_edge_id = tile.right_edge_id();
            match edge_map.get_mut(&right_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(right_edge_id, v);
                }
            }
        }

        let mut edge_tiles = HashSet::new();
        //println!("{}", &edge_map.iter().map(|(k, v)| format!("{:?}", k) + v.len().to_string().as_str() + "\n").fold("".to_owned(), |a,b| a + &b));

        for edges in edge_map.values() {
             if edges.len() == 1 {
                 edge_tiles.insert(edges[0]);
             }
        }

        edge_tiles.iter().filter(|t| num_unique_sides(**t, &edge_map) == 2).map(|t| t.id).product::<u64>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let tiles = parse_input(input);

        // identify common edges
        let mut edge_map : HashMap<EdgeId, Vec<&Tile>> = HashMap::new();

        for tile in tiles.iter() {
            let top_edge_id = tile.top_edge_id();
            match edge_map.get_mut(&top_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(top_edge_id, v);
                }
            }

            let bottom_edge_id = tile.bottom_edge_id();
            match edge_map.get_mut(&bottom_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(bottom_edge_id, v);
                }
            }

            let left_edge_id = tile.left_edge_id();
            match edge_map.get_mut(&left_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(left_edge_id, v);
                }
            }

            let right_edge_id = tile.right_edge_id();
            match edge_map.get_mut(&right_edge_id) {
                Some(v) => v.push(tile),
                None => {
                    let mut v = Vec::new();
                    v.push(tile);
                    edge_map.insert(right_edge_id, v);
                }
            }
        }

        let mut edge_tiles = HashSet::new();

        for edges in edge_map.values() {
            if edges.len() == 1 {
                edge_tiles.insert(edges[0]);
            }
        }

        // arrange tiles in grid with correct orientations
        let mut arranged_tiles = Vec::new();
        let mut current_row = Vec::new();
        let mut first_corner = (**edge_tiles.iter().find(|t| num_unique_sides(**t, &edge_map) == 2).unwrap()).clone();
        while edge_map.get(&first_corner.left_edge_id()).unwrap().len() != 1 || edge_map.get(&first_corner.top_edge_id()).unwrap().len() != 1 {
            first_corner.rotate_clockwise();
        }
        current_row.push(first_corner);

        let mut tiles_placed = 1;
        while tiles_placed < tiles.len() {
            let mut last_tile = current_row.last();
            match last_tile {
                Some(last_tile) => {
                    match edge_map.get(&last_tile.right_edge_id()).unwrap().iter().find(|t| t.id != last_tile.id) {
                        Some(next_tile) => {
                            let mut next_tile = (**next_tile).clone();
                            while next_tile.left_edge_id() != last_tile.right_edge_id() {
                                next_tile.rotate_clockwise();
                            }

                            if next_tile.left_edge().iter().collect::<String>() != last_tile.right_edge().iter().rev().collect::<String>() {
                                next_tile.flip_vertically();
                            }
                            current_row.push(next_tile);
                            tiles_placed += 1;
                        },
                        None => {
                            arranged_tiles.push(current_row);
                            current_row = Vec::new();
                        }
                    }
                },
                None => {
                    // start of row
                    let start_of_last_row = arranged_tiles.last().unwrap().first().unwrap();
                    match edge_map.get(&start_of_last_row.bottom_edge_id()).unwrap().iter().find(|t| t.id != start_of_last_row.id) {
                        Some(next_tile) => {
                            let mut next_tile = (**next_tile).clone();
                            while next_tile.top_edge_id() != start_of_last_row.bottom_edge_id() {
                                next_tile.rotate_clockwise();
                            }

                            if next_tile.top_edge().iter().collect::<String>() != start_of_last_row.bottom_edge().iter().rev().collect::<String>() {
                                next_tile.flip_horizontally();
                            }
                            current_row.push(next_tile);
                            tiles_placed += 1;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
        arranged_tiles.push(current_row);
        //println!("{}", arranged_tiles.iter().map(|r| r.iter().map(|t| t.pretty_print() + "\n").collect::<String>() + "\n*******\n").collect::<String>());

        // stitch tiles into single tile

        // iterate through orientations to find sea monster

        // count rough ocean pixels

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day20{}.part1("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."), "20899048083289");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day20{}.part2("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."), "273");
    }

    // #[test]
    // fn rotate_clockwise_test() {
    //     let mut tile = Tile::new(0, vec![vec!['#', '0', '0', '0'],
    //                                                    vec!['#', '0', '0', '#'],
    //                                                    vec!['0', '0', '0', '0'],
    //                                                    vec!['#', '0', '#', '0']]);
    //     let left_id = tile.left_edge_id();
    //     let right_id = tile.right_edge_id();
    //     let top_id = tile.top_edge_id();
    //     let bottom_id = tile.bottom_edge_id();
    //
    //     tile.rotate_clockwise();
    //
    //     println!("{}", tile.pixels.iter()
    //         .map(|r| r.iter()
    //             .map(|c| *c)
    //             .fold("".to_owned(), |acc, a| acc + &a.to_string()).to_owned() + "\n")
    //         .fold("".to_owned(), |acc, a| acc + &a.to_string()));
    //     assert!(left_id.eq(&tile.top_edge_id()));
    //     assert!(top_id.eq(&tile.right_edge_id()));
    //     assert!(right_id.eq(&tile.bottom_edge_id()));
    //     assert!(bottom_id.eq(&tile.left_edge_id()));
    // }
}
