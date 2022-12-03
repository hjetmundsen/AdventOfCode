use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn read_input() -> Vec<Vec<u32>> {
    let file = File::open("../inputs/day_02.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<u32>> = Vec::new();
    
    reader.lines()
            .map(|line| line.unwrap().split_whitespace())
            .map(|split| split.map(|item| item.parse::<u32>().unwrap()).collect())
            .collect()
}

fn star1() -> u32 {
    0
}

fn star2() -> u32 {
    0
}

pub fn main() {
    println!("DAY 2\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}