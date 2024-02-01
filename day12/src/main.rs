use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{collections::HashMap, fs};

#[allow(unused)]
const TEST_PATH: &str = "test.txt";
#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

const SPRINGS_OPTIONS: [char; 2] = ['.', '#'];
const TIMES: usize = 5;

#[derive(Clone, Debug)]
struct Row {
    row: String,
    values: Vec<u8>,
}

impl Row {
    fn row_chars(&self) -> Vec<char> {
        return self.row.chars().collect();
    }
}

fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let row = split[0];
            let values = split[1]
                .split(',')
                .map(|e| e.parse::<u8>().unwrap())
                .collect();
            Row {
                row: String::from(row),
                values,
            }
        })
        .collect()
}

fn check_row(row: &str, values: &Vec<u8>) -> bool {
    let mut is_accumulating = false;
    let mut count = 0;
    let mut nums = vec![];

    for ch in row.chars() {
        if ch == '#' {
            count += 1;
            is_accumulating = true;
        } else {
            if is_accumulating {
                nums.push(count);
                is_accumulating = !is_accumulating;
            }
            count = 0;
        }
    }
    if count != 0 {
        nums.push(count);
    }

    // println!("nums: {nums:?}, values: {values:?}");
    nums == *values
}

fn get_combinations_magic(row_len: usize) -> Vec<String> {
    (2..row_len).fold(
        SPRINGS_OPTIONS
            .iter()
            .cartesian_product(SPRINGS_OPTIONS.iter())
            .map(|(&a, &b)| format!("{}{}", a, b))
            .collect(),
        |acc, _| {
            acc.into_iter()
                .cartesian_product(SPRINGS_OPTIONS.iter()) // iterative cartesian of cartesians!
                .map(|(a, b)| format!("{}{}", a, b))
                .collect()
        },
    )
}

fn count_question_marks(row: &Row) -> usize {
    row.row.chars().filter(|e| *e == '?').count()
}

fn insert_combination(row: &Row, combination: &str) -> String {
    let mut accum = String::from("");
    let mut count = 0;
    let combination: Vec<char> = combination.chars().collect();

    for ch in row.row.chars() {
        if ch == '?' {
            accum.push(combination[count]);
            count += 1;
        } else {
            accum.push(ch);
        }
    }

    accum
}

fn num_of_combinations(rows: &[Row]) -> usize {
    rows.iter()
        .progress()
        .map(|row| {
            get_combinations_magic(count_question_marks(row))
                .iter()
                .filter(|&combination| {
                    check_row(&insert_combination(row, combination), &row.values)
                })
                .count()
        })
        .sum()
}

fn repeat_with_separator(string: &str, times: usize, sep: &str) -> String {
    let mut accum = String::new();

    for i in 0..times {
        accum.push_str(string);
        if i != times - 1 {
            accum.push_str(sep);
        }
    }

    accum
}

fn unfold_springs(rows: &[Row], times: usize) -> Vec<Row> {
    let mut unfolded_rows = vec![];

    for row in rows {
        unfolded_rows.push(Row {
            row: repeat_with_separator(&row.row, times, "?"),
            values: row.values.repeat(times),
        })
    }

    unfolded_rows
}

fn get_row_combinations_dynamic(
    row: &Vec<char>,
    blocks: &Vec<u8>,
    row_i: usize,
    blocks_i: usize,
    current_block: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let cache_key = (row_i, blocks_i, current_block);
    if let Some(&cache_entry) = cache.get(&cache_key) {
        return cache_entry;
    }

    if row_i == row.len() {
        if blocks_i == blocks.len() && current_block == 0
            || blocks_i == blocks.len() - 1 && blocks[blocks_i] as usize == current_block
        {
            return 1;
        } else {
            return 0;
        }
    }

    let mut partial_sum = 0;

    for c in SPRINGS_OPTIONS {
        if row[row_i] == c || row[row_i] == '?' {
            if c == '.' && current_block == 0 {
                partial_sum +=
                    get_row_combinations_dynamic(row, blocks, row_i + 1, blocks_i, 0, cache);
            } else if c == '.'
                && current_block > 0
                && blocks_i < blocks.len()
                && blocks[blocks_i] as usize == current_block
            {
                partial_sum +=
                    get_row_combinations_dynamic(row, blocks, row_i + 1, blocks_i + 1, 0, cache);
            } else if c == '#' {
                partial_sum += get_row_combinations_dynamic(
                    row,
                    blocks,
                    row_i + 1,
                    blocks_i,
                    current_block + 1,
                    cache,
                );
            }
        }
    }

    cache.insert(cache_key, partial_sum);

    partial_sum
}

fn num_of_combinations_dynamic(rows: &[Row]) -> usize {
    let mut dp = HashMap::new();
    let mut sum = 0;

    #[allow(unused_variables)]
    #[allow(clippy::unused_enumerate_index)]
    for (i, row) in rows.iter().progress().enumerate() {
        // println!("Row: {}, \nvalues: {:?}", row.row, row.values);
        let score = get_row_combinations_dynamic(&row.row_chars(), &row.values, 0, 0, 0, &mut dp);
        // println!("Sum in {} iteration: {}", i, sum);
        // println!("Score in iteration {}: {}", i, score);
        sum += score;
        dp.clear();
    }

    sum
}

fn main() {
    let path = INPUT_PATH;
    let input = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("File {} couldn't be read because of {:?}", path, err));

    let rows = parse_input(&input);
    println!(
        "[PART 1] Number of combinations (bruteforce): {}",
        num_of_combinations(&rows)
    );
    println!(
        "[PART 1] Number of combinations (dynamic): {}",
        num_of_combinations(&rows)
    );

    let unfolded_rows = unfold_springs(&rows, TIMES);
    // println!("[PART 2] Unfolded springs: \n{:#?}", unfolded_rows);
    println!(
        "[PART 2] Number of combinations (dynamic): {}",
        num_of_combinations_dynamic(&unfolded_rows)
    );
}
