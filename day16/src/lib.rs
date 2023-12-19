use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type PuzzleInput = Vec<Vec<char>>;

type Coord = (isize, isize);
type Dir = (isize, isize);

fn simulate(input: &PuzzleInput, start: Coord, dir: Dir) -> usize {
    // Keep a separate map to color in
    let mut map = input.clone();

    let w = input[0].len() as isize;
    let h = input.len() as isize;

    let mut work = VecDeque::new();
    work.push_front((start, dir));
    let mut seen = HashSet::new();

    while let Some(elem) = work.pop_front() {
        let (mut x, mut y): Coord = elem.0;
        let (mut dx, mut dy) = elem.1;
        while x >= 0 && x < w && y >= 0 && y < h {
            // color unconditionally, it's Christmas after all
            map[y as usize][x as usize] = '#';
            match input[y as usize][x as usize] {
                '.' => {}
                '/' => (dx, dy) = (-dy, -dx),
                '\\' => {
                    (dx, dy) = (dy, dx);
                }
                '|' => match (dx, dy) {
                    (_, 0) => {
                        // split into two. This job spawns two copies and ends here
                        let job_up = ((x, y - 1), (0, -1));
                        let job_down = ((x, y + 1), (0, 1));
                        for job in [job_down, job_up] {
                            if seen.insert(job) {
                                work.push_front(job);
                            }
                        }
                        break;
                    }
                    _ => { /* no-op */ }
                },
                '-' => match (dx, dy) {
                    (0, _) => {
                        let job_left = ((x - 1, y), (-1, 0));
                        let job_right = ((x + 1, y), (1, 0));
                        for job in [job_left, job_right] {
                            if seen.insert(job) {
                                work.push_front(job);
                            }
                        }
                        break;
                    }
                    _ => { /* no-op */ }
                },

                _ => unreachable!(),
            }
            y += dy;
            x += dx;
        }
    }

    // Count all the colored cells in our scratch map
    map.iter().flatten().filter(|&&x| x == '#').count()
}

pub fn part1(input: &PuzzleInput) -> usize {
    simulate(input, (0, 0), (1, 0))
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut max = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if x != 0 && x != input[0].len() - 1 && y != 0 && y != input.len() - 1 {
                // lmao O(n^2)
                continue;
            }

            let start = (x as isize, y as isize);
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let ret = simulate(input, start, dir);
                if ret > max {
                    max = ret;
                }
            }
        }
    }
    max
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
