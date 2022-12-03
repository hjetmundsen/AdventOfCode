use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn read_input() -> Vec<u32> {
    let file = File::open("../inputs/day_01.txt").expect("file wasn't found.");
    let mut reader = BufReader::new(file);
    
    let mut line = String::new();
    let _x = reader.read_line(&mut line);
    let line = line.trim();
    
    return line.chars().map(|c| c.to_digit(10).unwrap()).collect();
}

fn star1 () -> u32 {
    let input = read_input();
    let mut sum = 0;
    
    for (ind, val) in input.iter().enumerate() {
        let next_val = input.get((ind + 1) % input.len()).unwrap();
        
        if val == next_val {
            sum += val;
        }
    }
    
    sum
}

fn star2 () -> u32 {
    let input = read_input();
    let mut sum = 0;
    let next_ind = input.len() / 2;
    
    for (ind, val) in input.iter().enumerate() {
        let next_val = input.get((ind + next_ind) % input.len()).unwrap();
        
        if val == next_val {
            sum += val;
        }
    }
    
    sum
}

pub fn main() {
    println!("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
