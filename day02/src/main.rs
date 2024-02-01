use std::fs;

#[derive(Debug)]
struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl GameSet {
    fn new() -> GameSet {
        GameSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

fn parse_line(line: &str) -> Option<Game> {
    // line format:
    // Game N: X blue, X red; 1 red, 2 green, 6 blue; 2 green
    if line.trim().is_empty() {
        return None;
    }

    let line = line.trim();

    // strategy:
    // 1: split on `:`
    // 2: split the 2nd half on `;` and strip
    let split = line.split(':').collect::<Vec<&str>>();
    let id = split[0][5..].parse::<i32>().unwrap_or(0);
    let sets = split[1]
        .split(';')
        .map(|e| {
            let mut game = GameSet::new();
            e.trim().split(", ").for_each(|instruction| {
                let s = instruction.split_whitespace().collect::<Vec<&str>>();
                let (count, color) = (s[0].parse::<i32>().unwrap_or(0), s[1]);
                match color {
                    "blue" => game.blue += count,
                    "red" => game.red += count,
                    _ => game.green += count,
                };
            });
            game
        })
        .collect::<Vec<GameSet>>();

    Some(Game { id, sets })
}

fn parse_contents(contents: String) -> Vec<Game> {
    let mut games = Vec::<Game>::with_capacity(contents.lines().count());

    for line in contents.lines() {
        if let Some(game) = parse_line(line) {
            games.push(game);
        }
    }

    games
}

fn is_possible(game_set: &GameSet) -> bool {
    game_set.red <= 12 && game_set.green <= 13 && game_set.blue <= 14
}

fn sum_possible_game_ids(games: &[Game]) -> i32 {
    let mut sum = 0;
    for game in games.iter() {
        let mut should_add = true;
        for set in game.sets.iter() {
            if !is_possible(set) {
                should_add = false;
                break;
            }
        }
        if should_add {
            sum += game.id;
        }
    }

    sum
}

fn min_cubes(game: &Game) -> GameSet {
    let mut res_set = GameSet::new();
    for set in game.sets.iter() {
        if set.red > res_set.red {
            res_set.red = set.red;
        }
        if set.blue > res_set.blue {
            res_set.blue = set.blue;
        }
        if set.green > res_set.green {
            res_set.green = set.green;
        }
    }

    res_set
}

fn game_power(game_set: &GameSet) -> i32 {
    game_set.red * game_set.green * game_set.blue
}

fn sum_powers(games: &[Game]) -> i32 {
    let mut sum = 0;
    for game in games.iter() {
        sum += game_power(&min_cubes(game));
    }

    sum
}

fn main() {
    // let contents = r#"
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    // "#;

    let filename = "input.txt";
    let contents = fs::read_to_string(filename).unwrap_or_else(|e| {
        panic!("File {filename} could not be opened because of an unexpected error: {e:?}.")
    });

    let games = parse_contents(contents);
    println!("Possible game IDs sum to {}", sum_possible_game_ids(&games));
    println!("Sum of all game powers is {}", sum_powers(&games));
}
