use std::collections::HashSet;
#[allow(unused)]
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    yours: HashSet<u32>,
    wins: u32,
}

fn parse_contents(contents: String) -> Vec<Card> {
    let mut cards = Vec::<Card>::new();
    let contents = contents.trim();

    for line in contents.lines() {
        let split = line.trim().split(':').collect::<Vec<&str>>();

        let id = split.first().unwrap()[5..].trim().parse::<u32>().unwrap();

        let split2 = split
            .last()
            .unwrap()
            .trim()
            .split('|')
            .collect::<Vec<&str>>();

        let winning = split2
            .first()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let yours = split2
            .last()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let wins = get_num_of_wins(&winning, &yours);

        cards.push(Card {
            id,
            winning,
            yours,
            wins,
        });
    }

    cards
}

fn get_num_of_wins(winning: &HashSet<u32>, yours: &HashSet<u32>) -> u32 {
    winning.intersection(yours).collect::<Vec<_>>().len() as u32
}

fn get_points(card: &Card) -> u32 {
    match card.wins {
        0 => 0,
        i => 2u32.pow(i - 1),
    }
}

fn sum_points(cards: &Vec<Card>) -> u32 {
    let mut sum = 0;
    for card in cards {
        sum += get_points(card);
    }

    sum
}

fn iterate_cards(index: usize, cards: &Vec<Card>, total_cards: &mut Vec<Card>) {
    let n = cards[index].wins;
    for j in (index + 1)..=(index + n as usize) {
        if j < cards.len() {
            total_cards.push(cards[j].clone());
            iterate_cards(j, cards, total_cards);
        }
    }
}

fn get_total_cards(cards: &Vec<Card>) -> u32 {
    let mut total_cards = cards.clone();
    for (i, _) in cards.iter().enumerate() {
        iterate_cards(i, cards, &mut total_cards);
    }
    // println!("total_cards: {:?}", total_cards);

    total_cards.len() as u32
}

fn main() {
    #[allow(unused)]
    // let contents = r#"
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // "#.to_owned();
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).unwrap();

    let cards = parse_contents(contents);
    println!("[PART 1] sum = {}", sum_points(&cards));

    let now = Instant::now();
    println!("[PART 2] total cards = {}", get_total_cards(&cards));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
