use std::ops::Add;
use std::slice::Iter;
use std::{collections::HashMap, fs};

#[allow(unused)]
const TEST1_FILE: &str = "test1.txt";
#[allow(unused)]
const TEST2_FILE: &str = "test2.txt";
#[allow(unused)]
const INPUT_FILE: &str = "input.txt";
#[allow(unused)]
const TEST3_FILE: &str = "test3.txt";
#[allow(unused)]
const TEST4_FILE: &str = "test4.txt";

#[allow(unused)]
const START_TILE: char = 'S';
#[allow(unused)]
const NOTHING_TILE: char = '.';

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vec2 {
    pub i: i32,
    pub j: i32,
}

pub struct Rect {
    start: Vec2,
    end: Vec2,
}

impl Rect {
    fn from_vec<T>(from: &Vec<Vec<T>>) -> Self {
        Self {
            start: Vec2 { i: 0, j: 0 },
            end: Vec2 {
                i: from.len() as i32,
                j: from[0].len() as i32,
            },
        }
    }
}

impl Vec2 {
    fn is_inside(&self, rect: &Rect) -> bool {
        self.i >= rect.start.i
            && self.i < rect.end.i
            && self.j >= rect.start.j
            && self.j < rect.end.j
    }

    fn from_usize(i: usize, j: usize) -> Self {
        Self {
            i: i as i32,
            j: j as i32,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Self> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [Top, Bottom, Left, Right];
        DIRECTIONS.iter()
    }

    pub fn as_vec2(self) -> Vec2 {
        use Direction::*;
        match self {
            Top => Vec2 { i: -1, j: 0 },
            Bottom => Vec2 { i: 1, j: 0 },
            Left => Vec2 { i: 0, j: -1 },
            Right => Vec2 { i: 0, j: 1 },
        }
    }

    pub fn reversed(self) -> Self {
        use Direction::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
}

// #[derive(Clone, Copy, Debug)]
// enum Status {
//     In,
//     Out,
// }

type Tiles = HashMap<char, HashMap<Direction, Direction>>;
type Board = Vec<Vec<char>>;

// All the pipe types, mapped from char to directions
fn tiles() -> Tiles {
    use Direction::*;

    HashMap::from([
        ('F', HashMap::from([(Right, Bottom), (Bottom, Right)])),
        ('L', HashMap::from([(Right, Top), (Top, Right)])),
        ('J', HashMap::from([(Left, Top), (Top, Left)])),
        ('7', HashMap::from([(Left, Bottom), (Bottom, Left)])),
        ('|', HashMap::from([(Top, Bottom), (Bottom, Top)])),
        ('-', HashMap::from([(Right, Left), (Left, Right)])),
    ])
}

#[allow(unused)]
fn print_board<T: std::fmt::Display>(board: &[Vec<T>]) {
    for row in board.iter() {
        for tile in row {
            print!("{tile} ");
        }
        println!();
    }
}

#[allow(unused)]
fn print_mask(board: &[Vec<bool>]) {
    for row in board.iter() {
        for &tile in row {
            print!("{} ", if tile { "@" } else { "." });
        }
        println!();
    }
}

fn parse_input(input: &str) -> Board {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start(board: &Board) -> Vec2 {
    for (i, row) in board.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == START_TILE {
                return Vec2 {
                    i: i as i32,
                    j: j as i32,
                };
            }
        }
    }
    panic!("Could not find start tile (`S`) in the provided board!");
}

fn find_neighbours<T>(point: &Vec2, board: &Vec<Vec<T>>) -> Vec<(Vec2, Direction)> {
    let mut neighbours = vec![];

    for direction in Direction::iterator() {
        let new_point = direction.as_vec2() + point.clone();
        if new_point.is_inside(&Rect::from_vec(board)) {
            neighbours.push((new_point, *direction));
        }
    }

    neighbours
}

fn find_loop(board: &Board, tiles: &Tiles) -> Vec<Vec2> {
    // find all neighbours (north, west, sout, east) of the
    // starting tile that connected are pipes
    // choose one, follow it's direction until you
    // end up at `S` (start) again

    let start = find_start(board);
    // println!("Starting at point {:?}", start);
    let (mut current_point, mut next_direction) = (start.clone(), Direction::Bottom);
    let mut points = vec![];
    let mut found = false;

    let neighbours = find_neighbours(&current_point, board);
    // println!("Neighbours of tile {start:?}: {neighbours:?}");
    for (point, direction) in neighbours {
        let tile = board[point.i as usize][point.j as usize];
        if tile == NOTHING_TILE {
            continue;
        }
        // println!("key: `{tile}`");
        if tiles[&tile].get(&direction.reversed()).is_some() {
            // println!("Direction: {:?}, dir: {:?}", direction, dir);
            found = true;
            next_direction = direction;
            current_point = point;
            points.push(current_point.clone());
            break;
        }
    }
    if !found {
        panic!("Did not find any connected pipes!");
    }

    // println!("Chose to go in direction: {next_direction:?}, to point {current_point:?}, tile: {:?}", board[current_point.i as usize][current_point.j as usize]);

    while current_point != start {
        let tile = board[current_point.i as usize][current_point.j as usize];
        // println!("key: {tile:?}");
        next_direction = tiles[&tile][&next_direction.reversed()];
        current_point = next_direction.as_vec2() + current_point;
        // println!("Point: {current_point:?}");
        points.push(current_point.clone());
    }

    points.dedup();

    points
}

/// Ray casting - doesn't really work in the 2d tiled case
// #[allow(unused)]
// fn is_inside_loop(start: &Vec2, pipe_loop: &[Vec2], board: &Board) -> bool {
//     let mut min_counter = u32::MAX;
//     for direction in Direction::iterator() {
//         let mut intersection_counter = 0u32;
//         let mut current_point = start.clone();

//         while current_point.is_inside(&Rect::from_vec(board)) {
//             if pipe_loop.contains(&current_point) {
//                 intersection_counter += 1;
//             }
//             current_point = direction.as_vec2() + current_point;
//         }
//         if intersection_counter < min_counter {
//             min_counter = intersection_counter;
//         }
//     }
//
//     min_counter % 2 == 1
// }

fn unchecked_neighbours(start: &Vec2, board: &Vec<Vec<bool>>) -> Vec<Vec2> {
    let mut neighbours = vec![];

    for direction in Direction::iterator() {
        let point = start.clone() + direction.as_vec2();
        if point.is_inside(&Rect::from_vec(board)) && !board[point.i as usize][point.j as usize] {
            neighbours.push(point);
        }
    }

    neighbours
}

/// Works for the case of stretched map
fn flood_fill_iterative(start: &Vec2, mask: &mut Vec<Vec<bool>>) {
    let mut to_check = vec![start.clone()];

    while let Some(current_point) = to_check.pop() {
        let mut neighbours = unchecked_neighbours(&current_point, mask);
        to_check.append(&mut neighbours);

        // println!("Point: {:?}", current_point);
        mask[current_point.i as usize][current_point.j as usize] = true;
    }
}

#[allow(unused)]
fn flood_fill_recursive(start: &Vec2, mask: &mut Vec<Vec<bool>>) {
    for direction in Direction::iterator() {
        let current_point = direction.as_vec2() + start.clone();
        // println!("point: {:?}", current_point);
        if current_point.is_inside(&Rect::from_vec(mask))
            && !mask[current_point.i as usize][current_point.j as usize]
        {
            mask[current_point.i as usize][current_point.j as usize] = true;
            flood_fill_recursive(&current_point, mask);
        }
    }
}

/// Formula: i'th point on a board is just the 2i+1'th point in the mask
fn board_to_mask(point: &Vec2) -> Vec2 {
    Vec2 {
        i: point.i * 2 + 1,
        j: point.j * 2 + 1,
    }
}

fn is_board(mask_point: &Vec2) -> bool {
    const OFFSET: i32 = 1;
    mask_point.i % 2 == OFFSET && mask_point.j % 2 == OFFSET
}

// floodfill!!
// NEW APPROACH: add half-coordinates!
fn find_inside_recursive(pipe_loop: &[Vec2], board: &Board, tiles: &Tiles) -> Vec<Vec2> {
    // the plan:
    // 1. make a bool mask 2n+1 larger in every direction
    // 2. mark all tiles in pipe loop as true (i and j offset by +1)
    // 3. iterate (recursively) over the tiles:
    // starting at (0, 0):
    // - mark the tile true (if it isn't marked itself yet)
    // - find its neighbours and repeat for all of the neighbours
    let mut mask = vec![vec![false; 2 * board[0].len() + 1]; 2 * board.len() + 1];

    for (i, row) in board.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let current_point = Vec2::from_usize(i, j);

            if pipe_loop.contains(&current_point) {
                let mask_point = board_to_mask(&current_point);

                mask[mask_point.i as usize][mask_point.j as usize] = true;
                if *tile == 'S' {
                    continue;
                }

                for direction in tiles[tile].keys() {
                    let new_point = board_to_mask(&current_point.clone()) + direction.as_vec2();
                    if new_point.is_inside(&Rect::from_vec(&mask)) {
                        mask[new_point.i as usize][new_point.j as usize] = true;
                    }
                }
            }
        }
    }

    println!("Mask:");
    print_mask(&mask);
    // println!("Mask after marking loop:");
    // print_mask_bmp(&mask);

    flood_fill_iterative(&Vec2 { i: 0, j: 0 }, &mut mask);

    println!("Flood filled:");
    print_mask(&mask);

    // println!("Mask after marking all adjacent + loop:");
    // print_mask(&mask);

    // find all points that appear on the board and are not filled
    // formula:
    let mut inside = vec![];
    for (i, row) in mask.clone().iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_board(&Vec2::from_usize(i, j)) && !mask[i][j] {
                inside.push(Vec2::from_usize(i, j));
            }
        }
    }

    inside
}

