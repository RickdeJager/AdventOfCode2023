use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in input {
        let mut first = 0;
        let mut last = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                first = c.to_digit(10).unwrap() as usize;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = c.to_digit(10).unwrap() as usize;
                break;
            }
        }

        sum += first * 10 + last;
    }

    sum
}

const TOKENS: [(&'static str, usize); 20] = [
    ("zero", 0),
    ("0", 0),
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

pub fn part2(input: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in input {
        let first = TOKENS
            .iter()
            .filter_map(|(token, value)| {
                let idx = line.find(token)?;
                Some((idx, *value))
            })
            .min_by_key(|(idx, _value)| *idx)
            .expect("No first token found")
            .1;

        let last = TOKENS
            .iter()
            .filter_map(|(token, value)| {
                let idx = line.rfind(token)?;
                Some((idx, *value))
            })
            .max_by_key(|(idx, _value)| *idx)
            .expect("No last token found")
            .1;

        sum += first * 10 + last;
    }

    sum
}

pub fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}
