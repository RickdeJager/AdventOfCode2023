use dayXX::{part1, part2, read_input};

fn main() {
    // If there is an argument, use it as the input file name. Otherwise, use "input.txt"
    let input_file_name = std::env::args()
        .nth(1)
        .unwrap_or("input.txt".to_string());

    let input = read_input(&input_file_name);
    let res = part1(&input);
    println!("Part 1: {}", res);

    let res = part2(&input);
    println!("Part 2: {}", res);
}
