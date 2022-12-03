use std::collections::HashMap;
use crate::utils;

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_02.txt");
    
    let rules = HashMap::from([
        ('A', 'Z'),
        ('B', 'X'),
        ('C', 'Y'),
        ('X', 'C'),
        ('Y', 'A'),
        ('Z', 'B')
    ]);
    
    let letter_scores = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);

    let mut score = 0;
    for line in lines {
        let opponent = line.chars().nth(0).unwrap();
        let mine = line.chars().nth(2).unwrap();
        
        score += letter_scores.get(&mine).unwrap();
        
        if rules.get(&mine).unwrap() == &opponent {
            score += 6;
        } else if rules.get(&opponent).unwrap() == &mine {
            continue;
        } else {
            score += 3;
        }
    }
    
    score
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_02.txt");
    
    let beats = HashMap::from([
        ('A', 'C'),
        ('B', 'A'),
        ('C', 'B'),
    ]);

    let loses = HashMap::from([
        ('A', 'B'),
        ('B', 'C'),
        ('C', 'A'),
    ]);
    
    let letter_scores = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3)
    ]);

    let mut score = 0;
    for line in lines {
        let opponent = line.chars().nth(0).unwrap();
        let outcome = line.chars().nth(2).unwrap();
        
        if outcome == 'X' {
            let mine = beats.get(&opponent).unwrap();
            
            score += letter_scores.get(mine).unwrap();
        } else if outcome == 'Y' {
            score += letter_scores.get(&opponent).unwrap() + 3;
        } else {
            let mine = loses.get(&opponent).unwrap();
            
            score += letter_scores.get(mine).unwrap() + 6;
        }
    }
    
    score
}

pub fn main() {
    println!("DAY 2\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
