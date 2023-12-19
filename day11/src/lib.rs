use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone)]
pub struct PuzzleInput {
    map: HashMap<(isize, isize), bool>,
}

pub fn part1(input: &PuzzleInput) -> isize {
    solve(input, 1)
}

pub fn part2(input: &PuzzleInput) -> isize {
    solve(input, 1_000_000 - 1)
}

pub fn solve(input: &PuzzleInput, offset: isize) -> isize {
    let x_coords: Vec<_> = input.map.keys().map(|(x, _y)| *x).sorted().collect();
    let y_coords: Vec<_> = input.map.keys().map(|(_x, y)| *y).sorted().collect();

    let mut x_multipliers = Vec::new();
    let mut y_multipliers = Vec::new();

    x_coords.windows(2).for_each(|w| {
        for x in w[0] + 1..w[1] {
            x_multipliers.push(x);
        }
    });

    y_coords.windows(2).for_each(|w| {
        for y in w[0] + 1..w[1] {
            y_multipliers.push(y);
        }
    });

    input
        .map
        .keys()
        .combinations(2)
        .map(|c| {
            let (p1, p2) = (c[0], c[1]);
            let (mut x1, mut y1) = p1;
            let (mut x2, mut y2) = p2;
            x1 += offset * x_multipliers.iter().filter(|&&x| x < x1).count() as isize;
            x2 += offset * x_multipliers.iter().filter(|&&x| x < x2).count() as isize;

            y1 += offset * y_multipliers.iter().filter(|&&y| y < y1).count() as isize;
            y2 += offset * y_multipliers.iter().filter(|&&y| y < y2).count() as isize;

            (x1 - x2).abs() + (y1 - y2).abs()
        })
        .sum()
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let map = reader
        .lines()
        .flatten()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| match c == '#' {
                    true => Some(((x as isize, y as isize), true)),
                    false => None,
                })
                .collect::<HashMap<(isize, isize), bool>>()
        })
        .flatten()
        .collect::<HashMap<(isize, isize), bool>>();

    PuzzleInput { map }
}
