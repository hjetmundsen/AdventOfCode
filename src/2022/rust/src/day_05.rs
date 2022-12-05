use std::collections::VecDeque;
use crate::utils;

fn star1() -> String {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_05.txt");

    let mut stacks: Vec<Vec<char>> = Vec::from([
        Vec::from(['R', 'P', 'C', 'D', 'B', 'G']),
        Vec::from(['H', 'V', 'G']),
        Vec::from(['N', 'S', 'Q', 'D', 'J', 'P', 'M']),
        Vec::from(['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M']),
        Vec::from(['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S']),
        Vec::from(['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S']),
        Vec::from(['B', 'Z', 'M', 'H', 'F', 'T', 'Q']),
        Vec::from(['C', 'M', 'D', 'B', 'F']),
        Vec::from(['F', 'C', 'Q', 'G'])
    ]);

    let mut temp_stack = VecDeque::new();
    for line in lines {
        let mut instructions = line.split(" ");

        let (count, src, dst) = (
            instructions.nth(1).unwrap().parse::<usize>().unwrap(),
            instructions.nth(1).unwrap().parse::<usize>().unwrap(),
            instructions.nth(1).unwrap().parse::<usize>().unwrap()
        );

        {
            let src_vec = &mut stacks[src - 1];

            for _ in 0..count {
                temp_stack.push_back(src_vec.pop().unwrap());
            }
        }

        {
            let dest_vec = &mut stacks[dst - 1];

            while let Some(val) = temp_stack.pop_front() {
                dest_vec.push(val);
            }
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn star2() -> String {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_05.txt");

    let mut stacks: Vec<Vec<char>> = Vec::from([
        Vec::from(['R', 'P', 'C', 'D', 'B', 'G']),
        Vec::from(['H', 'V', 'G']),
        Vec::from(['N', 'S', 'Q', 'D', 'J', 'P', 'M']),
        Vec::from(['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M']),
        Vec::from(['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S']),
        Vec::from(['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S']),
        Vec::from(['B', 'Z', 'M', 'H', 'F', 'T', 'Q']),
        Vec::from(['C', 'M', 'D', 'B', 'F']),
        Vec::from(['F', 'C', 'Q', 'G'])
    ]);

    let mut temp_stack = Vec::new();
    for line in lines {
        let mut instructions = line.split(" ");

        let (count, src, dst) = (
            instructions.nth(1).unwrap().parse::<usize>().unwrap(),
            instructions.nth(1).unwrap().parse::<usize>().unwrap(),
            instructions.nth(1).unwrap().parse::<usize>().unwrap()
        );

        {
            let src_vec = &mut stacks[src - 1];

            for _ in 0..count {
                temp_stack.push(src_vec.pop().unwrap());
            }
        }

        {
            let dest_vec = &mut stacks[dst - 1];

            while let Some(val) = temp_stack.pop() {
                dest_vec.push(val);
            }
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn solve() {
    println!("DAY 5\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
