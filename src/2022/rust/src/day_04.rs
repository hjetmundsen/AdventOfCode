use crate::utils;

fn star1() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_04.txt");

    let mut result: u32 = 0;
    for line in lines {
        let mut line_split = line.split(",");
        let mut elf1 = line_split.next().unwrap().split("-");
        let mut elf2 = line_split.next().unwrap().split("-");

        let (elf1_min, elf1_max) = (elf1.next().unwrap().parse::<u32>().unwrap(), elf1.next().unwrap().parse::<u32>().unwrap());
        let (elf2_min, elf2_max) = (elf2.next().unwrap().parse::<u32>().unwrap(), elf2.next().unwrap().parse::<u32>().unwrap());

        if (elf1_min <= elf2_min && elf1_max >= elf2_max) || (elf2_min <= elf1_min && elf2_max >= elf1_max) {
            result += 1;
        }
    }

    result
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day_04.txt");

    let mut result: u32 = 0;
    for line in lines {
        let mut line_split = line.split(",");
        let mut elf1 = line_split.next().unwrap().split("-");
        let mut elf2 = line_split.next().unwrap().split("-");

        let (elf1_min, elf1_max) = (elf1.next().unwrap().parse::<u32>().unwrap(), elf1.next().unwrap().parse::<u32>().unwrap());
        let (elf2_min, elf2_max) = (elf2.next().unwrap().parse::<u32>().unwrap(), elf2.next().unwrap().parse::<u32>().unwrap());

        if (elf1_min <= elf2_min && elf1_max >= elf2_min) ||
        (elf1_min <= elf2_max && elf1_max >= elf2_max) ||
        (elf1_min <= elf2_min && elf1_max >= elf2_max) ||
        (elf2_min <= elf1_min && elf2_max >= elf1_max) {
            result += 1;
        }
    }

    result
}

pub fn solve() {
    println!("DAY 4\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
