use crate::{Solution, SolutionPair};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::read_to_string;

lazy_static! {
    static ref CARD_TO_VALUE: HashMap<char, u64> = {
        let mut map = HashMap::new();
        map.insert('2', 0);
        map.insert('3', 1);
        map.insert('4', 2);
        map.insert('5', 3);
        map.insert('6', 4);
        map.insert('7', 5);
        map.insert('8', 6);
        map.insert('9', 7);
        map.insert('T', 8);
        map.insert('J', 9);
        map.insert('Q', 10);
        map.insert('K', 11);
        map.insert('A', 12);
        map
    };
    static ref JCARD_TO_VALUE: HashMap<char, u64> = {
        let mut map = HashMap::new();
        map.insert('J', 0);
        map.insert('2', 1);
        map.insert('3', 2);
        map.insert('4', 3);
        map.insert('5', 4);
        map.insert('6', 5);
        map.insert('7', 6);
        map.insert('8', 7);
        map.insert('9', 8);
        map.insert('T', 9);
        map.insert('Q', 10);
        map.insert('K', 11);
        map.insert('A', 12);
        map
    };
}

#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
    Error,
}

#[derive(Debug)]
enum CardComp {
    Equal,
    Card1,
    Card2,
}

#[derive(Debug)]
struct CardSet {
    hand: HandType,
    cards: Vec<char>,
    points: u64,
}

