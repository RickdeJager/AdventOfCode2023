use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Elem {
    id: Option<u32>,
    symbol: Option<char>,
    value: Option<u32>,
}

#[derive(Debug)]
pub struct Schematic {
    w: usize,
    h: usize,
    data: Vec<Vec<Option<Elem>>>,
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
                        row.push(None);
                    }
                    b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                        let mut num = 1;
                        let mut val = (line[idx] - b'0') as u32;
                        while line.get(idx + 1).is_some_and(|x| x.is_ascii_digit()) {
                            idx += 1;
                            val = val * 10 + (line[idx] - b'0') as u32;
                            num += 1;
                        }
                        for _ in 0..num {
                            row.push(Some(Elem {
                                id: Some(cur_id),
                                symbol: None,
                                value: Some(val),
                            }));
                        }
                        cur_id += 1;
                    }
                    x => row.push(Some(Elem {
                        id: None,
                        symbol: Some(x as char),
                        value: None,
                    })),
                }
                idx += 1;
            }
            data.push(row);
        }
        Some(Self { w, h, data })
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&Elem> {
        let x = x as isize;
        let y = y as isize;
        let mut res = Vec::new();
        let mut seen = Vec::new();

        for dx in (x - 1)..=(x + 1) {
            for dy in (y - 1)..=(y + 1) {
                if dx == x && dy == y {
                    continue;
                }
                if let Some(Some(elem)) = self
                    .data
                    .get(dy as usize)
                    .and_then(|row| row.get(dx as usize))
                {
                    // Dedupe numbers to make part2 a little easier
                    if let Some(id) = elem.id {
                        if seen.contains(&id) {
                            continue;
                        }
                        seen.push(id);
                    }
                    res.push(elem);
                }
            }
        }
        res
    }
}

pub fn part1(input: &Schematic) -> usize {
    let mut res = 0usize;
    // Since we're looping in the same order as we assigned the IDs in, we
    // can keep track of dupes w/ a single number rather than a hash set (2x perf increase)
    let mut cur_id = 0;
    for y in 0..input.h {
        for x in 0..input.w {
            if let Some(elem) = &input.data[y][x] {
                if let Some(id) = elem.id {
                    if cur_id < id
                        && input
                            .get_neighbors(x, y)
                            .iter()
                            .any(|elem| elem.symbol.is_some())
                    {
                        let val = elem.value.unwrap() as usize;
                        res += val;
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
    for y in 0..input.h {
        for x in 0..input.w {
            if let Some(elem) = &input.data[y][x] {
                if let Some('*') = elem.symbol {
                    let number_neighbors = input
                        .get_neighbors(x, y)
                        .iter()
                        .filter_map(|elem| elem.value)
                        .collect::<Vec<u32>>();
                    if number_neighbors.len() == 2 {
                        res += number_neighbors[0] * number_neighbors[1]
                    }
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
