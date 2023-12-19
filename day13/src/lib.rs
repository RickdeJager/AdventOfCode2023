type Pattern = Vec<Vec<bool>>;
type PuzzleInput = Vec<Pattern>;

fn solve(input: &PuzzleInput, target_score: usize) -> usize {
    let mut ret = 0;
    'next_pattern: for pattern in input {
        // Find a vertical slice
        for x in 1..pattern[0].len() {
            let score: usize = pattern
                .iter()
                .map(|row| {
                    let before = row[..x].iter().rev();
                    let after = row[x..].iter();
                    let iter = before.zip(after);
                    iter.map(|(a, b)| (a != b) as usize).sum::<usize>()
                })
                .sum();

            if score == target_score {
                ret += x;
                continue 'next_pattern;
            }
        }

        // Find a horizontal slice
        for y in 1..pattern.len() {
            let score: usize = {
                let before = (0..y).into_iter().rev();
                let after = (y..pattern.len()).into_iter();
                let iter = before.zip(after);
                iter.map(|(a, b)| {
                    pattern[a]
                        .iter()
                        .zip(pattern[b].iter())
                        .map(|(&c, &d)| (c != d) as usize)
                        .sum::<usize>()
                })
                .sum::<usize>()
            };

            if score == target_score {
                ret += y * 100;
                continue 'next_pattern;
            }
        }
        unreachable!("Didn't find any reflections");
    }
    ret
}

pub fn part1(input: &PuzzleInput) -> usize {
    solve(input, 0)
}

pub fn part2(input: &PuzzleInput) -> usize {
    solve(input, 1)
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let data = std::fs::read_to_string(filename).expect("Failed to open file");

    data.split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}