fn find_hand_type(cards: &Vec<char>) -> HandType {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for &c in cards {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let max_matches = char_counts.values().cloned().max().unwrap_or(0);
    match max_matches {
        1 => {
            return HandType::HighCard;
        }
        2 => {
            let mut pairs = 0;
            for count in char_counts.values() {
                if *count == 2 {
                    pairs += 1;
                }
            }
            if pairs >= 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }
        3 => {
            if char_counts.values().cloned().min().unwrap_or(0) == 1 {
                return HandType::ThreeKind;
            } else {
                return HandType::FullHouse;
            }
        }
        4 => {
            return HandType::FourKind;
        }
        5 => {
            return HandType::FiveKind;
        }
        _ => {
            return HandType::Error;
        }
    }
}

fn jfind_hand_type(cards: &Vec<char>) -> HandType {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for &c in cards {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let j_matches = char_counts.get(&'J').unwrap_or(&0).clone();
    char_counts.remove(&'J');
    let max_matches = char_counts.values().cloned().max().unwrap_or(0) + j_matches;

    match max_matches {
        1 => {
            return HandType::HighCard;
        }
        2 => {
            let mut pairs = 0;
            for count in char_counts.values() {
                if *count == 2 {
                    pairs += 1;
                }
            }
            if pairs >= 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }
        3 => {
            if char_counts.values().cloned().min().unwrap_or(0) == 1 {
                return HandType::ThreeKind;
            } else {
                return HandType::FullHouse;
            }
        }
        4 => {
            return HandType::FourKind;
        }
        5 => {
            return HandType::FiveKind;
        }
        _ => {
            println!(
                " array: {:?}, max_matches {}, j_matches {}",
                char_counts, max_matches, j_matches
            );
            return HandType::Error;
        }
    }
}

fn parse_input(input: &str, joker: bool) -> Vec<CardSet> {
    let mut full_set = Vec::new();
    for line in input.lines() {
        let mut splits = line.split(" ");
        let cards: Vec<char> = splits.next().unwrap().chars().collect();
        let points: u64 = splits.next().unwrap().parse().unwrap();
        let hand: HandType;
        if !joker {
            hand = find_hand_type(cards.as_ref());
        } else {
            hand = jfind_hand_type(cards.as_ref());
        }
        let set = CardSet {
            hand,
            cards,
            points,
        };
        // println!("{:?}", set);
        full_set.push(set);
    }
    full_set
}

///////////////////////////////////////////////////////////////////////////////

fn compare_cards(cardset1: Vec<char>, cardset2: Vec<char>, joker: bool) -> CardComp {
    for x in 0..cardset1.len() {
        let val1: Option<&u64>;
        let val2: Option<&u64>;
        if !joker {
            val1 = CARD_TO_VALUE.get(&cardset1[x]);
            val2 = CARD_TO_VALUE.get(&cardset2[x]);
        } else {
            val1 = JCARD_TO_VALUE.get(&cardset1[x]);
            val2 = JCARD_TO_VALUE.get(&cardset2[x]);
        }
        if val1 > val2 {
            return CardComp::Card1;
        } else if val2 > val1 {
            return CardComp::Card2;
        }
    }
    CardComp::Equal
}

fn insert_card(card1: CardSet, insert_set: &mut Vec<CardSet>, joker: bool) {
    for (index, card) in insert_set.iter().enumerate() {
        let compare = compare_cards(card.cards.clone(), card1.cards.clone(), joker);
        match compare {
            CardComp::Card1 => {
                insert_set.insert(index, card1);
                return;
            }
            CardComp::Card2 => {
                continue;
            }
            CardComp::Equal => {
                insert_set.insert(index + 1, card1);
                return;
            }
        }
    }
    insert_set.push(card1);
}

fn sort_cards(set: Vec<CardSet>, joker: bool) -> Vec<CardSet> {
    let mut sorted_cards: Vec<CardSet> = Vec::new();
    for card in set {
        if sorted_cards.is_empty() {
            sorted_cards.push(card);
            continue;
        }
        insert_card(card, &mut sorted_cards, joker);
    }
    sorted_cards
}

fn solution1(set: Vec<CardSet>) -> u64 {
    let mut highcard: Vec<CardSet> = vec![];
    let mut onepairs: Vec<CardSet> = vec![];
    let mut twopairs: Vec<CardSet> = vec![];
    let mut threekind: Vec<CardSet> = vec![];
    let mut fullhouse: Vec<CardSet> = vec![];
    let mut fourkind: Vec<CardSet> = vec![];
    let mut fivekind: Vec<CardSet> = vec![];

    for card in set {
        match card.hand {
            HandType::HighCard => {
                highcard.push(card);
            }
            HandType::OnePair => {
                onepairs.push(card);
            }
            HandType::TwoPair => {
                twopairs.push(card);
            }
            HandType::ThreeKind => {
                threekind.push(card);
            }
            HandType::FullHouse => {
                fullhouse.push(card);
            }
            HandType::FourKind => {
                fourkind.push(card);
            }
            HandType::FiveKind => {
                fivekind.push(card);
            }
            HandType::Error => {
                println!("invalid HandType found?");
                println!("{:?}", card);
            }
        }
    }
    let mut complete_list: Vec<CardSet> = Vec::new();
    let mut answer = 0;
    complete_list.append(&mut sort_cards(highcard, false));
    complete_list.append(&mut sort_cards(onepairs, false));
    complete_list.append(&mut sort_cards(twopairs, false));
    complete_list.append(&mut sort_cards(threekind, false));
    complete_list.append(&mut sort_cards(fullhouse, false));
    complete_list.append(&mut sort_cards(fourkind, false));
    complete_list.append(&mut sort_cards(fivekind, false));
    for (index, element) in complete_list.iter().enumerate() {
        answer += (index as u64 + 1) * element.points;
    }
    answer
}

fn solution2(set: Vec<CardSet>) -> u64 {
    let mut highcard: Vec<CardSet> = vec![];
    let mut onepairs: Vec<CardSet> = vec![];
    let mut twopairs: Vec<CardSet> = vec![];
    let mut threekind: Vec<CardSet> = vec![];
    let mut fullhouse: Vec<CardSet> = vec![];
    let mut fourkind: Vec<CardSet> = vec![];
    let mut fivekind: Vec<CardSet> = vec![];

    for card in set {
        match card.hand {
            HandType::HighCard => {
                highcard.push(card);
            }
            HandType::OnePair => {
                onepairs.push(card);
            }
            HandType::TwoPair => {
                twopairs.push(card);
            }
            HandType::ThreeKind => {
                threekind.push(card);
            }
            HandType::FullHouse => {
                fullhouse.push(card);
            }
            HandType::FourKind => {
                fourkind.push(card);
            }
            HandType::FiveKind => {
                fivekind.push(card);
            }
            HandType::Error => {
                println!("invalid HandType found?");
                println!("{:?}", card);
            }
        }
    }
    let mut complete_list: Vec<CardSet> = Vec::new();
    let mut answer = 0;
    complete_list.append(&mut sort_cards(highcard, true));
    complete_list.append(&mut sort_cards(onepairs, true));
    complete_list.append(&mut sort_cards(twopairs, true));
    complete_list.append(&mut sort_cards(threekind, true));
    complete_list.append(&mut sort_cards(fullhouse, true));
    complete_list.append(&mut sort_cards(fourkind, true));
    complete_list.append(&mut sort_cards(fivekind, true));
    for (index, element) in complete_list.iter().enumerate() {
        answer += (index as u64 + 1) * element.points;
    }
    answer
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day7_input.txt").unwrap();
    let card_set = parse_input(&contents, false);
    let jcard_set = parse_input(&contents, true);
    let sol1: u64 = solution1(card_set);
    let sol2: u64 = solution2(jcard_set);

    (Solution::from(sol1), Solution::from(sol2))
}
