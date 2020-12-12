use crate::utils;

fn star1() -> i32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day2.txt");
    let mut result: i32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let bounds: Vec<i32> = parts[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        let letter: char = parts[1].chars().next().unwrap();
        let password: &str = parts[2];

        let count: i32 = password.matches(letter).count() as i32;

        if count >= bounds[0] && count <= bounds[1] {
            result += 1;
        }
    }

    return result;
}

fn star2() -> i32 {
    let lines: Vec<String> = utils::file_to_lines("../inputs/day2.txt");
    let mut result: i32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let positions: Vec<i32> = parts[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        let letter: char = parts[1].chars().next().unwrap();
        let password = parts[2];

        if (password.chars().nth((positions[0]-1) as usize).unwrap() == letter) ^ (password.chars().nth((positions[1]-1) as usize).unwrap() == letter) {
            result += 1;
        }
    }

    return result;
}

pub fn main() {
    println!("DAY 2\n=====\nSTAR 1: {}\nSTAR 2: {}\n\n", star1(), star2());
}
