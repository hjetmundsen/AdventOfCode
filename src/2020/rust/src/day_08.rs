use std::collections::HashSet;

use crate::utils;

fn star1() -> i32 {
    let lines = utils::file_to_lines("../inputs/day8.txt");
    let mut visited: HashSet<usize> = HashSet::new();
    let mut index = 0;
    let mut acc = 0;

    loop {
        if visited.contains(&index) {
            break;
        }

        visited.insert(index);
        let line: Vec<&str> = lines[index].split(' ').collect();
        let instruction = line[0];
        let sign: i32 = if &line[1][0..1] == "+" { 1 } else { -1 };
        let value: i32 = line[1][1..].parse::<i32>().unwrap();

        if instruction == "nop" {
            index += 1;
        } else if instruction == "acc" {
            index += 1;
            acc += value * sign;
        } else {
            let temp = index as i32;
            index = (temp + (value * sign)) as usize;
        }
    }

    acc
}

fn star2() -> u32 {
    0
}

pub fn main() {
    println!("DAY 8\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
