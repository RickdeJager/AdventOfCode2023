use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

type PuzzleInput = Vec<Vec<u8>>;

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Default, Clone, Copy, Debug)]
struct Elem {
    pos: (isize, isize),
    dir: (isize, isize),
    combo: isize,
}

#[derive(PartialEq, Eq)]
struct ElemDist {
    elem: Elem,
    dist: isize,
}

impl Ord for ElemDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for ElemDist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub enum Part {
    Part1,
    Part2,
}

pub fn solve(input: &PuzzleInput, part: Part) -> usize {
    let mut visited: HashMap<Elem, isize> = HashMap::new();
    let mut priority = BinaryHeap::new();

    let (min_combo, max_combo) = match part {
        Part::Part1 => (0, 3),
        Part::Part2 => (4, 10),
    };

    let start = ElemDist {
        dist: 0,
        elem: Elem {
            pos: (0, 0),
            dir: (0, 0), // (make sure 'backwards' is always false initially)
            combo: 0,    // (first move was free)
        },
    };

    let w = input[0].len() as isize;
    let h = input.len() as isize;

    priority.push(start);

    while let Some(elem_dist) = priority.pop() {
        let dist = elem_dist.dist;
        let elem = elem_dist.elem;

        // Check that we're not moving in the same dir for too long.
        if elem.combo > max_combo {
            continue;
        }

        if visited.contains_key(&elem) {
            if elem.pos.0 != 0 && elem.pos.1 != 0 {
                assert!(&dist >= visited.get(&elem).unwrap());
            }
            continue;
        }

        visited.insert(elem, dist);

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            // Check that we're not moving in reverse.
            if -dir.0 == elem.dir.0 && -dir.1 == elem.dir.1 {
                continue;
            }

            // We're only allowed to turn if we've reached the min combo.
            // (except for the start)
            if elem.combo < min_combo && dir != elem.dir && elem.dir != (0, 0) {
                continue;
            }

            // Check that the now pos will be in bounds.
            let new_pos = (elem.pos.0 + dir.0, elem.pos.1 + dir.1);
            if new_pos.0 < 0 || new_pos.0 >= w || new_pos.1 < 0 || new_pos.1 >= h {
                continue;
            }

            // We can not end with a combo thats less than the min.
            if new_pos == (w - 1, h - 1) && elem.combo < min_combo {
                continue;
            }

            let cost = input[new_pos.1 as usize][new_pos.0 as usize];

            let new_elem = Elem {
                pos: new_pos,
                dir,
                combo: match dir == elem.dir {
                    true => elem.combo + 1,
                    false => 1,
                },
            };

            if visited.contains_key(&new_elem) {
                continue;
            }

            let new_dist = dist + cost as isize;
            priority.push(ElemDist {
                elem: new_elem,
                dist: new_dist,
            });
        }
    }

    visited
        .iter()
        .filter_map(|(&elem, &val)| {
            match elem.pos.0 == w - 1 && elem.pos.1 == h - 1 && elem.combo >= min_combo {
                true => Some(val),
                false => None,
            }
        })
        .min()
        .unwrap() as usize
}

pub fn part1(input: &PuzzleInput) -> usize {
    solve(input, Part::Part1)
}

pub fn part2(input: &PuzzleInput) -> usize {
    solve(input, Part::Part2)
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
