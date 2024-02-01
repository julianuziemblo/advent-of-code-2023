use std::fs;

#[allow(unused)]
const TEST_PATH: &str = "test.txt";
#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn inverse(&self) -> Self {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

type Board = Vec<Vec<Tile>>;
type Sum = usize;

const ROW_MULTIPLIER: Sum = 100;

trait GridUtility {
    type Output: Copy;

    fn row(&self, index: usize) -> Vec<Self::Output>;
    fn column(&self, index: usize) -> Vec<Self::Output>;
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
}

impl GridUtility for Board {
    type Output = Tile;

    fn row(&self, index: usize) -> Vec<Self::Output> {
        self[index].clone()
    }

    fn column(&self, index: usize) -> Vec<Self::Output> {
        self.iter().map(|row| row[index]).collect()
    }

    fn rows(&self) -> usize {
        self.len()
    }

    fn columns(&self) -> usize {
        self[0].len()
    }
}

#[allow(unused)]
fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!(
                "{} ",
                match cell {
                    Tile::Ash => ".",
                    Tile::Rock => "#",
                }
            )
        }
        println!()
    }
}

#[allow(unused)]
fn print_boards(boards: &[Board]) {
    for (i, board) in boards.iter().enumerate() {
        println!("Board {i}:");
        print_board(board);
        println!()
    }
}

