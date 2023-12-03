use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;

type Throw = HashMap<String, u64>;
type Game = Vec<Throw>;

fn parse_color(input: &str) -> Throw {
    let mut temp: Throw = HashMap::new();
    let color_map: Vec<&str> = input.split(", ").collect();
    for color in color_map {
        let num: Vec<&str> = color.split(" ").collect();
        let color_name = num[1].to_string().to_lowercase();
        let num_cubes = num[0].parse::<u64>().unwrap();
        temp.insert(color_name, num_cubes);
    }
    temp
}

fn parse_throws(input: &str) -> Game {
    let mut temp: Game = vec![];
    let throws: Vec<&str> = input.split("; ").collect();
    for throw in throws {
        temp.push(parse_color(throw));
    }
    temp
}

fn parse_game(input: &str) -> Game {
    let throws: Vec<&str> = input.split(": ").collect();
    parse_throws(throws[1])
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut parsed_input: Vec<Game> = vec![];
    for line in input.lines() {
        parsed_input.push(parse_game(line));
    }
    parsed_input
}

///////////////////////////////////////////////////////////////////////////////
// only 12 red cubes, 13 green cubes, and 14 blue cubes
fn problem_1_answer(games: Vec<Game>) -> u64 {
    let mut allowed_game: Throw = HashMap::new();
    allowed_game.insert("red".to_string(), 12);
    allowed_game.insert("green".to_string(), 13);
    allowed_game.insert("blue".to_string(), 14);

    let mut answer: u64 = 0;
    let mut loop_counter = 0;
    for game in games {
        let mut possible_game = true;
        for throw in game {
            for color in throw {
                let color_name = color.0;
                let num_cubes = color.1;
                if let Some(exists) = allowed_game.get(&color_name) {
                    if num_cubes > exists.to_owned() {
                        possible_game = false;
                        break;
                    }
                }
            }
        }
        loop_counter = loop_counter + 1;
        if possible_game {
            answer = answer + loop_counter;
        }
    }
    answer
}

fn problem_2_answer(games: Vec<Game>) -> u64 {
    let mut power: u64 = 0;
    for game in games {
        let mut max_game: Throw = HashMap::new();
        max_game.insert("red".to_string(), 0);
        max_game.insert("green".to_string(), 0);
        max_game.insert("blue".to_string(), 0);
        for throw in game {
            for color in throw {
                let color_name = color.0;
                let num_cubes = color.1;
                if let Some(max_val) = max_game.get(&color_name) {
                    if num_cubes > max_val.to_owned() {
                        max_game.insert(color_name, num_cubes);
                    }
                }
            }
        }

        let mut multiple = 1;
        for max in max_game {
            multiple = max.1 * multiple;
        }
        power = power + multiple;
    }
    power
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = fs::read_to_string("input/day2_input.txt").unwrap();
    let game_info = parse_input(contents.as_ref());
    let sol1: u64 = problem_1_answer(game_info.clone());
    let sol2: u64 = problem_2_answer(game_info.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
