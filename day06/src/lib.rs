use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_intersections(m: isize, d: isize) -> (f64, f64) {
    // -X^2 + mx - d
    let a = -1f64;
    let b = m as f64;
    let c = -d as f64;

    let x1 = (-b + f64::sqrt(b.powf(2f64) - 4f64 * a * c)) / 2f64 * a;
    let x2 = (-b - f64::sqrt(b.powf(2f64) - 4f64 * a * c)) / 2f64 * a;

    (x1, x2)
}

fn get_ways(t: isize, d: isize) -> isize {
    let (x1, x2) = get_intersections(t, d);
    let x1 = match x1.fract() == 0.0 {
        true => x1 as isize + 1,
        false => x1.ceil() as isize,
    };
    let x2 = match x2.fract() == 0.0 {
        true => x2 as isize - 1,
        false => x2.floor() as isize,
    };
    x2 - x1 + 1
}

pub fn part1(input: &[String]) -> isize {
    let times: Vec<isize> = input[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<isize> = input[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut ret = 1;

    for i in 0..times.len() {
        let t = times[i];
        let d = distances[i];
        ret *= get_ways(t, d);
    }

    ret
}

pub fn part2(input: &[String]) -> isize {
    let t: isize = input[0]
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let d: isize = input[1]
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    get_ways(t, d)
}

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader.lines().flatten().collect()
}
