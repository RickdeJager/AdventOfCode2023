use std::fs::File;
use std::io::{BufRead, BufReader};

type PuzzleInput = Vec<String>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dir_to_delta(dir: &Direction) -> (isize, isize) {
    use Direction::*;
    match dir {
        Up => (0, 1),
        Down => (0, -1),
        Left => (-1, 0),
        Right => (1, 0),
    }
}

struct Instruction {
    op: Direction,
    arg: isize,
}

fn solve(instructions: &Vec<Instruction>) -> isize {
    let mut points = Vec::new();
    let mut perim = 0;
    let (mut x, mut y) = (0, 0);
    for instruction in instructions {
        let delta = dir_to_delta(&instruction.op);
        x += delta.0 * instruction.arg;
        y += delta.1 * instruction.arg;
        perim += instruction.arg;
        points.push((x, y));
    }

    // Calculate the area using the shoelace formula
    let mut area = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area += points[i].0 * points[j].1;
        area -= points[i].1 * points[j].0;
    }

    area.abs() / 2 + perim / 2 + 1
}

pub fn part1(input: &PuzzleInput) -> isize {
    let instructions = input
        .iter()
        .map(|line| {
            let mut iter = line.split(' ');
            let op = match iter.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let arg = iter.next().unwrap().parse::<isize>().unwrap();

            Instruction { op, arg }
        })
        .collect::<Vec<Instruction>>();
    solve(&instructions)
}

pub fn part2(input: &PuzzleInput) -> isize {
    let instructions = input
        .iter()
        .map(|line| {
            let hex = isize::from_str_radix(
                line.split_once('#').unwrap().1.split_once(')').unwrap().0,
                16,
            )
            .unwrap();
            let arg = (hex & 0xfffff0) >> 4;
            let op = match hex & 0xf {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!("Invalid direction"),
            };

            Instruction { op, arg }
        })
        .collect::<Vec<Instruction>>();
    solve(&instructions)
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().flatten().collect()
}
