use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

type Label = u32

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct PuzzleInput {
    // left = true, right = false
    directions: Vec<bool>,
    nodes: HashMap<String, Node>
}

// https://rosettacode.org/wiki/Least_common_multiple#Rust
use std::cmp::{max, min};

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: &usize) -> usize {
    a * b / gcd(a, *b)
}


pub fn part1(input: &PuzzleInput) -> usize {
    let mut cur_pos = "AAA";
    let mut i = 0;
    while cur_pos != "ZZZ" {
        let options = input.nodes.get(cur_pos).unwrap();
        cur_pos = match input.directions[i % input.directions.len()] {
            true => options.left.as_str(),
            false => options.right.as_str(),
        };
        i += 1;
    }
    
    i
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut start_nodes: Vec<&str> = input.nodes.keys().filter(|node| node.ends_with('A')).map(|node| node.as_str()).collect();
    let cycles: Vec<usize> = start_nodes.iter_mut().map(|node| {
        let mut i = 0;
        while node.as_bytes()[2] != b'Z' {
            let options = input.nodes.get(&node.to_string()).unwrap();
            *node = match input.directions[i % input.directions.len()] {
                true => options.left.as_str(),
                false => options.right.as_str(),
            };
            i += 1;
        }
        i
    }).collect();
    
    let res = cycles.iter().fold(1usize, lcm);
    
    res
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut iter = reader.lines().flatten();
    let directions = iter.next().unwrap().chars().map(|c| c == 'L').collect();
    iter.next();
    let nodes = iter.map(|line| {
        let (key, tuple) = line.split_once(" = ").unwrap();
        let (left, right) = tuple[1..tuple.len()-1].split_once(", ").unwrap();
        let (left, right) = (left.to_string(), right.to_string());
        (key.to_string(), Node{left, right})
    }).collect();

    PuzzleInput { directions, nodes }
}
