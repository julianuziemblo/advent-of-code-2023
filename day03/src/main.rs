#[allow(unused)]
use std::fs;

type Matrix<T> = Vec<Vec<T>>;
type Schematic = Matrix<char>;

#[derive(PartialEq, Clone, Debug)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(PartialEq, Clone, Debug)]
struct Number {
    value: u32,
    coords: Point,
    len: usize,
}

#[derive(Clone, Debug)]
struct Gear {
    #[allow(dead_code)]
    coords: Point,
    ratio: u32,
}

#[derive(PartialEq, Clone, Debug)]
enum Mask {
    Num(Number),
    Symbol(char),
    None,
}

#[allow(dead_code)]
fn print_board(board: &Matrix<Mask>) {
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                Mask::Num(_) => print!("N ", ),
                Mask::Symbol(s) => print!("{s} "),
                Mask::None => print!(". "),
            }
        }
        println!("");
    }
}

fn parse_contents(contents: String) -> Schematic {
    let contents = contents.trim();
    let mut res: Schematic = vec![];

    for line in contents.lines() {
        let line = line.trim();
        res.push(line.chars().collect());
    }
    
    res
}

fn get_number(i: usize, mut j: usize, schematic: &Schematic, mask: &mut Matrix<bool>) -> Number {
    let mut num = String::new();
    let coords = Point{ i, j };

    while j < schematic[0].len() && schematic[i][j].is_digit(10) {
        num.push(schematic[i][j]);
        mask[i][j] = false;
        j += 1;
    }

    // println!("num={num}");

    Number { value: num.parse::<u32>().unwrap(), coords, len: num.len() }
}

fn check_column(start_j: i32, number: &Number, schematic: &Schematic) -> bool {
    if start_j >= 0 && start_j < schematic[0].len() as i32 {
        for i in -1..=1 {
            let curr_i = number.coords.i as i32 + i;
            if  curr_i >= 0 && 
                curr_i < schematic.len() as i32 &&
                !(schematic[curr_i as usize][start_j as usize].is_digit(10)) && 
                !(schematic[curr_i as usize][start_j as usize] == '.') {
                return true;
            }
        }
    }
    false
}

fn symbol_in_corners(number: &Number, schematic: &Schematic) -> bool {
    check_column(number.coords.j as i32 - 1, number, schematic) ||
    check_column((number.coords.j + number.len) as i32, number, schematic)
}

// return the value of the number, if it has no symbols as neighbours
// otherwise, return 0
fn parse_numbers_neighbours(number: &Number, schematic: &Schematic) -> u32 {
    // first: check left-top, left and left-bottom (if they exist)
    // also:  check right-top, right and right-left (if they exist)
    // second: check top and bottom for every digit

    if symbol_in_corners(number, schematic) {
        return number.value;
    }

    for dj in 0..number.len {
        if check_column((number.coords.j + dj) as i32, number, schematic) {
            return number.value;
        }
    }

    0
}

fn sum_adjacent(schematic: &Schematic) -> u32 {
    let mut sum: u32 = 0;
    let mut mask: Matrix<bool> = vec![vec![true ; schematic[0].len()]; schematic.len()];

    for (i, row) in schematic.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if mask[i][j] && cell.is_digit(10) {
                let number = get_number(i, j, schematic, &mut mask);
                sum += parse_numbers_neighbours(&number, schematic);
                // println!("Found number: {:#?}", number);
            }
        }
    }

    sum
}


fn get_num(i: usize, mut j: usize, schematic: &Schematic, mask: &mut Matrix<Mask>) -> Number {
    let mut num = String::new();
    let coords = Point{ i, j };

    while j < schematic[0].len() && schematic[i][j].is_digit(10) {
        num.push(schematic[i][j]);
        j += 1;
    }

    // println!("num={num}");

    let num = Number { value: num.parse::<u32>().unwrap(), coords: coords.clone(), len: num.len() };
    for dj in (coords.j)..(j) {
        mask[i][dj] = Mask::Num(num.clone());
    }
    num
}

fn get_matrix(schematic: &Schematic) -> Matrix<Mask> {
    let mut mask: Matrix<Mask> = vec![vec![Mask::None ; schematic[0].len()]; schematic.len()];

    for (i, row) in schematic.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if mask[i][j] == Mask::None && cell.is_digit(10) {
                let _ = get_num(i, j, schematic, &mut mask);
                // println!("Found number: {:#?}", number);
            }
            if mask[i][j] == Mask::None && *cell != '.' {
                mask[i][j] = Mask::Symbol(*cell);
            }
        }
    }

    print_board(&mask);

    mask
}

fn get_gear(i: usize, j: usize, matrix: &Matrix<Mask>) -> Option<Gear> {
    if let Mask::Symbol(_) = matrix[i][j] {
        let mut nums = Vec::<Number>::new();
        for di in -1..=1 {
            for dj in -1..=1 {
                let (curr_i, curr_j) = (i as i32 + di, j as i32 + dj);
                if  curr_i >= 0 && 
                    curr_i < matrix.len() as i32 &&
                    curr_j >= 0 &&
                    curr_j < matrix[0].len() as i32 {
                    
                    if let Mask::Num(num) = matrix[curr_i as usize][curr_j as usize].clone() {
                        if !nums.contains(&num) {
                            // println!("Found num {} for gear in coords ({i}, {j})", num.value);
                            nums.push(num)
                        }
                    }
                } 
            }
        }

        // println!("Nums for gear: {:#?}", nums);

        if nums.len() == 2 {
            return Some(Gear { coords: Point { i, j }, ratio: nums[0].value * nums[1].value });
        }
        
    }
    None
} 

    

fn get_gears(schematic: &Schematic) -> Vec<Gear> {
    let matrix = get_matrix(schematic);
    let mut gears = Vec::<Gear>::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if let Some(gear) = get_gear(i, j, &matrix) {
                gears.push(gear);
            }
        }
    }

    gears
}

fn sum_gear_ratios(schematic: &Schematic) -> u32 {
    let mut sum = 0;
    let gears = get_gears(schematic);

    // println!("{:#?}", gears);

    for gear in gears.iter() {
        sum += gear.ratio;
    }

    sum
}

fn main() {
    // let contents = 
    //     r#"
    //     467..114..
    //     ...*......
    //     ..35..633.
    //     ......#...
    //     617*......
    //     .....+.58.
    //     ..592.....
    //     ......755.
    //     ...$.*....
    //     .664.598..
    //     "#.to_owned();

    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect(format!("Couldn't open file {filename}.").as_str());

    let schematic: Schematic = parse_contents(contents);
    // println!("schematic = {:?}", schematic);
    println!("[PART 1] sum = {}", sum_adjacent(&schematic));
    println!("[PART 2] sum = {}", sum_gear_ratios(&schematic));
}