// #[allow(unused)]
// // Arbitrary chosen raycasting direction
// const RAYCAST_DIRECTION: Direction = Direction::Right;

// // true -> is inside, false -> is outside
// fn cast_ray(start: &Vec2, pipe_loop: &[Vec2], board: &Board) -> bool {
//     if pipe_loop.contains(start) {
//         return false;
//     }

//     let mut crossings = 0;
//     let mut current_point = start.clone();

//     while current_point.is_inside(&Rect::from_vec(board)) {
//         let current_tile = board[current_point.i as usize][current_point.j as usize];
//         if pipe_loop.contains(&current_point) {
//             match current_tile {
//                 'S' | '|' | 'F' | '7' =>
//                     crossings += 1,
//                 _ => {},
//             };
//         };

//         current_point = RAYCAST_DIRECTION.as_vec2() + current_point;
//     }
//     if crossings == 0 {
//         return false;
//     }
//     crossings % 2 == 1
// }

/// Utility functions
fn print_loop(pipe_loop: &[Vec2], board: &Board) {
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if pipe_loop.contains(&Vec2::from_usize(i, j)) {
                print!("@ ");
            } else {
                print!(". ");
            }
        }
        println!()
    }
}
// fn print_loop_inside(inside: &[Vec2],pipe_loop: &[Vec2], board: &Board) {
//     for (i, row) in board.iter().enumerate() {
//         for (j, _) in row.iter().enumerate() {
//             let point = Vec2::from_usize(i, j);
//             if inside.contains(&point) {
//                 print!("@ ");
//             }
//             else if pipe_loop.contains(&point) {
//                 print!("{} ", board[i][j]);
//             } else {
//                 print!(". ");
//             }
//         }
//         println!()
//     }
// }

