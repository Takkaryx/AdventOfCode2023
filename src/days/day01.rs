use crate::{Solution, SolutionPair};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref WORD_TO_NUMBER: HashMap<&'static str, char> = {
        let mut map = HashMap::new();
        map.insert("one", '1');
        map.insert("two", '2');
        map.insert("three", '3');
        map.insert("four", '4');
        map.insert("five", '5');
        map.insert("six", '6');
        map.insert("seve", '7');
        map.insert("eight", '8');
        map.insert("nine", '9');
        map
    };
}

fn find_indecies(input: &str, search: &str) -> Vec<u64> {
    let mut indicies: Vec<u64> = Vec::new();
    let mut start_index = 0;

    while let Some(index) = input[start_index..].find(search) {
        let absolute_index = start_index + index;
        indicies.push(absolute_index as u64);
        start_index = absolute_index + search.len();
    }
    indicies
}

fn replace_char_at_index(input: &str, index: u64, new_char: char) -> String {
    let mut modified_string = input.to_owned();
    let char_indicies: Vec<_> = modified_string.char_indices().collect();
    if let Some((byte_index, _)) = char_indicies.get(index as usize) {
        modified_string.replace_range(
            *byte_index..byte_index + new_char.len_utf8(),
            &new_char.to_string(),
        );
    }
    modified_string
}

fn string_convert(input: &str) -> String {
    let lowercase_input = input.to_lowercase().to_owned();
    let mut my_input = lowercase_input.clone();
    for string_int in WORD_TO_NUMBER.keys() {
        if lowercase_input.contains(string_int) {
            let indecies = find_indecies(input, string_int);
            for index in indecies {
                my_input =
                    replace_char_at_index(my_input.as_ref(), index, WORD_TO_NUMBER[string_int]);
            }
        }
    }
    my_input
}

fn get_first_and_last(input: String) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let mut final_num: Vec<u64> = vec![];
    let numbers: Vec<u64> = re
        .find_iter(input.as_ref())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();
    for num in numbers {
        if num < 10 {
            final_num.push(num);
            continue;
        } else {
            let digits: Vec<char> = num.to_string().chars().collect();
            for digit in digits {
                final_num.push((digit as u8 - b'0') as u64);
                continue;
            }
        }
    }
    if final_num.is_empty() {
        return 0;
    }
    final_num[0] * 10 + final_num[final_num.len() - 1]
}

fn input_generator2(input: &str) -> u64 {
    let new_input = string_convert(input);
    get_first_and_last(new_input)
}

fn input_generator1(input: &str) -> u64 {
    get_first_and_last(input.to_string())
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;
    let contents = fs::read_to_string("input/day1_input.txt").unwrap();
    for line in contents.lines() {
        let temp_val = input_generator1(line);
        sol1 = temp_val + sol1;
        let temp_val2 = input_generator2(line);
        sol2 = temp_val2 + sol2;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
