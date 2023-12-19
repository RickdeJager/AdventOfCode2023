use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type PuzzleInput = Vec<Vec<char>>;

#[allow(unused)]
fn draw_board(board: &PuzzleInput) {
    for row in board {
        for char in row {
            print!("{}", char);
        }
        println!();
    }
}

fn score_board(board: &PuzzleInput) -> usize {
    let h = board.len();

    board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|c| (h - y) * (*c == 'O') as usize)
                .sum::<usize>()
        })
        .sum()
}

fn tilt_north(input: &mut PuzzleInput) {
    for x in 0..input[0].len() {
        let mut p1 = 0;

        while p1 < input.len() {
            match input[p1][x] {
                '#' | 'O' => {}
                // We found an empty slot, we could potentially slot in a new rock
                '.' => {
                    let mut p2 = p1 + 1;
                    // Skip ahead until we reach of one of the following conclusions:
                    // - '#': move p1 up an exit the loop
                    // - 'O': we can swap this rock into the free spot at p1
                    // - End of line: we're done processing this column. --> we don't care
                    while p2 < input.len() {
                        match input[p2][x] {
                            '#' => {
                                p1 = p2;
                                break;
                            }
                            '.' => {
                                p2 += 1;
                            }
                            'O' => {
                                // do the swap
                                input[p1][x] = input[p2][x];
                                input[p2][x] = '.';
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
            p1 += 1;
        }
    }
}

fn tilt_south(input: &mut PuzzleInput) {
    for x in 0..input[0].len() {
        let mut p1: isize = (input.len() - 1) as isize;

        while p1 >= 0 {
            match input[p1 as usize][x] {
                '#' | 'O' => {}
                '.' => {
                    let mut p2 = p1 - 1;
                    while p2 >= 0 {
                        match input[p2 as usize][x] {
                            '#' => {
                                p1 = p2;
                                break;
                            }
                            '.' => {
                                p2 -= 1;
                            }
                            'O' => {
                                input[p1 as usize][x] = input[p2 as usize][x];
                                input[p2 as usize][x] = '.';
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
            p1 -= 1;
        }
    }
}

fn tilt_west(input: &mut PuzzleInput) {
    for y in 0..input.len() {
        let mut p1 = 0;

        while p1 < input.len() {
            match input[y][p1] {
                '#' | 'O' => {}
                '.' => {
                    let mut p2 = p1 + 1;
                    while p2 < input.len() {
                        match input[y][p2] {
                            '#' => {
                                p1 = p2;
                                break;
                            }
                            '.' => {
                                p2 += 1;
                            }
                            'O' => {
                                input[y][p1] = input[y][p2];
                                input[y][p2] = '.';
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
            p1 += 1;
        }
    }
}

fn tilt_east(input: &mut PuzzleInput) {
    for y in 0..input.len() {
        let mut p1: isize = (input[0].len() - 1) as isize;

        while p1 >= 0 {
            match input[y][p1 as usize] {
                '#' | 'O' => {}
                '.' => {
                    let mut p2 = p1 - 1;
                    while p2 >= 0 {
                        match input[y][p2 as usize] {
                            '#' => {
                                p1 = p2;
                                break;
                            }
                            '.' => {
                                p2 -= 1;
                            }
                            'O' => {
                                input[y][p1 as usize] = input[y][p2 as usize];
                                input[y][p2 as usize] = '.';
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
            p1 -= 1;
        }
    }
}

fn tilt_cycle(input: &mut PuzzleInput) {
    tilt_north(input);
    tilt_west(input);
    tilt_south(input);
    tilt_east(input);
}

pub fn part1(input: &PuzzleInput) -> usize {
    let mut input = input.clone();
    tilt_north(&mut input);
    score_board(&input)
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut input = input.clone();
    let mut board_map = HashMap::new();

    let mut i = 0;
    let mut left = 1_000_000_000;
    while left > 0 {
        tilt_cycle(&mut input);

        match board_map.get(&input) {
            Some(prev_cycle) => {
                let cycle_len = i - prev_cycle;
                left %= cycle_len;
            }
            None => {
                board_map.insert(input.clone(), i);
            }
        };
        left -= 1;
        i += 1;
    }
    score_board(&input)
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect()
}
