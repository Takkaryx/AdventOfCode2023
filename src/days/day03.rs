use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

type EngineMap = Vec<Vec<char>>;
type Coords = (u64, u64);

fn parse_input(input: &str) -> EngineMap {
    let mut map: Vec<Vec<char>> = vec![];
    let mut counter = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        counter = counter + 1;
        map.push(chars);
    }
    map
}

///////////////////////////////////////////////////////////////////////////////

fn find_symbols(map: EngineMap) -> Vec<Coords> {
    let mut locations = vec![];
    let mut y_loc = 0;
    let re: Regex = Regex::new(r"[\p{P}\p{S}&&[^.]]").unwrap();
    for row in map {
        let text: String = row.into_iter().collect();
        let x_locs: Vec<usize> = re.find_iter(&text).map(|mat| mat.start()).collect();
        for x_loc in x_locs {
            let point: Coords = (x_loc as u64, y_loc);
            locations.push(point);
        }
        y_loc = y_loc + 1;
    }
    locations
}

fn handle_row(row: Vec<char>, x_loc: u64) -> u64 {
    let mut total = 0;
    let re = Regex::new(r"\d+").unwrap();
    let text: String = row.into_iter().collect();
    let matches: Vec<(usize, usize, u64)> = re
        .find_iter(&text)
        .map(|mat| {
            let number: u64 = mat.as_str().parse().unwrap();
            (mat.start(), mat.end(), number)
        })
        .collect();
    for element in matches {
        let start = element.0 as u64;
        let end = element.1 as u64;
        if start <= x_loc && x_loc <= end {
            total = element.2;
            break;
        } else if end == x_loc - 1 {
            total = total + element.2;
        } else if start == x_loc + 1 {
            total = total + element.2;
        }
    }
    total
}

fn find_numbers(map: EngineMap, loc: Coords) -> Vec<u64> {
    let mut numbers: Vec<u64> = vec![];
    let y_loc: i32 = loc.1 as i32;
    for i in -1..=1i32 {
        if y_loc + i < 0 || y_loc + i > map.len() as i32 {
            continue;
        }
        let row = handle_row(map[(y_loc + i) as usize].clone(), loc.0);
        numbers.push(row);
    }
    numbers
}

fn solution1(map: EngineMap) -> u64 {
    let mut total: u64 = 0;
    let symbol_locations: Vec<Coords> = find_symbols(map.clone());
    for loc in symbol_locations {
        let part_sum: u64 = find_numbers(map.clone(), loc).into_iter().sum();
        total = total + part_sum
    }
    total
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day3_input.txt").unwrap();
    let map = parse_input(contents.as_ref());
    let sol1: u64 = solution1(map);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
