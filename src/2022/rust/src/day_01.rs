use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::utils;

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_01.txt");

    let mut max_cals: u32 = 0;
    for elf in lines.split(|line| line.is_empty()) {
        max_cals = cmp::max(max_cals, elf.iter().map(|item| item.parse::<u32>().unwrap()).sum());
    }

    max_cals
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_01.txt");
    let mut min_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    for elf in lines.split(|line| line.is_empty()) {
        min_heap.push(Reverse(elf.iter().map(|item| item.parse::<u32>().unwrap()).sum()));
        
        if min_heap.len() > 3 {
            min_heap.pop();
        }
    }
    
    min_heap.iter().map(|Reverse(x)| x).sum()
}

pub fn main() {
    println!("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
