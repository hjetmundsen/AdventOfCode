extern crate regex;

use crate::utils;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return false,
        }
    };
}

fn validate_fields(fields: &HashMap<&str, &str>) -> bool {
    let eye_colors: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();
    let byr = unwrap_or_return!(fields.get("byr").unwrap().parse::<u32>());
    let iyr = unwrap_or_return!(fields.get("iyr").unwrap().parse::<u32>());
    let eyr = unwrap_or_return!(fields.get("eyr").unwrap().parse::<u32>());
    let hgt = *fields.get("hgt").unwrap();
    let hgt_val = unwrap_or_return!(hgt[..hgt.len() - 2].parse::<u32>());
    let hgt_unit = &hgt[hgt.len() - 2..];
    let hcl = *fields.get("hcl").unwrap();
    let ecl = *fields.get("ecl").unwrap();
    let pid = fields.get("pid").unwrap();
    let hex_regex = Regex::new("^#(?:[0-9a-fA-F]{6})$").unwrap();

    if pid.len() != 9 {
        return false;
    }

    byr >= 1920
        && byr <= 2002
        && iyr >= 2010
        && iyr <= 2020
        && eyr >= 2020
        && eyr <= 2030
        && ((hgt_unit == "cm" && hgt_val >= 150 && hgt_val <= 193)
            || (hgt_unit == "in" && hgt_val >= 59 && hgt_val <= 76))
        && hex_regex.is_match(hcl)
        && eye_colors.contains(ecl)
        && pid.parse::<u64>().is_ok()
}

fn star1() -> u32 {
    let lines = utils::file_to_lines("../inputs/day4.txt");
    let required: Vec<&str> = vec!["hcl", "iyr", "eyr", "ecl", "pid", "byr", "hgt"];
    let mut result: u32 = 0;
    let mut found: HashSet<&str> = HashSet::new();

    for line in &lines {
        if line == "" {
            if required.iter().all(|&x| found.contains(x)) {
                result += 1;
            }
            found.clear();
        } else {
            for field in line.split(' ') {
                found.insert(field.get(..3).unwrap());
            }
        }
    }

    result
}

fn star2() -> u32 {
    let lines = utils::file_to_lines("../inputs/day4.txt");
    let required: Vec<&str> = vec!["hcl", "iyr", "eyr", "ecl", "pid", "byr", "hgt"];
    let mut result: u32 = 0;
    let mut found: HashMap<&str, &str> = HashMap::new();

    for line in &lines {
        if line == "" {
            if required.iter().all(|&x| found.contains_key(x)) && validate_fields(&found) {
                result += 1;
            }
            found.clear();
        } else {
            for field in line.split(' ') {
                found.insert(field.get(..3).unwrap(), field.get(4..).unwrap());
            }
        }
    }

    result
}

pub fn main() {
    println!("DAY 4\n=====\nSTAR 1: {}\nSTAR 2: {}\n", star1(), star2());
}
