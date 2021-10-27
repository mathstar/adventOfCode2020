use std::collections::HashMap;
use crate::day::Day;

// Using Axial hexagonal grid coordinates, see https://www.redblobgames.com/grids/hexagons/#coordinates-axial

pub struct Day24 {}

#[derive(Debug)]
struct Tile {
    black: bool
}

impl Tile {
    fn new() -> Tile {
        Tile {black: false}
    }
}

fn directions_to_coordinates(input: &str) -> (i32, i32) {
    let mut coord = (0, 0);

    let mut prev = None;
    for char in input.chars() {
        match char {
            'e' => match prev {
                Some('n') => {
                    coord = (coord.0 + 1, coord.1 - 1);
                    prev = None;
                },
                Some('s') => {
                    coord = (coord.0, coord.1 + 1);
                    prev = None;
                },
                None => coord = (coord.0 + 1, coord.1),
                _ => panic!("Invalid input")
            },
            'w' => match prev {
                Some('n') => {
                    coord = (coord.0, coord.1 - 1);
                    prev = None;
                },
                Some('s') => {
                    coord = (coord.0 - 1, coord.1 + 1);
                    prev = None;
                },
                None => coord = (coord.0 - 1, coord.1),
                _ => panic!("Invalid input")
            },
            'n' | 's' => prev = Some(char),
            _ => panic!("Invalid input character")
        }
    }

    coord
}

// generate the initial tile state based on the input directions
fn generate_initial_grid(input: &str) -> HashMap<i32, HashMap<i32, Tile>> {
    let mut tiles = HashMap::new();
    tiles.insert(0,HashMap::new());
    tiles.get_mut(&0).unwrap().insert(0, Tile::new());

    for line in input.lines() {
        let coord = directions_to_coordinates(line);

        let mut tile = get_or_generate_tile(coord.0, coord.1, &mut tiles);
        tile.black = !tile.black;
    }
    tiles
}

// gets the tile at the given coordinate, initializing it as needed
fn get_or_generate_tile(x: i32, y: i32, grid: &mut HashMap<i32, HashMap<i32, Tile>>) -> &mut Tile {
    if !grid.contains_key(&x) {
        grid.insert(x, HashMap::new());
    }

    if !grid.get(&x).unwrap().contains_key(&y) {
        grid.get_mut(&x).unwrap().insert(y, Tile::new());
    }
    grid.get_mut(&x).unwrap().get_mut(&y).unwrap()
}

// counts the number of black tiles adjacent to each tile
fn count_adjacencies(grid: &HashMap<i32, HashMap<i32, Tile>>) -> HashMap<i32, HashMap<i32, i32>> {
    let mut adjacencies = HashMap::new();

    // populate the map initially with all present tiles, that way we capture existing tiles with no adjacent black tiles
    for (x, y) in grid.iter().flat_map(|(x, r)| r.keys().map(move |y| (*x,*y))) {
        populate_value(x, y, &mut adjacencies);
    }

    // for each black tile, increment counts for all neighbors
    for (x,y) in grid.iter().flat_map(|(x, r)| r.iter().filter(|(_, t)| t.black).map(move |(y, _)| (*x,*y))) {
        // increment each neighbor
        increment_value(x, y - 1, &mut adjacencies);
        increment_value(x + 1, y - 1, &mut adjacencies);
        increment_value(x + 1, y, &mut adjacencies);
        increment_value(x, y + 1, &mut adjacencies);
        increment_value(x - 1, y + 1, &mut adjacencies);
        increment_value(x - 1, y, &mut adjacencies);
    }
    adjacencies
}

// initialize an adjacency value as 0
fn populate_value(x: i32, y:i32, adjacencies: &mut HashMap<i32, HashMap<i32, i32>>) {
    if !adjacencies.contains_key(&x) {
        adjacencies.insert(x, HashMap::new());
    }
    if !adjacencies.get(&x).unwrap().contains_key(&y) {
        adjacencies.get_mut(&x).unwrap().insert(y, 0);
    }
}

// increment an adjacency value
fn increment_value(x: i32, y:i32, adjacencies: &mut HashMap<i32, HashMap<i32, i32>>) {
    populate_value(x, y, adjacencies);
    let v = *adjacencies.get(&x).unwrap().get(&y).unwrap();
    adjacencies.get_mut(&x).unwrap().insert(y, v + 1);
}

// prints all present tiles as a tuple (x, y, black?)
#[allow(dead_code)]
fn pretty_print_grid(grid: &HashMap<i32, HashMap<i32, Tile>>) {
    for line in grid.iter()
        .flat_map(|(x, r)| r.iter().map(move |(y, t)| (x,y,t.black)))
        .map(|(x,y,t)| format!("({},{},{})", x, y, t)) {
        println!("{}", line);
    }
}

impl Day for Day24 {
    fn part1(&self, input: &str) -> String {
        let tiles = generate_initial_grid(input);
        tiles.values().flat_map(|r| r.values()).filter(|t| t.black).count().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut tiles = generate_initial_grid(input);

        for _ in 0..100 {
            // count adjacencies for all tiles
            let adjacencies = count_adjacencies(&tiles);

            // for each tile in the adjacency map, apply lifecycle rules
            for (x, y, v) in adjacencies.iter().flat_map(|(x,r)| r.iter().map(move |(y, v)| (*x,*y,*v))) {
                let mut tile = get_or_generate_tile(x, y, &mut tiles);
                if tile.black && v != 1 && v != 2 {
                    tile.black = false;
                } else if !tile.black && v == 2 {
                    tile.black = true;
                }
            }
        }
        tiles.values().flat_map(|r| r.values()).filter(|t| t.black).count().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day24{}.part1("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"), "10");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day24{}.part2("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"), "2208");
    }
}
