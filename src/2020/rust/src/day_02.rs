use crate::utils;

fn star1() -> u32 {
    let lines = utils::file_to_lines("../inputs/day2.txt");
    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let bounds: Vec<u32> = parts[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let letter = parts[1].chars().next().unwrap();
        let password = parts[2];

        let count = password.matches(letter).count() as u32;

        if count >= bounds[0] && count <= bounds[1] {
            result += 1;
        }
    }

    return result;
}

fn star2() -> u32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day2.txt");
    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let positions: Vec<u32> = parts[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let letter = parts[1].chars().next().unwrap();
        let password = parts[2];

        if (password.chars().nth((positions[0] - 1) as usize).unwrap() == letter)
            ^ (password.chars().nth((positions[1] - 1) as usize).unwrap() == letter)
        {
            result += 1;
        }
    }

    return result;
}

pub fn main() {
    println!("DAY 2\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
