use crate::utils;
use regex::Regex;

fn star1() -> u32 {
    let lines = utils::file_to_lines("../inputs/day7.txt");
    let re = Regex::new("(.*) bags contain").unwrap();

    return 0;
}

fn star2() -> u32 {
    0
}

pub fn main() {
    println!("DAY 7\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
