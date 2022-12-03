use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::utils;

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_01.txt");

    let mut max_cals: u32 = 0;
    let mut cur_cals: u32 = 0;
    for line in lines {
        if line.is_empty() {
            max_cals = cmp::max(cur_cals, max_cals);
            cur_cals = 0;
            continue;
        }

        cur_cals += line.parse::<u32>().unwrap();
    }

    max_cals
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_01.txt");
    let mut min_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut cur_cals: u32 = 0;

    for line in lines {
        if line.is_empty() {
            if min_heap.len() < 3 {
                min_heap.push(Reverse(cur_cals));
            } else if &Reverse(cur_cals) < min_heap.peek().unwrap() {
                min_heap.pop();
                min_heap.push(Reverse(cur_cals));
            }

            cur_cals = 0;
            continue;
        }

        cur_cals += line.parse::<u32>().unwrap();
    }

    let mut total_cals: u32 = 0;
    while let Some(Reverse(cal)) = min_heap.pop() {
        total_cals += cal;
    }

    total_cals
}

pub fn main() {
    println!("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
