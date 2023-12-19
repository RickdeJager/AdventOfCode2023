use std::fs::File;
use std::io::{BufRead, BufReader};

type PuzzleInput = String;

#[derive(Default)]
struct Hasher {
    cur: u8,
}

impl Hasher {
    fn update(&mut self, byte: u8) {
        self.cur = (self.cur+byte) * 17
    }
    fn reset(&mut self) {
        self.cur = 0;
    }
    fn hash_string(&mut self, s: &str) -> u8 {
        self.reset();
        for c in s.chars() {
            self.update(c as u8);
        }
        self.cur
    }
}

#[derive(Default, Clone, Debug)]
struct HashElem {
    key: String,
    value: usize,
}
struct HashMapButFestive {
    hasher: Hasher,
    buckets: Vec<Vec<HashElem>>,
}

impl HashMapButFestive {
    fn new() -> Self {
        Self {
            hasher: Hasher::default(),
            buckets: vec![Vec::new(); 256],
        }
    }

    fn insert(&mut self, key: String, value: usize) {
        let hash = self.hasher.hash_string(&key);

        // find the correct bucket
        let bucket = self.buckets.get_mut(hash as usize).unwrap();

        // update?
        match bucket.iter_mut().find(|elem| elem.key == key) {
            Some(elem) => {
                elem.value = value
            },
            None => {
                // take ownership of the key string here
                let h = HashElem {
                    key,
                    value
                };
                bucket.push(h)
            }
        }
    }

    fn remove(&mut self, key: &str) {
        let hash = self.hasher.hash_string(&key);

        // find the correct bucket
        let bucket = self.buckets.get_mut(hash as usize).unwrap();

        // update?
        match bucket.iter().position(|elem| &elem.key == key) {
            Some(index) => {
                bucket.remove(index);
            },
            None => { /* no-op */}
        }
    }

    fn power(&self) -> usize {
        self.buckets.iter().enumerate().map(|(box_idx, bucket)| -> usize {
            bucket.iter().enumerate().map(|(slot, elem)| {
                elem.value * (slot + 1)
            }).sum::<usize>() * (box_idx + 1)
        }).sum::<usize>()
    }
    
}

pub fn part1(input: &PuzzleInput) -> usize {
    let mut hasher = Hasher::default();
    let mut res = 0;
    for block in input.split(",") {
        for c in block.chars() {
            hasher.update(c as u8);
        }
        res += hasher.cur as usize;
        hasher.reset();
    }
    res
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut hashmap = HashMapButFestive::new();
    for block in input.split(",") {
        let tok = match block.contains("=") {
            true => '=',
            false => '-',
        };

        let (key, value) = block.split_once(tok).unwrap();
        match tok {
            '-' => hashmap.remove(key),
            '=' => {
                let value: usize = value.parse().unwrap();
                hashmap.insert(key.to_string(), value)
            }
            _ => unreachable!()
        }
    }
    hashmap.power()
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    
    reader.lines()
        .flatten()
        .collect()
}
