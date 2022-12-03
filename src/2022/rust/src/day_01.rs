use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::utils;

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_01.txt");

    lines.split(|line| line.is_empty())
        .map(|elf| elf.iter().map(|item| item.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
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

pub fn solve() {
    println!("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
