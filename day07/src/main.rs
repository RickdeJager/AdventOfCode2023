use day07::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (43.386 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 249483956 (43.683 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 252137472 (48.028 µs)
}
