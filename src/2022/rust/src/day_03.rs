use std::collections::HashSet;
use crate::utils;

const SCORES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_03.txt");

    let mut score = 0;
    for rucksack in lines {
        let (sack1, sack2) = rucksack.split_at(rucksack.len() / 2);
        let sack1_set = sack1.chars().collect::<HashSet<char>>();
        
        score += 1 + SCORES.find(sack2.chars().filter(|c| sack1_set.contains(c)).next().unwrap()).unwrap() as u32;
    }
    
    score
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_03.txt");
    
    let mut score = 0;
    for chunk in lines.chunks(3) {
        let (sack1_set, sack2_set, sack3) = (
            chunk[0].chars().collect::<HashSet<char>>(),
            chunk[1].chars().collect::<HashSet<char>>(),
            &chunk[2]
        );
        
        score += 1 + SCORES.find(sack3.chars()
                .filter(|item| sack1_set.contains(item) && sack2_set.contains(item))
                .next().unwrap()
            ).unwrap() as u32;
    }
    
    score
}

pub fn solve() {
    println!("DAY 3\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
