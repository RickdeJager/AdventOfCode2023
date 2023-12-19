use std::fs::File;
use std::io::{BufRead, BufReader};

type PuzzleInput = Vec<String>;

pub fn part1(input: &PuzzleInput) -> usize {
    1
}

pub fn part2(input: &PuzzleInput) -> usize {
    1
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    
    reader.lines()
        .flatten()
        .collect()
}
