use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Elem {
    Empty,
    Symbol(char),
    /// Part Number (id, value)
    Number(u32, u32),
}

#[derive(Debug)]
pub struct Schematic {
    w: usize,
    h: usize,
    data: Vec<Vec<Elem>>,
}

impl Schematic {
    fn from_vec(s: &Vec<String>) -> Option<Self> {
        let w = s[0].len();
        let h = s.len();

        let mut cur_id = 1;

        let mut data = Vec::new();
        for line in s {
            let mut row = Vec::new();
            let mut idx = 0usize;
            let line = line.as_bytes();
            while idx < w {
                match line[idx] {
                    b'.' => {
                        row.push(Elem::Empty);
                    }
                    b'0'..=b'9' => {
                        let mut num = 1;
                        let mut val = (line[idx] - b'0') as u32;
                        while line.get(idx + 1).is_some_and(|x| x.is_ascii_digit()) {
                            idx += 1;
                            val = val * 10 + (line[idx] - b'0') as u32;
                            num += 1;
                        }
                        for _ in 0..num {
                            row.push(Elem::Number(cur_id, val));
                        }
                        cur_id += 1;
                    }
                    x => row.push(Elem::Symbol(x as char)),
                }
                idx += 1;
            }
            data.push(row);
        }
        Some(Self { w, h, data })
    }

    fn get_neighbors<'a>(&'a self, neighbors: &mut Vec<&'a Elem>, x: usize, y: usize) {
        let x = x as isize;
        let y = y as isize;
        let mut seen = [0u32; 9];

        // remove any stale entries from neighbors
        neighbors.clear();

        for dx in (x - 1)..=(x + 1) {
            for dy in (y - 1)..=(y + 1) {
                if dx == x && dy == y {
                    continue;
                }
                if let Some(elem) = self
                    .data
                    .get(dy as usize)
                    .and_then(|row| row.get(dx as usize))
                {
                    if let Elem::Number(id, _val) = elem {
                        // Dedupe numbers to make part2 a little easier
                        if seen.contains(id) {
                            continue;
                        }
                        seen[((x - dx + 1) * 3 + (y - dy + 1)) as usize] = *id;
                    }
                    neighbors.push(elem);
                }
            }
        }
    }
}

pub fn part1(input: &Schematic) -> usize {
    let mut res = 0usize;
    // Since we're looping in the same order as we assigned the IDs in, we
    // can keep track of dupes w/ a single number rather than a hash set (2x perf increase)
    let mut cur_id = 0;
    // A reusable allocation to store neighbors
    let mut neighbors = Vec::<&Elem>::new();
    for y in 0..input.h {
        for x in 0..input.w {
            if let Elem::Number(id, value) = input.data[y][x] {
                if cur_id < id {
                    // update the neighbors vec
                    input.get_neighbors(&mut neighbors, x, y);
                    if neighbors.iter().any(|elem| matches!(elem, Elem::Symbol(_))) {
                        res += value as usize;
                        cur_id = id;
                    }
                }
            }
        }
    }
    res
}

pub fn part2(input: &Schematic) -> u32 {
    let mut res = 0;
    let mut neighbors = Vec::new();
    for y in 0..input.h {
        for x in 0..input.w {
            if let Elem::Symbol('*') = &input.data[y][x] {
                input.get_neighbors(&mut neighbors, x, y);
                let number_neighbors = neighbors
                    .iter()
                    .filter_map(|elem| match elem {
                        Elem::Number(_id, val) => Some(*val),
                        _ => None,
                    })
                    .collect::<Vec<u32>>();
                if number_neighbors.len() == 2 {
                    res += number_neighbors[0] * number_neighbors[1]
                }
            }
        }
    }
    res
}

pub fn read_input(filename: &str) -> Schematic {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    Schematic::from_vec(&reader.lines().flatten().collect()).expect("failed to parse schematic")
}
