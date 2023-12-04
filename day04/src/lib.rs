use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct ScratchTicket {
    winning: BTreeSet<u8>,
    mine: BTreeSet<u8>,
}

impl From<&str> for ScratchTicket {
    fn from(line: &str) -> Self {
        let (_id, numbers) = line.split_once(": ").unwrap();

        let (winning, mine) = numbers.split_once(" | ").unwrap();
        let winning = winning
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mine = mine
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        ScratchTicket { winning, mine }
    }
}

pub fn part1(input: &Vec<ScratchTicket>) -> usize {
    let mut res = 0usize;
    for ticket in input {
        let matching = ticket.mine.intersection(&ticket.winning).count();
        if matching > 0 {
            res += 2usize.pow(matching as u32 - 1);
        }
    }
    res
}

pub fn part2(input: &Vec<ScratchTicket>) -> usize {
    let mut quantities = vec![1usize; input.len()];
    for (idx, ticket) in input.iter().enumerate() {
        let matching = ticket.mine.intersection(&ticket.winning).count();
        let instances = quantities[idx];

        let start = idx + 1;
        let end = usize::min(start + matching, input.len());
        for i in start..end {
            quantities[i] += instances;
        }
    }
    quantities.iter().sum()
}

pub fn read_input(filename: &str) -> Vec<ScratchTicket> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .flatten()
        .map(|x| x.as_str().into())
        .collect()
}
