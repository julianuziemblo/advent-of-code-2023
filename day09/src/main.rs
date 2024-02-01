use std::fs;

#[allow(unused)]
const TEST_FILE: &str = "test.txt";
#[allow(unused)]
const INPUT_FILE: &str = "input.txt";

type Number = i32;
type Sequence = Vec<Number>;

fn parse_input(input: &str) -> Vec<Vec<Number>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(
                    |num| 
                    num.trim()
                        .parse()
                        .unwrap()
                )
                .collect()
        })
        .collect()
}

fn is_zeroed(vector: &Sequence) -> bool {
    *vector == vec![0; vector.len()]
}

fn calc_diff(seq: &Sequence) -> Sequence {
    let mut diffs = vec![0; seq.len()-1];
    for (i, window) in seq.windows(2).enumerate() {
        diffs[i] = window[1] - window[0];
    }

    diffs
}

fn next_num(seq: &Sequence) -> Number {
    let diffs = calc_diff(seq);
    if is_zeroed(&diffs) || diffs.len() == 1 {
        return seq.last().unwrap() + diffs.last().unwrap();
    }

    seq.last().unwrap() + next_num(&diffs)
}

fn previous_num(seq: &Sequence) -> Number {
    let diffs = calc_diff(seq);
    if is_zeroed(&diffs) || diffs.len() == 1 {
        return seq[0] - diffs[0];
    }

    seq[0] - previous_num(&diffs) 
}

fn sum_predicate<F>(seqs: &Vec<Sequence>, predicate: &F) -> Number
where 
    F: Fn(&Sequence) -> Number + Clone
{
    let mut sum = 0;

    for seq in seqs {
        let next_number = predicate(seq);
        sum += next_number;
    }

    sum
}

fn main() {
    let filename = INPUT_FILE;
    let input = fs::read_to_string(filename)
        .unwrap_or_else(
            |err| 
                panic!("Couldn't read from file `{filename}`, cause: {:?}", err)
        );

    let seqs = parse_input(&input);
    // println!("lines: {:?}", lines);

    // let diffs = calc_diff(&lines[0]);
    // println!("line: {:?}\ndiff: {:?}",lines[0], diffs);

    // let next_number = next_num(&lines[0]);
    // println!("line: {:?}\nnext: {:?}",lines[0], next_number);

    println!("[PART 1] Sum of all next numbers: {:?}", sum_predicate(&seqs, &next_num));
    println!("[PART 2] Sum of all previous numbers: {:?}", sum_predicate(&seqs, &previous_num));
}
