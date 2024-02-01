use std::{cmp::Ordering, fs};

#[allow(unused)]
const TEST_FILE: &str = "test.txt";
#[allow(unused)]
const INPUT_FILE: &str = "day07/input.txt";

type Card = char;
type Strength = u8;

const CARDS_LEN: usize = 13;
const HAND_LEN: usize = 5;
const CARDS: [Card; CARDS_LEN] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const JOKER_INDEX: usize = 0;
const JOKER: char = CARDS[JOKER_INDEX];

trait CardStrength {
    fn strength(self) -> Strength;
}

impl CardStrength for Card {
    fn strength(self) -> Strength {
        CARDS.into_iter().position(|c| c == self).unwrap_or(0) as Strength
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum CardKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardKind {
    fn cmp(&self, other: &CardKind) -> Ordering {
        #[allow(clippy::comparison_chain)]
        if self > other {
            return Ordering::Greater;
        } else if self < other {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

#[derive(Clone, Debug)]
struct Hand {
    cards: [Card; HAND_LEN],
    kind: CardKind,
    bid: u32,
}

fn histogram(cards: &[Card; HAND_LEN]) -> [Strength; CARDS_LEN] {
    let mut histogram: [Strength; CARDS_LEN] = [0; CARDS_LEN];

    for card in cards {
        histogram[card.strength() as usize] += 1;
    }

    // println!("Histogram for cards {cards:?}: {histogram:?}");

    histogram
}

fn get_cards_and_bid(line: &str) -> ([Card; HAND_LEN], u32) {
    let mut cards = ['0'; HAND_LEN];
    let tokens = line.split_whitespace().collect::<Vec<_>>();

    let bid = tokens[1].parse().unwrap();

    let chars = tokens[0].chars().collect::<Vec<char>>();

    #[allow(clippy::manual_memcpy)]
    for i in 0..HAND_LEN {
        cards[i] = chars[i];
    }

    (cards, bid)
}

/// XDXDDDDXDXDXDXDDD
fn get_hand(line: &str) -> Hand {
    let (cards, bid) = get_cards_and_bid(line);
    let histogram = histogram(&cards);

    let (mut pairs, mut seen_three) = (0, false);

    for (i, times) in histogram.iter().enumerate() {
        let current_card = CARDS[i];
        match times {
            // [PPPPP] or [JJJJJ] => FiveOfAKind 1 or FiveOfAKind 6
            5 => {
                return Hand {
                    cards,
                    kind: CardKind::FiveOfAKind,
                    bid,
                }
            }
            4 => {
                let remaining_card = CARDS[histogram.iter().position(|&card| card == 1).unwrap()];

                // [PPPP][J] or [P][JJJJ] => FiveOfAKind 2 or FiveOfAKind 5
                if current_card == JOKER || remaining_card == JOKER {
                    return Hand {
                        cards,
                        kind: CardKind::FiveOfAKind,
                        bid,
                    };
                }
                // [PPPP][Q] => FourOfAKind 1
                return Hand {
                    cards,
                    kind: CardKind::FourOfAKind,
                    bid,
                };
            }
            3 => seen_three = true,
            2 => pairs += 1,
            _ => continue,
        };
    }

    let joker_times = histogram[JOKER_INDEX];

    // 1 three
    if seen_three {
        // 1 pair
        if pairs == 1 {
            // jokers are the three or the pair
            if joker_times == 3 || joker_times == 2 {
                // [PP][JJJ] or [PPP][JJ] => FiveOfAKind 3 or FiveOfAKind 4
                return Hand {
                    cards,
                    kind: CardKind::FiveOfAKind,
                    bid,
                };
            }

            // 0 jokers
            // [PPP][QQ] => FullHouse 1
            return Hand {
                cards,
                kind: CardKind::FullHouse,
                bid,
            };
        }

        // 0 pairs, 3 jokers
        if joker_times == 3 {
            // [P][JJJ][Q] => FourOfAKind 4
            return Hand {
                cards,
                kind: CardKind::FourOfAKind,
                bid,
            };
        }

        // 0 pairs, 1 joker
        if joker_times == 1 {
            // [PPP][J][Q] => FourOfAKind 2
            return Hand {
                cards,
                kind: CardKind::FourOfAKind,
                bid,
            };
        }

        // 0 pairs and 0 jokers
        // [PPP][Q][R] => ThreeOfAKind 1
        return Hand {
            cards,
            kind: CardKind::ThreeOfAKind,
            bid,
        };
    }

    // 0 threes
    // 2 pairs of 2
    if pairs == 2 {
        // 2 jokers are 1 of the pairs
        if joker_times == 2 {
            // [PP][JJ][Q] => FourOfAKind 3
            return Hand {
                cards,
                kind: CardKind::FourOfAKind,
                bid,
            };
        }

        // 1 joker apart from the pairs
        if joker_times == 1 {
            // [PP][J][QQ] => FullHouse 2
            return Hand {
                cards,
                kind: CardKind::FullHouse,
                bid,
            };
        }

        // 0 jokers
        // [PP][QQ][R] => TwoPair 1
        return Hand {
            cards,
            kind: CardKind::TwoPair,
            bid,
        };
    }

    // 0 threes
    // only 1 pair
    if pairs == 1 {
        // 2 jokers as the pair or 1 joker apart from the pair
        if joker_times == 1 || joker_times == 2 {
            // [P][JJ][Q][R] or [PP][J][Q][R] => ThreeOfAKind 2 or ThreeOfAKind 3
            return Hand {
                cards,
                kind: CardKind::ThreeOfAKind,
                bid,
            };
        }

        // 0 jokers
        // [PP][Q][R][S] => OnePair 1
        return Hand {
            cards,
            kind: CardKind::OnePair,
            bid,
        };
    }

    if joker_times == 1 {
        return Hand {
            cards,
            kind: CardKind::OnePair,
            bid,
        };
    }

    // 0 threes
    // 0 pairs
    // [P][Q][R][S][T] => HighCard 1
    Hand {
        cards,
        kind: CardKind::HighCard,
        bid,
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::<Hand>::with_capacity(input.lines().count());

    for line in input.lines() {
        let hand = get_hand(line.trim());

        hands.push(hand);
    }

    hands
}

fn rank_hands(hands: &[Hand]) -> Vec<Hand> {
    let mut hands = hands.to_vec().clone();

    hands.sort_by(|current, next| {
        let comparison = current.kind.cmp(&next.kind);
        if comparison == Ordering::Equal {
            for (curr_char, next_char) in current.cards.iter().zip(next.cards.iter()) {
                #[allow(clippy::comparison_chain)]
                if curr_char.strength() > next_char.strength() {
                    return Ordering::Greater;
                } else if curr_char.strength() < next_char.strength() {
                    return Ordering::Less;
                }
            }
            // return Ordering::Equal;
        } else if comparison == Ordering::Greater {
            return Ordering::Greater;
        }

        Ordering::Less
    });

    hands
}

fn total_winnings(hands: &[Hand]) -> u32 {
    let ranked_hands = rank_hands(hands);
    println!("Ranked: {:#?}", ranked_hands);

    let mut sum = 0;

    for (i, hand) in ranked_hands.iter().enumerate() {
        sum += (i + 1) as u32 * hand.bid;
    }

    sum
}

fn main() {
    let filepath = INPUT_FILE;
    let input = fs::read_to_string(filepath).unwrap_or_else(|e| {
        panic!("Couldn't read file {filepath} because of an unexpected error: {e:?}")
    });

    let hands = parse_input(&input);
    // println!("parsed: {:#?}", hands);
    // println!("Ranked: {:#?}", rank_hands(&hands));
    println!("total winnings: {:#?}", total_winnings(&hands));
}
