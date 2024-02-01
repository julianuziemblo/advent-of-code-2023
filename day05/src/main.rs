#[allow(unused)]
use std::fs;
use std::ops::Range;

use indicatif::ProgressIterator;

#[allow(unused)]
const TEST_INPUT: &str = r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    "#;

type Seed = usize;
type Location = usize;
type Offset = i64;

#[derive(Clone, Debug)]
struct Map {
    src_ranges: Vec<Range<Seed>>,
    offsets: Vec<Offset>,
}

fn parse_category(category: &str) -> Map {
    let lines = category.lines().collect::<Vec<&str>>();

    let ranges = &lines[1..];

    let mut src_ranges = Vec::<Range<Seed>>::with_capacity(ranges.len());
    let mut offsets = Vec::<Offset>::with_capacity(ranges.len());

    for range in ranges.iter() {
        let range = range.split_whitespace().collect::<Vec<&str>>();
        let src_range_start = range[1].trim().parse::<Seed>().unwrap();
        let dst_range_start = range[0].trim().parse::<Seed>().unwrap();
        let range_len = range[2].trim().parse::<Seed>().unwrap();

        src_ranges.push(src_range_start..src_range_start + range_len);
        offsets.push(dst_range_start as Offset - src_range_start as Offset);
    }

    Map {
        src_ranges,
        offsets,
    }
}

fn parse_input(input: String) -> (Vec<Seed>, Vec<Map>) {
    let mut maps = Vec::<Map>::new();

    let categories = input
        // .split("\n\n")                   // for test cases
        .split("\r\n\r") // for actual input
        .map(|category| category.trim())
        .collect::<Vec<&str>>();
    // println!("{categories:#?}");

    // println!("Seed: {:#?}", &categories[0].split_whitespace().collect::<Vec<&str>>()[1..]);

    let seeds = categories[0].split_whitespace().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|seed| seed.trim().parse::<Seed>().unwrap())
        .collect::<Vec<Seed>>();

    let categories = &categories[1..];

    for category in categories.iter() {
        let map = parse_category(category);
        maps.push(map);
    }

    // println!("Seeds: {seeds:#?}");
    // println!("Maps: {maps:#?}");

    (seeds, maps)
}

fn traverse(seed: Seed, maps: &Vec<Map>) -> Location {
    let mut seed = seed as Offset;

    for map in maps {
        let mut offset = 0;
        for (i, src_range) in map.src_ranges.iter().enumerate() {
            if src_range.contains(&(seed as Seed)) {
                offset = map.offsets[i];
            }
        }
        // println!("{:?} number for {:?} {:?} is {:?}", map.dst_category, map.src_category, seed, seed + offset);
        seed += offset;
    }

    seed as Seed
}

fn lowest_location(seeds: &[Seed], maps: &Vec<Map>) -> Location {
    let mut lowest = Seed::MAX;

    for seed in seeds.iter() {
        let seed_location = traverse(*seed, maps);
        // println!("Location for seed {seed}: {seed_location}");
        if seed_location < lowest {
            lowest = seed_location;
        }
    }

    lowest
}

fn get_ranges(seeds: &Vec<Seed>) -> Vec<(Range<Seed>, usize)> {
    let mut ranges = Vec::<(Range<Seed>, usize)>::with_capacity(seeds.len() / 2);
    for i in 0..(seeds.len() / 2) {
        let range_start_idx = 2 * i;
        let range_len_idx = range_start_idx + 1;

        ranges.push((
            seeds[range_start_idx]..seeds[range_start_idx] + seeds[range_len_idx],
            seeds[range_len_idx],
        ));
    }

    ranges
}

fn lowest_location_ranges(seeds: &Vec<Seed>, maps: &Vec<Map>) -> Location {
    let mut lowest = Seed::MAX;
    let mut sum = 0;

    let ranges = get_ranges(seeds);

    let ranges_len = ranges.len();

    for (i, range) in ranges.into_iter().enumerate() {
        // if doesnt work: maybe start a new thread for every range?
        let (range, range_len) = range;
        println!(
            "[{}/{ranges_len}] Calculating for seeds in range {range:#?} (length: {range_len:#?})",
            i + 1
        );
        for seed in range.progress() {
            let seed_location = traverse(seed, maps);
            // println!("Location for seed {seed}/{range_len}: {seed_location}");
            if seed_location < lowest {
                lowest = seed_location;
            }
        }
        sum += range_len;
    }

    println!("Seeds parsed: {sum}");

    lowest
}

fn main() {
    let filename = "input.txt";
    let input = fs::read_to_string(filename).unwrap();

    // let input = TEST_INPUT.to_owned();

    let (seeds, maps) = parse_input(input);

    println!("Lowest location: {}", lowest_location(&seeds, &maps));

    println!(
        "Lowest location in ranges of seeds: {}",
        lowest_location_ranges(&seeds, &maps)
    );
}
