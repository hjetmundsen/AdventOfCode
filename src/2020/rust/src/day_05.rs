use crate::utils;

fn calculate_seat_ids() -> Vec<u32> {
    let mut seat_ids: Vec<u32> = Vec::new();
    let lines = utils::file_to_lines("../inputs/day5.txt");

    for line in lines {
        let (mut r_min, mut r_max) = (0, 127);
        let (mut c_min, mut c_max) = (0, 7);

        for r in line[..7].chars() {
            let r_mid = r_min + ((1 + (r_max - r_min)) / 2);
            if r == 'F' {
                r_max = r_mid
            } else {
                r_min = r_mid
            }
        }
        
        let row: u32 = r_min;

        for c in line[7..].chars() {
            let c_mid = c_min + ((1 + (c_max - c_min)) / 2);
            if c == 'L' {
                c_max = c_mid;
            } else {
                c_min = c_mid;
            }

            let col = c_min;

            seat_ids.push(row * 8 + col);
        }

    }

    return seat_ids;
}

fn star1() -> u32 {
    return *calculate_seat_ids().iter().max().unwrap();
}

fn star2() -> u32 {
    let mut seat_ids = calculate_seat_ids();
    seat_ids.sort();

    for (i,v) in seat_ids.iter().enumerate() {
        if i == seat_ids.len() {
            break;
        }

        if *seat_ids.get(i+1).unwrap() == v + 2 {
            return v + 1;
        }
    }

    return 0;
}

pub fn main() {
    println!("DAY 5\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}