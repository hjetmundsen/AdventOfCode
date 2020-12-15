use crate::utils;

fn star1() -> u32 {
    let lines = utils::file_to_lines("../inputs/day3.txt");
    let (mut row, mut col) = (0, 0);
    let mut result: u32 = 0;

    while row < lines.len() {
        let line = lines.get(row).unwrap();
        if line.chars().nth(col).unwrap() == '#' {
            result += 1;
        }
        row += 1;
        col = (col + 3) % line.len();
    }

    result
}

fn star2(x: usize, y: usize) -> u64 {
    let lines = utils::file_to_lines("../inputs/day3.txt");
    let mut result: u64 = 0;
    let (mut row, mut col) = (0, 0);

    while row < lines.len() {
        let line = lines.get(row).unwrap();
        if line.chars().nth(col).unwrap() == '#' {
            result += 1;
        }
        row += y;
        col = (col + x) % line.len();
    }

    result
}

pub fn main() {
    let star1 = star1();
    let star2 = star2(1, 1) * star2(3, 1) * star2(5, 1) * star2(7, 1) * star2(1, 2);
    println!("DAY 3\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1, star2);
}