// DID NOT WORK!
// sadly, there seems to be a problem with casting rays - too many edgecases
// fn find_inside(pipe_loop: &[Vec2], board: &Board) -> Vec<Vec2> {
//     // NEW plan:
//     // RAY CASTING! cast rays for each tile
//     // pros: only have to cast 1 ray for each tile
//     // cons: have to only count intersections
//     // with tiles oriented perpendicular
//     // ('-' for vertical and '|' for horizontal directions)
//     // really hope it works!!!
//     let mut points = vec![];

//     for (i, row) in board.iter().enumerate() {
//         let mut status = Status::Out;

//         for (j, tile) in row.iter().enumerate() {
//             let point = Vec2::from_usize(i, j);
//              if pipe_loop.contains(&point) {
//                 match tile {
//                     'S' | 'F' | '|' | '7' =>
//                         status = match status {
//                             Status::In => Status::Out,
//                             Status::Out => Status::In,
//                         },
//                     _ => continue,
//                 };
//              } else {
//                 match status {
//                     Status::In => points.push(point),
//                     Status::Out => continue,
//                 }
//              }

//         }
//     }

//     points.dedup();

//     points
// }

fn main() {
    let filename = INPUT_FILE;
    let input = fs::read_to_string(filename).unwrap_or_else(|err| {
        panic!(
            "Unexpected error occured while opening file {}, error: {:#?}",
            filename, err
        )
    });

    let board = parse_input(&input);
    let tiles: Tiles = tiles();
    println!("Board: ");
    print_board(&board);

    // println!("Board: {:?}", board);
    // println!("tiles: {:?}", tiles);

    let pipe_loop = find_loop(&board, &tiles);
    //println!("Loop: {pipe_loop:#?}");
    println!("Pipe loop: ");
    print_loop(&pipe_loop, &board);

    // println!("Board dimensions: {} x {} ({} tiles)", board.len(), board[0].len(), board.len() * board[0].len());

    let _ = pipe_loop.len() / 2;
    // println!("Farthest: {farthest}");

    // let inside = find_inside(&pipe_loop, &board);
    // println!("There are {} inside tiles", inside.len());
    // println!("Loop inside:");
    // print_loop_inside(&inside, &pipe_loop, &board);

    let inside = find_inside_recursive(&pipe_loop, &board, &tiles);
    println!("Points inside: {:?}", inside.len());
}
