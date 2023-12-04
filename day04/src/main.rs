use day04::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (135.94 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 18519 (14.132 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 11787590 (14.360 µs)
}
