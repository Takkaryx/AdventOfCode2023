use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn solution1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();
    let distance: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut pos = 0;
    let mut answer = 1;
    for run in times {
        let mut matches = 0;
        for x in 1..run {
            let dist = (run - x) * x;
            if dist > distance[pos] {
                matches = matches + 1;
            }
        }
        answer = answer * matches;
        pos = pos + 1;
    }
    answer
}

fn solution2(input: &str) -> u64 {
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();
    let distance: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut pos = 0;
    let mut answer = 1;
    for run in times {
        let mut matches = 0;
        for x in 1..run {
            let dist = (run - x) * x;
            if dist > distance[pos] {
                matches = matches + 1;
            }
        }
        answer = answer * matches;
        pos = pos + 1;
    }
    answer
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day6_input.txt").unwrap();
    let sol1: u64 = solution1(contents.as_ref());
    let sol2: u64 = solution2(contents.as_ref());

    (Solution::from(sol1), Solution::from(sol2))
}
