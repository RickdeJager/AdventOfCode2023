use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_forward(data: &[i32]) -> i32 {
    // Base case, all data is 0, so our prediction for the next value is 0
    // (aka this is the lowest layer)
    if data.iter().all(|&x| x == 0) {
        return 0;
    }

    // inductive case, we have to build the next array and recurse
    let next_data = data.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();

    data[data.len() - 1] + solve_forward(&next_data)
}

fn solve_backwards(data: &[i32]) -> i32 {
    // Base case, all data is 0, so our prediction for the next value is 0
    // (aka this is the lowest layer)
    if data.iter().all(|&x| x == 0) {
        return 0;
    }

    // inductive case, we have to build the next array and recurse
    let next_data = data.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();

    data[0] - solve_backwards(&next_data)
}

pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|x| solve_forward(&x)).sum()
}

pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|x| solve_backwards(&x)).sum()
}

pub fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}
