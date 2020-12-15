use std::collections::HashSet;

use crate::utils;

fn star1() -> i32 {
    let lines = utils::file_to_lines("../inputs/day1.txt");
    let mut visited: HashSet<i32> = HashSet::new();

    for line in lines {
        let num = line.parse::<i32>().unwrap();
        if visited.contains(&(2020 - num)) {
            return num * (2020 - num);
        }

        visited.insert(num);
    }

    return 0;
}

fn star2() -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let lines = utils::file_to_lines("../inputs/day1.txt");

    for line in lines {
        nums.push(line.parse::<i32>().unwrap());
    }

    nums.sort();

    for (ind, num) in nums.iter().enumerate() {
        let mut left = ind + 1;
        let mut right = nums.len() - 1;

        while left < right {
            if num + nums.get(left).unwrap() + nums.get(right).unwrap() > 2020 {
                right -= 1;
            } else if num + nums.get(left).unwrap() + nums.get(right).unwrap() < 2020 {
                left += 1;
            } else {
                return num * nums.get(left).unwrap() * nums.get(right).unwrap();
            }
        }
    }

    return 0;
}

pub fn main() {
    println!("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
