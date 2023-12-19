use day13::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (70.863 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 35232 (35.640 µs)
    let res = part2(&input);
    println!("Part 2: {}", res); // 37982 (38.066 µs)
}
