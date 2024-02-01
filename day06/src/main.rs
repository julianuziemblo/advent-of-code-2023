use std::fs;

#[allow(unused)]
const TEST_FILE: &str = "test.txt";
#[allow(unused)]
const INPUT_FILE_PART1: &str = "input.txt";
#[allow(unused)]
const INPUT_FILE_PART2: &str = "input2.txt";

type Unit = u64;

#[derive(Clone, Debug)]
struct Race {
    time: Unit,
    record_distance: Unit,
}

fn parse_line(line: &str) -> Vec<Unit> {
    Vec::from(
        &line
            .split_whitespace()
            .map(|token| token.trim().parse::<Unit>().unwrap_or(0))
            .collect::<Vec<Unit>>()[1..],
    )
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<&str>>();

    let times = parse_line(lines[0]);
    let distances = parse_line(lines[1]);

    let mut races = Vec::with_capacity(times.len());
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        races.push(Race {
            time,
            record_distance: distance,
        });
    }

    races
}

fn beats_record(time: Unit, race: &Race) -> bool {
    let speed = time;
    let distance = (race.time - time) * speed;

    // println!("speed: {}, distance traveled: {}, race distance: {}, beats? {}", speed, distance, race.record_distance, distance > race.record_distance);

    distance > race.record_distance
}

fn multiply_record_beating_ways(races: &Vec<Race>) -> Unit {
    let mut mul: Unit = 1;

    for race in races {
        let (mut left, mut right) = (1, race.time - 1);
        for time in left..=right {
            if beats_record(time, race) {
                left = time;
                break;
            }
        }

        for time in (left..=right).rev() {
            if beats_record(time, race) {
                right = time;
                break;
            }
        }

        // println!("Race: {race:?} least time: {}, most time: {}, multiplier: {}", left, right, (right - left).max(1));

        mul *= (right - left + 1).max(1);
    }

    mul
}

fn main() {
    let filepath = INPUT_FILE_PART2;
    let input = fs::read_to_string(filepath).unwrap_or_else(|e| {
        panic!("File `{filepath}` could not be opened because of an unexpected error {e:?}.")
    });

    let races = parse_input(&input);
    // println!("Races: {races:#?}");
    let part1 = multiply_record_beating_ways(&races);
    println!("{part1}");
}
