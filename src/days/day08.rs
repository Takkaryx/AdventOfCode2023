use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum Seq {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Coords {
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (Vec<Seq>, HashMap<String, Coords>) {
    let mut steps: Vec<Seq> = Vec::new();
    let mut map: HashMap<String, Coords> = HashMap::new();
    let mut lines = input.lines();
    let sequence_str: Vec<char> = lines.next().unwrap().chars().collect();
    for char in sequence_str {
        match char {
            'R' => {
                steps.push(Seq::Right);
            }
            'L' => {
                steps.push(Seq::Left);
            }
            _ => {
                println!("invalid char found {}", char);
            }
        }
    }

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut split1 = line.split("=");
        let loc = split1.next().unwrap().trim().to_string();
        let mut split2 = split1.next().unwrap().trim().split(", ");
        let left = split2.next().unwrap().to_string();
        let right = split2.next().unwrap().to_string();
        let coord = Coords { left, right };
        map.insert(loc, coord);
    }

    (steps, map)
}

///////////////////////////////////////////////////////////////////////////////i

fn take_step(choice: Seq, options: &Coords) -> String {
    match choice {
        Seq::Left => {
            return options.left.clone();
        }
        Seq::Right => {
            return options.right.clone();
        }
    }
}

fn solution1(data: &(Vec<Seq>, HashMap<String, Coords>)) -> u64 {
    let mut loc = "AAA".to_string();
    let steps = data.0.clone();
    let map = data.1.clone();
    let mut answer = 0;
    let mut current_step = 0;
    while loc != "ZZZ" {
        let curr_options = map.get(&loc).unwrap();
        let curr_step = steps[current_step];
        loc = take_step(curr_step, curr_options);
        current_step += 1;
        if current_step >= steps.len() {
            current_step = 0;
        }
        answer += 1;
    }
    answer
}

// fn solution2_proper(data: &(Vec<Seq>, HashMap<String, Coords>)) -> u64 {
//     let steps = data.0.clone();
//     let map = data.1.clone();
//     let temp: Vec<&String> = map.keys().filter(|key| key.ends_with('A')).collect();
//     let mut locs: Vec<String> = temp.iter().map(|s| s.to_string()).collect();
//     let mut answer = 0;
//     let mut current_step = 0;

//     loop {
//         let mut new_locs: Vec<String> = Vec::new();
//         for loc in &locs {
//             let curr_options = map.get(loc).unwrap();
//             let curr_step = steps[current_step];
//             new_locs.push(take_step(curr_step, curr_options));
//         }
//         answer += 1;

//         if new_locs.iter().all(|s| s.ends_with('Z')) {
//             println!("{:?}", new_locs);
//             break;
//         }

//         locs.splice(0..locs.len(), new_locs);
//         current_step += 1;
//         if current_step >= steps.len() {
//             current_step = 0;
//         }
//     }
//     answer
// }

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(numbers: &[u64]) -> Option<u64> {
    if numbers.is_empty() {
        return None;
    }

    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = (result * num) / gcd(result, num);
    }
    Some(result)
}

fn solution2_lcm(data: &(Vec<Seq>, HashMap<String, Coords>)) -> u64 {
    let map = data.1.clone();
    let temp: Vec<&String> = map.keys().filter(|key| key.ends_with('A')).collect();
    let locs: Vec<String> = temp.iter().map(|s| s.to_string()).collect();

    let mut steps = data.0.iter().cycle();
    let mut multiples: Vec<u64> = vec![];
    for loc in locs {
        let mut current_node_name = loc.clone();
        let mut answer = 0;
        while !current_node_name.ends_with('Z') {
            let choice = steps.next().unwrap();
            let next_node_name: String = match choice {
                Seq::Left => map.get(&current_node_name).unwrap().left.clone(),
                Seq::Right => map.get(&current_node_name).unwrap().right.clone(),
            };
            current_node_name = next_node_name;
            answer += 1;
        }
        multiples.push(answer);
    }
    lcm(&multiples).unwrap()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day8_input.txt").unwrap();
    let data = parse_input(&contents);
    let sol1: u64 = solution1(&data);
    let sol2: u64 = solution2_lcm(&data);

    (Solution::from(sol1), Solution::from(sol2))
}
