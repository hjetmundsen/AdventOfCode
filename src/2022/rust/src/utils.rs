use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn file_to_lines(path: &str) -> Vec<String> {
    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut string_lines: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            string_lines.push(line);
        };
    }

    string_lines
}