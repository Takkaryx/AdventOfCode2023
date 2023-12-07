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

fn find_gears(map: EngineMap) -> Vec<Coords> {
    let mut locations = vec![];
    let mut y_loc = 0;
    let re: Regex = Regex::new(r"[\*]").unwrap();
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

fn determine_row_total(x_loc: usize, mat: (usize, usize, u64)) -> u64 {
    let start = mat.0;
    let end = mat.1;
    let val = mat.2;
    if start == end {
        if start + 1 == x_loc || start == x_loc + 1 {
            return val;
        }
    }
    if start <= x_loc {
        if x_loc <= (end + 1) {
            return val;
        }
    }
    if start > x_loc {
        if x_loc + 1 == start {
            return val;
        }
    }
    0
}
fn handle_row1(row: &mut Vec<char>, x_loc: u64) -> (u64, usize) {
    let mut total = 1;
    let re = Regex::new(r"\d+").unwrap();
    let text: String = row.clone().to_owned().into_iter().collect();
    let matches: Vec<(usize, usize, u64)> = re
        .find_iter(&text)
        .map(|mat| {
            let number: u64 = mat.as_str().parse().unwrap();
            (mat.start(), mat.end() - 1, number)
        })
        .collect();
    let mut num_touch = 0;
    for element in matches {
        let val = determine_row_total(x_loc as usize, element);
        if val == 0 {
            continue;
        }
        total += val;
        num_touch = num_touch + 1;
    }
    (total, num_touch)
}

fn handle_row2(row: &mut Vec<char>, x_loc: u64) -> (u64, usize) {
    let mut total = 1;
    let re = Regex::new(r"\d+").unwrap();
    let text: String = row.clone().to_owned().into_iter().collect();
    let matches: Vec<(usize, usize, u64)> = re
        .find_iter(&text)
        .map(|mat| {
            let number: u64 = mat.as_str().parse().unwrap();
            (mat.start(), mat.end() - 1, number)
        })
        .collect();
    let mut num_touch = 0;
    for element in matches {
        let val = determine_row_total(x_loc as usize, element);
        if val == 0 {
            continue;
        }
        total *= val;
        num_touch = num_touch + 1;
    }
    (total, num_touch)
}

fn find_numbers(map: &mut EngineMap, loc: Coords) -> Vec<u64> {
    let mut numbers: Vec<u64> = vec![];
    let y_loc: i32 = loc.1 as i32;
    for i in -1..=1i32 {
        if y_loc + i < 0 || y_loc + i > map.len() as i32 {
            continue;
        }
        let row_val = handle_row1(&mut map[(y_loc + i) as usize], loc.0);
        numbers.push(row_val.0);
    }
    numbers
}

fn find_gear_ratios(map: &mut EngineMap, loc: Coords) -> u64 {
    let mut numbers: Vec<u64> = vec![];
    let mut touches = 0;
    let y_loc: i32 = loc.1 as i32;
    for i in -1..=1i32 {
        if y_loc + i < 0 || y_loc + i > map.len() as i32 {
            continue;
        }
        let row_val = handle_row2(&mut map[(y_loc + i) as usize], loc.0);
        touches += row_val.1;
        numbers.push(row_val.0);
    }
    if touches == 2 {
        return numbers.into_iter().product();
    }
    0
}

fn solution1(map: EngineMap) -> u64 {
    let mut local_map = map.to_owned();
    let mut total: u64 = 0;
    let symbol_locations: Vec<Coords> = find_symbols(map.clone());
    for loc in symbol_locations {
        let part_sum: u64 = find_numbers(&mut local_map, loc).into_iter().sum();
        total = total + part_sum
    }
    total
}

fn solution2(map: EngineMap) -> u64 {
    let mut local_map = map.to_owned();
    let mut total: u64 = 0;
    let symbol_locations: Vec<Coords> = find_gears(map.clone());
    for loc in symbol_locations {
        let part_sum: u64 = find_gear_ratios(&mut local_map, loc);
        total = total + part_sum
    }
    total
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day3_input.txt").unwrap();
    let map = parse_input(contents.as_ref());
    let sol1: u64 = solution1(map.clone());
    let sol2: u64 = solution2(map.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
