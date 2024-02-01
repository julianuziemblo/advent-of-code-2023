use std::{fs, collections::HashSet};
use std::ops::Range;

#[allow(unused)]
const TEST_PATH: &str = "test.txt";
#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

type Board = Vec<Vec<char>>;
type Pairs = HashSet<(Point, Point)>;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Point {
    i: usize,
    j: usize,
}

#[allow(unused)]
fn print_board(board: &Board) {
    for row in board.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!()
    }
}

fn parse_input(input: &str) -> Board {
    input.trim()
        .lines()
        .map(|line|
            line.chars()
                .collect()
        )
        .collect()
}

fn is_empty(slice: &[char]) -> bool {
    for &tile in slice.iter() {
        if tile != '.' {
            return false;
        }
    }
    true
}

fn get_column(column_index: usize, board: &Board) -> Vec<char> {
    board.iter()
        .map(|row|
            row[column_index]
        )
        .collect()
}

fn insert_column(column_index: usize, board: &mut Board) {
    for (i, _) in board.clone().iter().enumerate() {
        board[i].insert(column_index, '.');
    }
}

fn expand(board: &mut Board) {
    let mut i = 0;
    while let Some(row) = board.get(i) {
        // println!("row {i}: {:?}", row);
        if is_empty(&board[i]) {
            board.insert(i, row.clone());
            i += 1;
        }
        i += 1;
    }

    let mut j = 0;
    while board[0].get(j).is_some() {
        // println!("row {i}: {:?}", row);
        if is_empty(&get_column(j, board)) {
            insert_column(j, board);
            j += 1;
        }
        j += 1;
    }
}

fn get_galaxies(board: &Board) -> Vec<Point> {
    let mut galaxies = vec![];
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                galaxies.push(Point { i, j })
            }
        }
    }

    galaxies
}

fn pairs(board: &Board) -> Pairs {
    let mut pairs = HashSet::new();
    let galaxies = get_galaxies(board);

    for galaxy1 in galaxies.iter() {
        for galaxy2 in galaxies.iter()  {
            if !(pairs.contains(&(galaxy1.clone(), galaxy2.clone())) || pairs.contains(&(galaxy2.clone(), galaxy1.clone()))) {
                pairs.insert((galaxy1.clone(), galaxy2.clone()));
            }
        }
    }

    pairs
}

fn shortest_path(pair: &(Point, Point)) -> usize {
    let (a, b) = pair;

    a.i.abs_diff(b.i) + a.j.abs_diff(b.j)
}

fn sum_shortest_paths(board: &Board) -> usize {
    let mut sum = 0;
    let pairs = pairs(board);

    for pair in pairs.iter() {
        sum += shortest_path(pair);
    }

    sum
}

fn get_should_expand_rows(board: &Board) -> Vec<usize> {
    let mut should_expand_rows = vec![];

    for (i, row) in board.iter().enumerate() {
        if is_empty(row) {
            should_expand_rows.push(i);
        }
    }

    should_expand_rows
}

fn get_should_expand_cols(board: &Board) -> Vec<usize> {
    let mut should_expand_cols = vec![];

    for (j, _) in board[0].iter().enumerate() {
        if is_empty(&get_column(j, board)) {
            should_expand_cols.push(j);
        }
    }

    should_expand_cols
}

fn abs_range(a: usize, b: usize) -> Range<usize> {
    if a > b {
        return (b+1)..a;
    }
    (a+1)..b
}

fn sum_shortest_paths_bigger_expansion(board: &Board, multiplier: usize) -> usize {
    let mut sum = 0;

    let should_expand_rows = get_should_expand_rows(board);
    let should_expand_cols = get_should_expand_cols(board);
    let pairs = pairs(board);

    for (a, b) in pairs.iter() {
        let path = shortest_path(&(a.clone(), b.clone()));
        let di = abs_range(a.i, b.i);
        let dj = abs_range(a.j, b.j);
        let rows = should_expand_rows.iter()
            .filter(|&i| di.contains(i))
            .count();
        let cols = should_expand_cols.iter()
            .filter(|&j| dj.contains(j))
            .count();
        sum += path + cols * (multiplier - 1) + rows * (multiplier - 1);
    }

    sum
}

fn main() {
    let path = INPUT_PATH;
    let input = fs::read_to_string(path).unwrap_or_else(
        |err| 
            panic!("Could not open file {:?} due to error {:?}", path, err)
    );

    let mut board = parse_input(&input);
    let original_board = board.clone();

    println!("Board before expansion ({}x{})", board.len(), board[0].len());
    print_board(&board);

    expand(&mut board);

    println!("Board after expansion ({}x{})", board.len(), board[0].len());
    print_board(&board);

    // part 1
    println!("Sum of shortest paths: {}", sum_shortest_paths(&board));

    // part 2
    let multiplier = 1_000_000;
    println!(
        "Sum of shortest paths ({}x expansion):\n{}", 
        multiplier,
        sum_shortest_paths_bigger_expansion(&original_board, multiplier)
    );
}
