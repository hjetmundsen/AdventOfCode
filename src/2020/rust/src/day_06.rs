use std::collections::HashSet;

use crate::utils;

fn star1() -> usize {
    let mut result: usize = 0;
    let mut yes: HashSet<char> = HashSet::new();
    let lines = utils::file_to_lines("../inputs/day6.txt");

    for line in lines {
        if line == "" {
            result += yes.len();
            yes.clear();
        } else {
            for i in line.chars() {
                yes.insert(i);
            }
        }
    }

    return result;
}

fn star2() -> usize {
    let mut result: usize = 0;
    let mut responses: Vec<HashSet<char>> = Vec::new();
    let lines: Vec<String> = utils::file_to_lines("../inputs/day6.txt");

    for line in lines {
        if line == "" {
            if !responses.is_empty() {
                let mut temp = responses[0].clone();
                for r in responses[1..].iter() {
                    temp = temp.intersection(r).copied().collect();
                }
                result += temp.len();
                responses.clear();
            }
        } else {
            let mut temp: HashSet<char> = HashSet::new();
            for i in line.chars() {
                temp.insert(i);
            }
            responses.push(temp);
        }
    }

    return result;
}

pub fn main() {
    println!("DAY 6\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}