use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn parse_line(input: &str) -> (HashSet<u64>, HashSet<u64>) {
    let string_set: Vec<&str> = input.split("|").collect();
    let before_pipe = string_set[0].split(":").nth(1).unwrap().trim();
    let after_pipe = string_set[1].trim();
    let set1: HashSet<u64> = before_pipe
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let set2: HashSet<u64> = after_pipe
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    (set1, set2)
}

fn solution1(input: String) -> u64 {
    let mut total_score = 0;
    for line in input.lines() {
        let data_sets = parse_line(line);
        let common_elements: HashSet<u64> =
            data_sets.0.intersection(&data_sets.1).cloned().collect();
        total_score += 1 << (common_elements.len() - 1)
    }
    total_score
}

fn solution2(input: String) -> u64 {
    let mut total_score = 0;
    let mut card_num = 0;
    let mut copies: HashMap<u64, u64> = HashMap::with_capacity(input.lines().count());
    for index in 0..input.lines().count() {
        copies.insert(index as u64, 1);
    }
    for line in input.lines() {
        let data_sets = parse_line(line);
        let common_elements: HashSet<u64> =
            data_sets.0.intersection(&data_sets.1).cloned().collect();

        let winning_nums = common_elements.len() as u64;
        let number_of_cards = copies.get(&card_num).unwrap().to_owned();

        for _ in 0..number_of_cards {
            for wins in 1..winning_nums + 1 {
                if let Some(value) = copies.get_mut(&(wins + card_num)) {
                    *value += 1;
                }
            }
        }
        card_num += 1;
        total_score += number_of_cards;
    }
    total_score
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day4_input.txt").unwrap();
    let sol1: u64 = solution1(contents.clone());
    let sol2: u64 = solution2(contents.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
