use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Default)]
pub struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

impl Mapping {
    fn end(&self) -> usize {
        self.source + self.length
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuzzleInput {
    seeds: Vec<usize>,
    mappings: Vec<Vec<Mapping>>,
}

impl PuzzleInput {
    fn from_vec(input: &Vec<String>) -> Option<Self> {
        let seeds: Vec<usize> = input
            .get(0)?
            .split_once("seeds: ")?
            .1
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut i = 3;
        let mut mappings = Vec::new();
        let mut ranges = Vec::new();
        while i < input.len() {
            let line = input.get(i)?;

            if line.is_empty() {
                i += 2;
                mappings.push(ranges.clone());
                ranges.clear();
                continue;
            }

            let mut splitter = line.split_whitespace();
            let dest: usize = splitter.next()?.parse().ok()?;
            let source: usize = splitter.next()?.parse().ok()?;
            let length: usize = splitter.next()?.parse().ok()?;

            ranges.push(Mapping {
                source,
                dest,
                length,
            });

            i += 1;
        }

        // Add the last range vec
        mappings.push(ranges);

        Some(PuzzleInput { seeds, mappings })
    }
}

pub fn part1(input: &PuzzleInput) -> usize {
    let mut lowest = usize::MAX;
    for seed in &input.seeds {
        let mut translated = *seed;
        for stage in &input.mappings {
            for mapping in stage {
                if translated > mapping.source && translated < mapping.end() {
                    let new = mapping.dest + (translated - mapping.source);
                    translated = new;
                    break;
                }
            }
        }
        if translated < lowest {
            lowest = translated;
        }
    }
    lowest
}

#[derive(Debug, Clone, Default)]
struct Range {
    start: usize,
    length: usize,
}

impl Range {
    fn overlaps(&self, range: &Mapping) -> bool {
        range.source < self.start + self.length && self.start < range.source + range.length
    }

    fn end(&self) -> usize {
        self.start + self.length
    }

    fn overlap(&self, range: &Mapping) -> Option<(Option<Range>, Range, Option<Range>)> {
        // Fast path. If this doesn't overlap at all we can just return None
        if !self.overlaps(range) {
            return None;
        }

        let start = usize::max(self.start, range.source);
        let end = usize::min(self.start + self.length, range.end());
        let overlap = Range {
            start,
            length: end - start,
        };

        let before = match self.start < range.source {
            true => Some(Range {
                start: self.start,
                length: range.source - self.start,
            }),
            false => None,
        };

        let after = match self.end() > range.end() {
            true => Some(Range {
                start: range.end(),
                length: self.end() - range.end(),
            }),
            false => None,
        };

        Some((before, overlap, after))
    }

    fn translate(&self, range: &Mapping) -> Self {
        Self {
            start: (self.start - range.source) + range.dest,
            length: self.length,
        }
    }

    fn split_translate(&self, mappings: &Vec<Mapping>) -> Vec<Self> {
        let mut translated = Vec::new();
        let mut todo = Vec::new();
        todo.push(self.clone());

        'outer: while !todo.is_empty() {
            let x = todo.pop().unwrap();
            for mapping in mappings {
                // Does this mapping apply?
                if let Some(parts) = x.overlap(mapping) {
                    translated.push(parts.1.translate(mapping));
                    if let Some(before) = parts.0 {
                        todo.push(before);
                    }
                    if let Some(after) = parts.2 {
                        todo.push(after);
                    }
                    continue 'outer;
                }
            }
            // We didn't manage to translate x, so apply identity to the whole range
            translated.push(x);
        }

        translated
    }
}

pub fn part2(input: &PuzzleInput) -> usize {
    let seed_ranges: Vec<Range> = input
        .seeds
        .chunks(2)
        .map(|x| Range {
            start: x[0],
            length: x[1],
        })
        .collect();

    let mut translated_ranges = seed_ranges;
    for mapping in &input.mappings {
        translated_ranges = translated_ranges
            .iter()
            .flat_map(|range| range.split_translate(mapping))
            .collect();
    }

    translated_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    PuzzleInput::from_vec(&reader.lines().flatten().collect()).expect("failed to parse")
}
