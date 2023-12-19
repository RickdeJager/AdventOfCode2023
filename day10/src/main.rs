use day10::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (30.745 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 6942 (963.84 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 297 (2.4916 ms)
}
