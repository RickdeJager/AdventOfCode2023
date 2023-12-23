use day17::{part1, part2, read_input};

fn main() {
    // If there is an argument, use it as the input file name. Otherwise, use "input.txt"
    let input_file_name = std::env::args()
        .nth(1)
        .unwrap_or("input.txt".to_string());
    let input = read_input(&input_file_name); // (29.361 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 785 (87.299 ms)

    let res = part2(&input);
    println!("Part 2: {}", res); // 922 (238.00 ms)
}
