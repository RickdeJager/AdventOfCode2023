use std::fs::File;
use std::io::{BufRead, BufReader};

type Label = u16;

#[derive(Debug, Default, Clone, Copy)]
pub struct Node {
    left: Label,
    right: Label,
}

#[derive(Debug)]
pub struct PuzzleInput {
    // left = true, right = false
    directions: Vec<bool>,
    // A huge sparse array (65k*sizeof(Node)) as a standin for a hashmap.
    // as long as you're willing to accept that we can fit each label in a u16
    // this gives a huge performance boost
    map: Vec<Node>,
    // Because we don't use a hashmap anymore, we need to track which keys are valid
    nodes: Vec<Label>,
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

#[inline(always)]
fn key_from_str(s: &str) -> Label {
    s.bytes()
        .enumerate()
        .take(3)
        .map(|(i, b)| ((b - b'A') as Label) << (i * 5))
        .sum()
}

#[inline(always)]
fn is_start_node(l: Label) -> bool {
    (l >> (2 * 5) & 0b11111) == 0u16
}

#[inline(always)]
fn is_end_node(l: Label) -> bool {
    (l >> (2 * 5) & 0b11111) == (b'Z' - b'A') as u16
}

pub fn part1(input: &PuzzleInput) -> usize {
    let mut cur_pos = key_from_str("AAA");
    let end = key_from_str("ZZZ");
    let mut i = 0;
    while cur_pos != end {
        let options = input.map[cur_pos as usize];
        cur_pos = match input.directions[i % input.directions.len()] {
            true => options.left,
            false => options.right,
        };
        i += 1;
    }

    i
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut start_nodes: Vec<Label> = input
        .nodes
        .iter()
        .copied()
        .filter(|x| is_start_node(*x))
        .collect();
    let cycles: Vec<usize> = start_nodes
        .iter_mut()
        .map(|node| {
            let mut i = 0;
            while !is_end_node(*node) {
                let options = input.map[*node as usize];
                *node = match input.directions[i % input.directions.len()] {
                    true => options.left,
                    false => options.right,
                };
                i += 1;
            }
            i
        })
        .collect();

    let res = cycles.iter().fold(1, lcm);

    res
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut iter = reader.lines().flatten();
    let directions = iter.next().unwrap().chars().map(|c| c == 'L').collect();
    iter.next();
    let mut map = vec![Node::default(); 65536];
    let nodes = iter
        .map(|line| {
            let (key, tuple) = line.split_once(" = ").unwrap();
            let (left, right) = tuple[1..tuple.len() - 1].split_once(", ").unwrap();
            let (left, right) = (key_from_str(left), key_from_str(right));
            let key = key_from_str(key);
            map[key as usize] = Node { left, right };
            key
        })
        .collect();

    PuzzleInput {
        directions,
        nodes,
        map,
    }
}
