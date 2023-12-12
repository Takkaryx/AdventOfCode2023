use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut data: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let row = line
            .split(" ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        data.push(row);
    }
    data
}

///////////////////////////////////////////////////////////////////////////////

fn solve_row_right(row: Vec<i64>) -> i64 {
    let mut subrow = Vec::new();
    let mut pre_pos = 0;

    for num in row.iter().skip(1) {
        let diff = num - row[pre_pos];
        pre_pos += 1;
        subrow.push(diff);
    }
    let last_diff = row[row.len() - 1];
    if subrow.clone().into_iter().all(|num| num == 0) {
        return last_diff;
    }

    let val = solve_row_right(subrow) + last_diff;
    val
}

fn solution1(data: &Vec<Vec<i64>>) -> i64 {
    let mut sol = 0;
    let mydata = data.to_owned();
    for row in mydata {
        let val = solve_row_right(row);
        sol += val;
    }
    sol
}

fn solve_row_left(row: Vec<i64>) -> i64 {
    let mut subrow = Vec::new();
    let mut pre_pos = 0;

    for num in row.iter().skip(1) {
        let diff = num - row[pre_pos];
        pre_pos += 1;
        subrow.push(diff);
    }
    let last_diff = row[0];
    if subrow.clone().into_iter().all(|num| num == 0) {
        return last_diff;
    }

    let val = last_diff - solve_row_left(subrow);
    val
}

fn solution2(data: &Vec<Vec<i64>>) -> i64 {
    let mut sol = 0;
    let mydata = data.to_owned();
    for row in mydata {
        let val = solve_row_left(row);
        sol += val;
    }
    sol
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day9_input.txt").unwrap();
    let data = parse_input(&contents);
    let sol1: i64 = solution1(&data);
    let sol2: i64 = solution2(&data);

    (Solution::from(sol1), Solution::from(sol2))
}
