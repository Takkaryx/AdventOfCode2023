use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

type MAP = (u64, u64, u64);

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<MAP>>) {
    let re = Regex::new(r"\d+").unwrap();
    let mut loop_counter = 0;
    let mut section = 0;
    let mut maps: Vec<Vec<MAP>> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    for line in input.lines() {
        loop_counter += 1;
        if loop_counter == 1 {
            seeds = line
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .into_iter()
                .map(|num| num.parse().unwrap())
                .collect();
            continue;
        }

        let mut set: MAP = (0, 0, 0);
        let mut current_set: Vec<u64> = Vec::new();
        if line.trim().is_empty() {
            if !current_set.is_empty() {
                current_set = Vec::new();
            }
            maps.push(Vec::new());
            section += 1;
        } else if let Some(_) = re.captures(line) {
            current_set = line
                .split(" ")
                .into_iter()
                .map(|num| num.parse().unwrap())
                .collect();
            set.0 = current_set[0];
            set.1 = current_set[1];
            set.2 = current_set[2];
        }

        if !current_set.is_empty() {
            maps[section - 1].push(set);
        }
    }
    (seeds, maps)
}

///////////////////////////////////////////////////////////////////////////////

fn check_map(seed: u64, trans: Vec<MAP>) -> u64 {
    for range in trans {
        let start = range.1;
        let end = range.1 + range.2 - 1;
        if start <= seed && seed <= end {
            let diff = seed - start;
            return range.0 + diff;
        }
    }
    seed
}

fn solution1(seeds: Vec<u64>, points: Vec<Vec<MAP>>) -> u64 {
    let mut answer = std::u64::MAX;
    for seed in seeds {
        let mut point = 0;
        for transaction in 0..7 {
            if transaction == 0 {
                point = seed;
            } else {
            }

            point = check_map(point, points[transaction].clone());
        }
        if point < answer {
            answer = point;
        }
    }
    answer
}

fn solution2(seeds: Vec<u64>, points: Vec<Vec<MAP>>) -> u64 {
    let mut answer = std::u64::MAX;
    let mut seed_ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        for y in seeds[i]..seeds[i + 1] {
            seed_ranges.push(y);
        }
    }
    for seed in seed_ranges {
        let mut point = 0;
        for transaction in 0..7 {
            if transaction == 0 {
                point = seed;
            } else {
            }

            point = check_map(point, points[transaction].clone());
        }
        if point < answer {
            answer = point;
        }
    }
    answer
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day5_input.txt").unwrap();
    let data = parse_input(&contents);
    let sol1: u64 = solution1(data.0.clone(), data.1.clone());
    let sol2: u64 = solution2(data.0, data.1);

    (Solution::from(sol1), Solution::from(sol2))
}
