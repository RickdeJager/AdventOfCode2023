use std::fs::File;
use std::io::{BufRead, BufReader};
use memoize::memoize;

type Springs = Vec<char>;
type Nums = Vec<usize>;
type PuzzleInput = Vec<(Springs, Nums)>;

#[memoize]
fn solve(springs: Vec<char>, nums: Vec<usize>) -> usize {
    // If we've run through all of the springs, nums better be empty as well
    if springs.is_empty() {
        return nums.is_empty() as usize;
    }

    // If the charter says there are no more broken strings, we may not encounter
    // any more '#'s
    if nums.is_empty() {
        return (!springs.contains(&'#')) as usize;
    }

    let mut res = 0;

    // Case 1: the current character is a dot, or a question mark we want to fill in as a dot.
    if springs[0] == '.' || springs[0] == '?' {
        res += solve(springs[1..].to_vec(), nums.clone())
    }

    if springs[0] == '#' || springs[0] == '?' {
        // clauses:
        // 1: There are enough springs left
        // 2: This block does not contain _any_ operational springs, for the full size
        //    of the noted block
        // 3: Either:
        //   3.1: The number of broken springs fits exactly in the number of springs left
        //   3.2: There are more springs, but the one directly after this block is not known to be broken
        if (nums[0]) <= springs.len()
            && !springs[..nums[0]].contains(&'.')
            && (nums[0] == springs.len() || springs[nums[0]] != '#')
        {
            if nums[0] == springs.len() {
                res += solve(Vec::new(), nums[1..].to_vec())
            } else {
                res += solve(springs[nums[0] + 1..].to_vec(), nums[1..].to_vec())
            }
        }
    }

    res
}

pub fn part1(input: &PuzzleInput) -> usize {
    input
        .iter()
        .map(|(springs, nums)| solve(springs.clone(), nums.clone()))
        .sum()
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut input = input.clone();
    input
        .iter_mut()
        .map(|(springs, nums)| {
            let spring_len = springs.len();
            let nums_len = nums.len();
            springs.push('?');
            let springs: Vec<char> = springs
                .iter()
                .cycle()
                .take(spring_len * 5 + 4)
                .copied()
                .collect();
            let nums: Vec<usize> = nums.iter().cycle().take(nums_len * 5).copied().collect();

            solve(springs, nums)
        })
        .sum()
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .map(|line| {
            let (springs, nums) = line.split_once(' ').unwrap();
            let springs = springs.chars().collect();
            let nums = nums.split(',').map(|x| x.parse().unwrap()).collect();
            (springs, nums)
        })
        .collect()
}
