use day14::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (18.370 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 113486 (11.696 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 104409 (13.125 ms)
}