fn parse_input(input: &str) -> Vec<Board> {
    input
        .trim()
        .split("\r\n\r")
        .map(|board| {
            board
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            '#' => Tile::Rock,
                            '.' => Tile::Ash,
                            other => panic!("Unsuported character: {other}"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn find_row_axis(board: &Board) -> Option<(usize, usize)> {
    for i in 0..(board.len() - 1) {
        let (mut p, mut q) = (i as i32, i as i32 + 1);
        while board[p as usize] == board[q as usize] {
            p -= 1;
            q += 1;

            if p < 0 || q >= board.len() as i32 {
                return Some((i, i + 1));
            }
        }
    }

    None
}

fn find_col_axis(board: &Board) -> Option<(usize, usize)> {
    for j in 0..(board[0].len() - 1) {
        let (mut p, mut q) = (j as i32, j as i32 + 1);
        while board.column(p as usize) == board.column(q as usize) {
            p -= 1;
            q += 1;

            if p < 0 || q >= board[0].len() as i32 {
                return Some((j, j + 1));
            }
        }
    }

    None
}

fn sum_for_board(board: &Board) -> Sum {
    let mut sum = 0;
    // print_board(board);
    #[allow(unused)]
    if let Some((top, bottom)) = find_row_axis(board) {
        // println!("Found row axis: {:?}", (top, bottom));
        sum += (top + 1) * ROW_MULTIPLIER;
    }
    #[allow(unused)]
    if let Some((left, right)) = find_col_axis(board) {
        // println!("Found column axis: {:?}", (left, right));
        sum += left + 1;
    }

    sum
}

fn sum_for_boards(boards: &[Board]) -> Sum {
    // println!("\n#########################");
    // println!("SUMMING BOARDS:");
    boards.iter().map(sum_for_board).sum()
}

fn find_diffs(row_a: Vec<Tile>, row_b: Vec<Tile>) -> Vec<usize> {
    let mut accum = vec![];
    for (i, (tile_a, tile_b)) in row_a.iter().zip(row_b.iter()).enumerate() {
        if tile_a != tile_b {
            accum.push(i);
        }
    }
    accum
}

fn find_row_smudge(
    board: &Board,
    previous_axis: (usize, usize),
    is_row: bool,
) -> Option<(usize, usize)> {
    // println!("Finding row axis for board:");
    // print_board(board);
    let (axis_i, axis_j);
    if is_row {
        (axis_i, axis_j) = previous_axis;
    } else {
        (axis_i, axis_j) = (board.rows() + 1, board.rows() + 1);
    }
    // println!("Found row axis: ({}, {})", axis_i, axis_j);
    for i in 0..(board.rows() - 1) {
        //println!("NEXT ITERATION");
        let (mut p, mut q) = (i as i32, i as i32 + 1);
        let mut diffs = vec![];
        let mut diffs_len;
        let mut the_row = 0;

        while diffs.len() <= 1 && p != axis_i as i32 && q != axis_j as i32 {
            diffs_len = diffs.len();
            diffs.extend(find_diffs(board.row(p as usize), board.row(q as usize)));
            if diffs.len() != diffs_len {
                the_row = p as usize;
            }
            // println!("Diffs (extended) for p={p}, q={q}: {:?}", diffs);
            p -= 1;
            q += 1;

            if (p < 0 || q >= board.rows() as i32) && diffs.len() == 1 {
                let mut board = board.clone();
                board[the_row][diffs[0]] = board[the_row][diffs[0]].inverse();
                // println!("Found NEW row axis: ({}, {})", i, i + 1);
                // println!("Corrected board:");
                // print_board(&board);
                return Some((i, i + 1));
            }
        }
    }
    // println!("Returning None");
    None
}

fn find_col_smudge(
    board: &Board,
    previous_axis: (usize, usize),
    is_column: bool,
) -> Option<(usize, usize)> {
    // println!("Finding column axis for board:");
    // print_board(board);

    let (axis_i, axis_j);
    if is_column {
        (axis_i, axis_j) = previous_axis;
    } else {
        (axis_i, axis_j) = (board.columns() + 1, board.columns() + 1);
    }

    // println!("Found col axis: ({}, {})", axis_i, axis_j);
    for j in 0..(board.columns() - 1) {
        // println!("NEXT ITERATION");
        let (mut p, mut q) = (j as i32, j as i32 + 1);
        let mut diffs = vec![];
        // let mut diffs_len;
        // let mut the_column = 0;

        while diffs.len() <= 1 && p != axis_i as i32 && q != axis_j as i32 {
            // diffs_len = diffs.len();
            diffs.extend(find_diffs(
                board.column(p as usize),
                board.column(q as usize),
            ));
            // if diffs.len() != diffs_len {
            //     the_column = p as usize;
            // }

            // println!("Diffs (extended) for p={p}, q={q}: {:?}", diffs);
            p -= 1;
            q += 1;

            if (p < 0 || q >= board.columns() as i32) && diffs.len() == 1 {
                // let mut board = board.clone();
                // board[diffs[0]][the_column] = board[diffs[0]][the_column].inverse();
                // println!("Found NEW column axis: ({}, {})", j, j + 1);
                // println!("Corrected board:");
                // print_board(&board);
                return Some((j, j + 1));
            }
        }
    }
    // println!("Returning None");
    None
}

fn sum_for_board_corrected(board: &Board) -> Sum {
    // println!();
    if let Some(axis) = find_row_axis(board) {
        if let Some((top, _)) = find_row_smudge(board, axis, true) {
            return (top + 1) * ROW_MULTIPLIER;
        }
        if let Some((left, _)) = find_col_smudge(board, axis, false) {
            return left + 1;
        }
    }
    if let Some(axis) = find_col_axis(board) {
        if let Some((top, _)) = find_row_smudge(board, axis, false) {
            return (top + 1) * ROW_MULTIPLIER;
        }
        if let Some((left, _)) = find_col_smudge(board, axis, true) {
            return left + 1;
        }
    }

    panic!("Every corrected board should have at least 1 axis!")
}

fn sum_for_boards_corrected(boards: &[Board]) -> Sum {
    boards.iter().map(sum_for_board_corrected).sum()
}

fn main() {
    let path = INPUT_PATH;
    let input = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Could not read the file `{path}` because of {err:?}"));

    let boards: Vec<Board> = parse_input(&input);

    println!("[PART 1] Sum for all boards: {}", sum_for_boards(&boards));

    println!(
        "[PART 2] Sum for all boards: {}",
        sum_for_boards_corrected(&boards)
    );
}
