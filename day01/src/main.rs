#[allow(unused)]
use std::fs;

fn part1() {
    let file_path = r"input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents.split('\n') {
        let (mut left, mut right) = (0, 0);

        for from_left in line.chars() {
            if let Some(l) = from_left.to_digit(10) {
                left = l;
                break;
            }
        }

        for from_right in line.chars().rev() {
            if let Some(r) = from_right.to_digit(10) {
                right = r;
                break;
            }
        }
        sum += left * 10 + right;
    }

    println!("[PART1]: sum={sum}");
}

fn part2() {
    let file_path = r"input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents.split('\n') {
        let (mut left, mut right) = (0, 0);

        for (i, ch) in line.char_indices() {
            // one, two, three, four, five, six, seven, eight, nine
            match ch {
                'o' => {
                    if &line[i..(i + 3).min(line.len() - 1)] == "one" {
                        left = 1;
                    } else {
                        continue;
                    }
                }
                't' => {
                    if &line[i..(i + 3).min(line.len() - 1)] == "two" {
                        left = 2;
                    } else if &line[i..(i + 5).min(line.len() - 1)] == "three" {
                        left = 3;
                    } else {
                        continue;
                    }
                }
                'f' => {
                    if &line[i..(i + 4).min(line.len() - 1)] == "four" {
                        left = 4;
                    } else if &line[i..(i + 4).min(line.len() - 1)] == "five" {
                        left = 5;
                    } else {
                        continue;
                    }
                }
                's' => {
                    if &line[i..(i + 3).min(line.len() - 1)] == "six" {
                        left = 6;
                    } else if &line[i..(i + 5).min(line.len() - 1)] == "seven" {
                        left = 7;
                    } else {
                        continue;
                    }
                }
                'e' => {
                    if &line[i..(i + 5).min(line.len() - 1)] == "eight" {
                        left = 8;
                    } else {
                        continue;
                    }
                }
                'n' => {
                    if &line[i..(i + 4).min(line.len() - 1)] == "nine" {
                        left = 9;
                    } else {
                        continue;
                    }
                }
                other => {
                    if let Some(digit) = other.to_digit(10) {
                        left = digit;
                    } else {
                        continue;
                    }
                }
            };
            break;
        }

        for (i, ch) in line.char_indices().rev() {
            // one, two, three, four, five, six, seven, eight, nine
            match ch {
                'o' => {
                    if &line[i..(i + 3).min(line.len() - 1)] == "one" {
                        right = 1;
                    } else {
                        continue;
                    }
                }
                't' => {
                    if &line[i..(i + 3).min(line.len() - 1)] == "two" {
                        right = 2;
                    } else if &line[i..(i + 5).min(line.len() - 1)] == "three" {
                        right = 3;
                    } else {
                        continue;
                    }
                }
                'f' => {
                    if &line[i..(i + 4).min(line.len() - 1)] == "four" {
                        right = 4;
                    } else if &line[i..(i + 4).min(line.len() - 1)] == "five" {
                        right = 5;
                    } else {
                        continue;
                    }
                }
                's' => {
                    if &line[i..(i + 3).min(line.len() - 1)] == "six" {
                        right = 6;
                    } else if &line[i..(i + 5).min(line.len() - 1)] == "seven" {
                        right = 7;
                    } else {
                        continue;
                    }
                }
                'e' => {
                    if &line[i..(i + 5).min(line.len() - 1)] == "eight" {
                        right = 8;
                    } else {
                        continue;
                    }
                }
                'n' => {
                    if &line[i..(i + 4).min(line.len() - 1)] == "nine" {
                        right = 9;
                    } else {
                        continue;
                    }
                }
                other => {
                    if let Some(digit) = other.to_digit(10) {
                        right = digit;
                    } else {
                        continue;
                    }
                }
            };
            break;
        }

        println!("{:?}, left: {left}, right: {right}", line);

        sum += left * 10 + right;
    }

    println!("[PART2]: sum={sum}");
}

fn main() {
    part1();
    part2();
}
