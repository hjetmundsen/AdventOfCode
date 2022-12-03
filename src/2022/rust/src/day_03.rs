use std::collections::HashSet;
use crate::utils;

fn star1() -> u32 {
    let rucksacks: Vec<String> = utils::file_to_lines("../inputs/day_03.txt");
    let mut errors: Vec<char> = Vec::new();
    let scores = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    for rs in rucksacks {
        let num_items = rs.len();
        let (sack1, sack2) = rs.split_at(num_items / 2);
        
        let sack1_set = sack1.chars().collect::<HashSet<char>>();
        
        let mut unison_set = HashSet::new();
        for c in sack2.chars() {
            if sack1_set.contains(&c) {
                unison_set.insert(c);
            }
        }
        
        errors.extend(unison_set.iter());
    }
    
    let mut score = errors.len() as u32;
    for item in errors {
        score += scores.find(item).unwrap() as u32;
    }

    score
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_03.txt");
    let scores = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    let mut score = 0;
    for x in (0..lines.len()).step_by(3) {
        let (sack1, sack2, sack3) = (
            lines.get(x).unwrap(),
            lines.get(x+1).unwrap(),
            lines.get(x+2).unwrap()
        );

        let sack1_set = sack1.chars().collect::<HashSet<char>>();
        let sack2_set = sack2.chars().collect::<HashSet<char>>();
        
        for item in sack3.chars() {
            if sack1_set.contains(&item) && sack2_set.contains(&item) {
               score += 1 + scores.find(item).unwrap() as u32; 
               break;
            }
        }
    }
    
    score
}

pub fn solve() {
    println!("DAY 3\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
