use std::collections::HashMap;
use crate::utils;

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_02.txt");
    
    let rules = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    
    lines.iter().map(|line| rules.get(line.as_str()).unwrap()).sum()
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_02.txt");

    let rules = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    lines.iter().map(|line| rules.get(line.as_str()).unwrap()).sum()
}

pub fn main() {
    println!("DAY 2\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
