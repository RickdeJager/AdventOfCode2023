use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in input {
        let first = line
            .chars()
            .find(char::is_ascii_digit)
            .expect("No first digit found")
            .to_digit(10)
            .unwrap() as usize;
        let last = line
            .chars()
            .rfind(char::is_ascii_digit)
            .expect("No last digit found")
            .to_digit(10)
            .unwrap() as usize;
        sum += first * 10 + last;
    }

    sum
}

const TOKENS: [(&str, usize); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
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
